use std::collections::{BTreeMap, HashMap};

use log::{debug, error, info, warn};

use crate::{
    flow::{Flow, ParsedInstructionSet},
    hex::Hex,
    instruction::{Instruction, InstructionSet, JumpType},
    memory::Memory,
    opcode::OpCodes,
    parser::parse_instruction_set,
};

pub struct FlowParser<'a> {
    instructions: &'a HashMap<Hex, Instruction>,
    flows: Vec<Flow>,
}

impl FlowParser<'_> {
    pub fn new(instructions: &HashMap<Hex, Instruction>) -> FlowParser {
        FlowParser {
            instructions,
            flows: Vec::new(),
        }
    }

    // Iterate over all instruction sets and reconstruct all jumps.
    pub fn parse_flows(&mut self) {
        info!("parsing flows");
        let sorted_instructions: BTreeMap<_, _> = self.instructions.clone().into_iter().collect();
        for instruction in sorted_instructions {
            debug!("{:?}", instruction);
        }
        let memory = Memory::new();
        let steps =
            parse_instruction_set(Hex::from(0), &self.instructions, None, memory.clone(), None);
        for step in &steps {
            debug!("first step: {:x}", step);
            // Update the stack for this section
            println!("Parsing jump {:?}", step.jump);

            let flow = Flow::new(step.clone());
            let flows = self.parse_next_step(flow);
            self.flows.extend(flows);
        }
    }

    fn parse_next_step(&self, flow: Flow) -> Vec<Flow> {
        let mut result = Vec::new();
        let mut flow = flow.clone();
        if let Some(last_step) = flow.get_last_step() {
            debug!(
            "Next step starts at {:?} with jump instruction {:?}. The added stack should be {:?}",
            last_step.target, last_step.jump, last_step.stack
        );
            if let Some(ref target) = last_step.target {
                if let Some(next_step) = self.instructions.get(&target.value) {
                    warn!("Our next step starts at {0:?}", next_step.index);
                    if next_step.opcode.code != OpCodes::JUMPDEST {
                        flow.print();
                        panic!("next_step does not start with JUMPDEST");
                    }
                    println!("{:?}", next_step);
                    let targets = parse_instruction_set(
                        next_step.index.clone(),
                        &self.instructions,
                        Some(last_step.stack.clone()),
                        last_step.memory.clone(),
                        None,
                    );
                    // Now, we want to reparse the next step, so we can update its stack from the
                    // 'leftovers' from the last step.
                    // Now we need to append our flow with the new step. Clone the flow for every jump
                    // found in the new_step.
                    if targets.len() == 0 {
                        warn!(
                            "Section {} does not have any defined targets.",
                            next_step.index
                        );
                    }
                    for target in &targets {
                        warn!("Parsing next step {:x}", target);
                        flow.add_step(target.clone());
                        if flow.len() < 25 {
                            warn!("Starting new flow branch, starting from {}", target.start);
                            result.append(&mut self.parse_next_step(flow.clone()));
                        } else {
                            error!("Flow got too long...");
                        }
                    }
                } else {
                    flow.print();
                    println!("{:?}", last_step);
                    panic!("Cannot parse next step; {:?} does not exist as an individual instruction set.", target.value);
                }
            } else {
                panic!("Reached end of flow, there is no target to jump to.");
            }
        } else {
            debug!("flow ends");
        }
        result.push(flow);
        result
    }

    pub fn flows(&self) -> &Vec<Flow> {
        &self.flows
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        flow_parser::FlowParser,
        opcode::OpCodes::{
            CALLVALUE as OPCODE_CALLVALUE, DUP1, ISZERO, JUMP, JUMPDEST, JUMPI, MSTORE, PUSH1,
            PUSH2, REVERT, STOP,
        },
        parser::Parser,
    };
    use test_log::test;

    #[test]
    fn parse_simple_flow() {
        let input = Vec::from([
            PUSH1 as u32,
            0xb, //0x0,0x1
            PUSH1 as u32,
            0x9, //0x2,0x3
            PUSH1 as u32,
            0x7,             // 0x4, 0x5, jumping to second section
            JUMP as u32,     //0x6
            JUMPDEST as u32, //0x7
            JUMP as u32,     //0x8
            JUMPDEST as u32, //0x9
            JUMP as u32,     //0x1
            JUMPDEST as u32, //0xb
            STOP as u32,     //0xc
        ]);

        let parser = Parser::new(input);
        let instruction_sets = parser.get_instruction_sets();
        let instructions = parser.get_instructions();
        assert_eq!(instruction_sets.len(), 4);

        let mut flow_parser = FlowParser::new(instructions);
        flow_parser.parse_flows();
        let flows = flow_parser.flows;
        assert_eq!(flows.len(), 1);
    }

    #[test]
    fn break_into_simple_instruction_sections() {
        let input: Vec<u32> = Vec::from([
            PUSH1 as u32,
            0x3,             // 0x0, 0x1 argument for PUSH1
            JUMP as u32,     // 0x2
            JUMPDEST as u32, // 0x3
            STOP as u32,     // 0x4
        ]);

        let parser = Parser::new(input);
        let instruction_sets = parser.get_instruction_sets();
        let instructions = parser.get_instructions();
        // we have two sections; one before the jump and one after the jump. From JUMPDEST to STOP.
        assert_eq!(instruction_sets.len(), 2);
        let mut flow_parser = FlowParser::new(instructions);
        flow_parser.parse_flows();

        let flow = flow_parser.flows;
        assert_eq!(flow.len(), 1);
    }
    #[test]
    fn parse_conditional_flow() {
        let input = Vec::from([
            PUSH1 as u32,
            0xe, //0x0,0x1
            PUSH1 as u32,
            0xc, //0x2,0x3
            PUSH1 as u32,
            0x1, // 0x4, 0x5
            PUSH1 as u32,
            0xa,             // 0x6,0x7
            JUMPI as u32,    // 0x8
            JUMP as u32,     //0x9
            JUMPDEST as u32, //0xa
            JUMP as u32,     //0xb
            JUMPDEST as u32, //0xc
            JUMP as u32,     //0xd
            JUMPDEST as u32, //0xe
            STOP as u32,     //0xf
        ]);
        let parser = Parser::new(input);
        let instructions = parser.get_instructions();
        let instruction_sets = parser.get_instruction_sets();
        assert_eq!(instruction_sets.len(), 4, "{:?}", instruction_sets);
        let mut flow_parser = FlowParser::new(instructions);
        flow_parser.parse_flows();
        let flows = flow_parser.flows;
        println!("flows: {:?}", flows);
        for flow in &flows {
            println!("{:?}", flow.print());
        }
        assert_eq!(flows.len(), 2);
    }
    #[test]
    fn real_flow() {
        let input = Vec::from([
            PUSH1 as u32,
            0x40, //0x0,0x1
            PUSH1 as u32,
            0x60, //0x2,0x3
            MSTORE as u32,
            OPCODE_CALLVALUE as u32,
            DUP1 as u32,
            ISZERO as u32,
            PUSH2 as u32,
            00 as u32,
            10 as u32,
            JUMPI as u32,
            PUSH1 as u32,
            00 as u32,
            DUP1 as u32,
            REVERT as u32,
            JUMPDEST as u32,
            REVERT as u32,
        ]);

        let parser = Parser::new(input);
        let instruction_sets = parser.get_instruction_sets();
        let instructions = parser.get_instructions();
        assert_eq!(instruction_sets.len(), 2);

        let mut flow_parser = FlowParser::new(instructions);
        flow_parser.parse_flows();
        let flows = flow_parser.flows;
        assert_eq!(flows.len(), 2);
    }
}
