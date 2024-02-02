use std::collections::HashMap;

use crate::{
    flow::{Flow, ParsedInstructionSet},
    instruction::{Hex, Instruction, InstructionSet, JumpInstruction, JumpType, ParsedInstruction},
    parser::parse_instruction_set,
};

pub struct FlowParser<'a> {
    instruction_sets: HashMap<Hex, InstructionSet>,
    instructions: &'a HashMap<Hex, Instruction>,

    flows: Vec<Flow>,
}

impl FlowParser<'_> {
    pub fn new(
        instruction_sets: HashMap<Hex, InstructionSet>,
        instructions: &HashMap<Hex, Instruction>,
    ) -> FlowParser {
        FlowParser {
            instruction_sets,
            instructions,
            flows: Vec::new(),
        }
    }

    // Iterate over all instruction sets and reconstruct all jumps.
    pub fn parse_flows(&mut self) {
        let new_step = self.instruction_sets.get(&(Hex(0))).unwrap().clone();
        for jump in new_step.jumps {
            // Update the stack for this section
            let instruction = parse_instruction_set(
                new_step.start,
                &self.instructions,
                None,
                Some(jump.instruction.index),
            )
            .unwrap();
            println!("instruction Hex(0): {:?}", instruction);
            println!("Parsing jump {:?}", jump);
            let parsed_instruction_set = ParsedInstructionSet {
                start: instruction.start,
                end: instruction.end,
                target: jump.target,
                jump: Some(jump.clone()),
                stack: instruction.stack.clone(),
            };
            let flow = Flow::new(parsed_instruction_set);
            let flows = self.parse_next_step(flow);
            self.flows.extend(flows);
        }
        println!("result: {:?}", self.flows);
    }

    fn parse_next_step(&self, flow: Flow) -> Vec<Flow> {
        let mut result = Vec::new();
        let mut flow = flow.clone();
        println!("parse_jumps, flow={:?}", flow);
        while let Some(last_step) = flow.get_last_step() {
            println!(
            "Next step starts at {:?} with jump instruction {:?}. The added stack should be {:?}",
            last_step.target, last_step.jump, last_step.stack
        );
            if let Some(target) = last_step.target {
                if let Some(next_step) = self.instruction_sets.get(&target) {
                    println!("Our next step starts at {0:?}", next_step.start);
                    println!("Our old next step stack is {0:?}", next_step.stack);
                    println!("{:?}", flow);
                    // Now, we want to reparse the next step, so we can update its stack from the
                    // 'leftovers' from the last step.
                    // Now we need to append our flow with the new step. Clone the flow for every jump
                    // found in the new_step.
                    if next_step.jumps.len() == 0 {
                        break;
                    }
                    for jump in &next_step.jumps {
                        println!(
                            "Parsing next step, starting from {:?}, ending at {:?}",
                            next_step.start, jump.instruction.index
                        );
                        let next_step = parse_instruction_set(
                            next_step.start,
                            &self.instructions,
                            Some(last_step.stack.clone()),
                            Some(jump.instruction.index),
                        )
                        .unwrap();
                        println!("Our new next step stack is {0:?}", next_step.stack);
                        println!("next step.jumps = {:?}", next_step.jumps);
                        println!("Current jump = {:?}", jump);
                        let next_step_jump = next_step
                            .jumps
                            .iter()
                            .find(|&j| j.instruction.index == jump.instruction.index)
                            .unwrap();
                        let new_parsed_instruction_set = ParsedInstructionSet {
                            start: next_step.start,
                            end: next_step.end,
                            target: next_step_jump.target,
                            jump: Some(next_step_jump.clone()),
                            stack: next_step.stack.clone(),
                        };
                        flow.add_step(new_parsed_instruction_set);
                        if jump.jump_type == JumpType::Conditional {
                            println!("Starting another flow because of a conditional step");
                            result.extend(self.parse_next_step(flow.clone()));
                        }
                    }
                } else {
                    println!("Cannot parse next step; target does not exist as an individual instruction set.");
                    break;
                }
            } else {
                println!("Reached end of flow, there is no target to jump to.");
                break;
            }
        }
        flow.print();
        result.push(flow);
        println!("parse_next_step result: {:?}", self.flows);
        result
    }

    //fn parse_jump(
    //    &self,
    //    new_step: &InstructionSet,
    //    flow: &mut Flow,
    //    jump: &JumpInstruction,
    //    previous_stack: Vec<Hex>,
    //) {
    //    // Update the stack for this section
    //    let instruction =
    //        parse_instruction_set(new_step.start, &self.instructions, Some(previous_stack))
    //            .unwrap();
    //    let parsed_instruction_set = ParsedInstructionSet {
    //        start: instruction.start,
    //        end: instruction.end,
    //        target: jump.target,
    //        jump: Some(jump.clone()),
    //        stack: instruction.stack,
    //    };
    //    flow.add_step(parsed_instruction_set);
    //}
}
#[cfg(test)]
mod tests {
    use crate::{
        flow_parser::FlowParser,
        opcode::OpCodes::{JUMP, JUMPDEST, JUMPI, PUSH1, STOP},
        parser::Parser,
    };
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

        let mut flow_parser = FlowParser::new(instruction_sets, instructions);
        flow_parser.parse_flows();
        let flows = flow_parser.flows;
        assert_eq!(flows.len(), 1);
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
        let instruction_sets = parser.get_instruction_sets();
        let instructions = parser.get_instructions();
        assert_eq!(instruction_sets.len(), 4);
        let mut flow_parser = FlowParser::new(instruction_sets, instructions);
        flow_parser.parse_flows();
        let flows = flow_parser.flows;
        println!("flows: {:?}", flows);
        for flow in &flows {
            println!("{:?}", flow.print());
        }
        assert_eq!(flows.len(), 2);
    }
}
