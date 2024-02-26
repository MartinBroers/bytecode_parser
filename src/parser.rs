use log::{debug, info, warn};
use std::collections::HashMap; // Use log crate when building application

use crate::{
    flow::ParsedInstructionSet,
    hex::Hex,
    instruction::{Instruction, InstructionSet, JumpInstruction, JumpType},
    memory::Memory,
    opcode::{self, opcodes},
    stack::{self, Stack},
    utils::find_sequence,
};

pub struct Parser {
    _bytecode: Vec<u32>,
    _cbor_part: Vec<u32>,
    instructions: HashMap<Hex, Instruction>,
    instruction_sets: HashMap<Hex, InstructionSet>,
}

// First, we have bytecode. Bytecode is a continuous array of hexidecimal integers. In order to
// be able to parse it, we need to parse it in a few separate steps:
//
// 1. Convert the continuous array to 'instructions'. An instruction is a data object which, as
//    the first argument, has the opcode and following its arguments required to function. For
//    example, '60 40' is an instruction, since the opcode is PUSH1 and there is 1 argument.
// 2. Now that we have instructions, we can group them into instruction sets. These sets start
//    where ever the previous set stops and continuous until it ends itself, for example at a
//    'JUMP' instruction, 'STOP' or those functions.
// 3. Now that we have an array of instruction sets, we can look if we can figure out which
//    sets jump to which sets, linking them together.
// 4. Also, what we need to do is to find all conditional statements, since they _really_
//    control the flow in the application. If we can identify them, we may also identify their
//    required inputs by walking the control flow backwards through the application and write
//    automated tests for a contract.
// 5. If we then have all links from step 3, we can do whatever we want in terms of sequences
//    in the contract; we know now which PUSH instructions push the JUMP addressses for all
//    jumps in the contract.
// 6. We can also inject instructions in instruction sets since we know which PUSH values we
//    need to update to which JUMPDESTS.
//

impl Parser {
    pub fn new(raw_bytecode: Vec<u32>) -> Parser {
        //a2 64 69 70 66 73 // i p f s
        //TODO Add other cbor's as well.
        let cbor_sequence: Vec<u32> = vec![0xa2, 0x64, 0x69, 0x70, 0x66, 0x73];
        let cbor_start = find_sequence(&raw_bytecode, &cbor_sequence);
        let mut cbor_part = vec![];
        let input: Vec<u32> = if let Some(cbor_loc) = cbor_start {
            let splice = raw_bytecode[0..cbor_loc].to_vec();
            cbor_part = raw_bytecode[cbor_loc..].to_vec();
            splice
        } else {
            raw_bytecode.to_vec()
        };
        let instructions = bytecode_to_instructions(input.to_vec());
        let instruction_sets = parse_instruction_sets(&instructions);
        Parser {
            instructions,
            instruction_sets,
            _bytecode: input,
            _cbor_part: cbor_part,
        }
    }

    pub fn get_instruction_sets(&self) -> HashMap<Hex, InstructionSet> {
        self.instruction_sets.clone()
    }

    pub fn get_instructions(&self) -> &HashMap<Hex, Instruction> {
        &self.instructions
    }
}

fn parse_instruction_sets(
    instructions: &HashMap<Hex, Instruction>,
) -> HashMap<Hex, InstructionSet> {
    let mut instruction_sets: HashMap<Hex, InstructionSet> = HashMap::new();

    let mut stack_pointer: Hex = 0x0.into();
    while let Some(instruction_set) = create_instruction_set(stack_pointer.clone(), &instructions) {
        info!("instruction_set: {:?}", instruction_set);
        instruction_sets.insert(stack_pointer, instruction_set.clone());
        stack_pointer = instruction_set.end + Hex(1);
    }
    instruction_sets
}

fn create_instruction_set(
    stack_pointer: Hex,
    instructions: &HashMap<Hex, Instruction>,
) -> Option<InstructionSet> {
    let mut instructions_section: InstructionSet = InstructionSet {
        start: stack_pointer.clone(),
        end: stack_pointer.clone(),
        jump: None,
        stack: Stack::new(),
    };
    let stack_pointer_in = stack_pointer.clone();
    let mut stack_pointer = stack_pointer;
    while let Some(instruction) = instructions.get(&stack_pointer) {
        info!("parsing {:?}: {:?}", stack_pointer, instruction);
        match instruction.opcode.code {
            //opcode::OpCodes::JUMPI => instructions_section.jumps.push(JumpInstruction {
            //    instruction: instruction.clone(),
            //    jump_type: JumpType::Conditional,
            //    target: None,
            //    source: instruction.index,
            //    condition: None,
            //}),
            opcode::OpCodes::JUMP => {
                //instructions_section.jump = Some(JumpInstruction {
                //    instruction: instruction.clone(),
                //    jump_type: JumpType::Unconditional,
                //    target: None,
                //    source: instruction.index,
                //    condition: None,
                //});
                break;
            }
            opcode::OpCodes::RETURN
            | opcode::OpCodes::STOP
            | opcode::OpCodes::REVERT
            | opcode::OpCodes::INVALID
            | opcode::OpCodes::SELFDESTRUCT => {
                break;
            }
            _ => {
                stack_pointer += Hex(instruction.opcode.input_arguments as u128);
            }
        }
        stack_pointer += 1.into();
    }
    if stack_pointer != stack_pointer_in {
        instructions_section.stack = Stack::new();
        instructions_section.end = stack_pointer;
        Some(instructions_section)
    } else {
        None
    }
}

pub fn parse_instruction_set(
    stack_pointer: Hex,
    instructions: &HashMap<Hex, Instruction>,
    input_stack: Option<Stack>,
    memory: Memory,
    end_at: Option<Hex>,
) -> Vec<ParsedInstructionSet> {
    let mut instruction_sections: Vec<ParsedInstructionSet> = vec![];
    let start_stack_pointer = stack_pointer.clone();
    let mut stack: Stack = input_stack.unwrap_or(Stack::new());
    let mut memory = memory.clone();
    let mut stack_pointer = stack_pointer.clone();
    while let Some(instruction) = instructions.get(&stack_pointer) {
        if let Some(ref end_at) = end_at {
            if stack_pointer > *end_at {
                debug!("Stopping parse of instruction set, because we reached the end.");
                break;
            }
        }
        info!(
            "parsing {:?} {:?},\t stack: {:?}",
            stack_pointer, instruction, stack
        );
        let instruction = instruction.clone();
        let result = instruction.parse(&mut stack, &mut stack_pointer, &mut memory);
        if let Ok(opcode_result) = result {
            match opcode_result {
                opcode::OpCodeResult::ConditionalJumpInstruction(mut ji) => {
                    ji.source = start_stack_pointer.clone();
                    let instructions_section: ParsedInstructionSet = ParsedInstructionSet {
                        start: start_stack_pointer.clone(),
                        end: stack_pointer.clone(),
                        jump: Some(ji.clone()),
                        stack: stack.clone(),
                        memory: memory.clone(),
                        target: Some(ji.target.clone()),
                    };
                    instruction_sections.push(instructions_section.clone());
                }
                opcode::OpCodeResult::JumpInstruction(mut ji) => {
                    ji.source = start_stack_pointer.clone();
                    let instructions_section: ParsedInstructionSet = ParsedInstructionSet {
                        start: start_stack_pointer,
                        end: stack_pointer,
                        jump: Some(ji.clone()),
                        stack: stack.clone(),
                        memory: memory.clone(),
                        target: Some(ji.target.clone()),
                    };
                    instruction_sections.push(instructions_section.clone());
                    break;
                }
                opcode::OpCodeResult::End => {
                    let instructions_section: ParsedInstructionSet = ParsedInstructionSet {
                        start: start_stack_pointer,
                        end: stack_pointer,
                        jump: None,
                        stack: stack.clone(),
                        memory: memory.clone(),
                        target: None,
                    };
                    instruction_sections.push(instructions_section);
                    break;
                }
                opcode::OpCodeResult::Ok => (),
            }
        } else {
            panic!("Could not parse instruction: {:?}", instruction);
        }
        stack_pointer += 1.into();
    }
    instruction_sections
}
fn bytecode_to_instructions(raw_bytecode: Vec<u32>) -> HashMap<Hex, Instruction> {
    let mut instructions: HashMap<Hex, Instruction> = HashMap::new();
    let opcodes = opcodes();

    //let mut iterator = self.raw_bytecode.iter();
    let mut iterator = raw_bytecode.iter().enumerate();
    while let Some((index, instruction)) = iterator.next() {
        // Check if instruction is a valid opcode
        if let Some(opcode) =
            opcodes.get(&num_traits::FromPrimitive::from_u32(*instruction).unwrap())
        {
            let input_args = opcode.input_arguments;
            let mut args = Vec::new();
            if input_args > 0 {
                for _ in 0..input_args {
                    let (_, arg) = iterator.next().unwrap().to_owned();
                    args.push((*arg).into());
                }
            }

            let instruction = Instruction {
                args,
                opcode: opcode.clone(),
                index: index.into(),
            };
            instructions.insert(index.into(), instruction);
        } else {
            panic!("Found unknown instruction: {:x}, aborting", instruction);
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{
        hex::Hex,
        memory::Memory,
        opcode::OpCodes::{ADD, JUMP, JUMPDEST, JUMPI, POP, PUSH1, STOP},
        parser::parse_instruction_set,
        stack::StackElement,
    };

    use super::{bytecode_to_instructions, Parser};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn simple_push() {
        init();
        let result = bytecode_to_instructions([0x60, 0x80].to_vec());
        assert_eq!(result.keys().len(), 1);
        let instruction = result.get(&Hex(0x0)).unwrap();
        assert_eq!(instruction.args.len(), 1);
        assert_eq!(*instruction.args.first().unwrap(), Hex(0x80));
        assert_eq!(instruction.opcode.code, PUSH1);
        assert_eq!(instruction.index, Hex(0x0));
    }
    #[test]
    #[should_panic]
    fn simple_push_no_arg() {
        init();
        let _ = bytecode_to_instructions([0x60].to_vec());
    }
    #[test]
    fn instructions_between_jump_target_push_and_jump_instruction() {
        let input = Vec::from([
            PUSH1 as u32,
            0x9, // 0x0, 0x1 argument for PUSH1
            PUSH1 as u32,
            0x43, // 0x2 push arguments for ADD command
            PUSH1 as u32,
            0x48,            // 0x4
            ADD as u32,      // 0x6
            POP as u32,      // 0x7
            JUMP as u32,     // 0x8
            JUMPDEST as u32, // 0x9
            STOP as u32,     // 0xa
        ]);

        let parser = Parser::new(input);
        let instruction_sections = parser.get_instruction_sets();
        assert_eq!(instruction_sections.len(), 2);
        let mut memory = Memory::new();
        let first_instruction_section = parse_instruction_set(
            Hex(0),
            parser.get_instructions(),
            None,
            Some(instruction_sections.get(&Hex(0)).unwrap().end),
            &mut memory,
        )
        .unwrap();
        let target_instruction_section = parse_instruction_set(
            Hex(9),
            parser.get_instructions(),
            None,
            Some(instruction_sections.get(&Hex(9)).unwrap().end),
            &mut memory,
        )
        .unwrap();
        let jump_target = &first_instruction_section.jumps.first().unwrap().target;
        let jump_dest = target_instruction_section.start;
        assert_eq!(
            jump_target,
            &Some(StackElement {
                value: jump_dest,
                origin: Hex(0),
                size: 1,
            })
        );
    }

    #[test]
    fn find_related_jumps() {
        let input = Vec::from([
            PUSH1 as u32,
            0x3,             // jumping to second section
            JUMP as u32,     //0x2
            JUMPDEST as u32, //0x3
            PUSH1 as u32,
            0x7,             //0x4,0x5
            JUMP as u32,     //0x6
            JUMPDEST as u32, //0x7
            STOP as u32,     //0x8
        ]);

        let parser = Parser::new(input);
        let instruction_sections = parser.get_instruction_sets();

        assert_eq!(
            instruction_sections.len(),
            3,
            "actual instruction section: {:?}",
            instruction_sections
        );

        let jumps = {
            let mut result = Vec::new();
            for (_, instruction_set) in &instruction_sections {
                result.extend(instruction_set.jumps.clone());
            }
            result
        };
        assert_eq!(jumps.len(), 2);
    }

    // - Jump value pushed in different section
    #[test]
    fn jump_push_in_other_section2() {
        let input = Vec::from([
            PUSH1 as u32,
            0x7, //0x0,0x1
            PUSH1 as u32,
            0x5,             // 0x2, 0x3, jumping to second section
            JUMP as u32,     //0x4
            JUMPDEST as u32, //0x5
            JUMP as u32,     //0x6
            JUMPDEST as u32, //0x7
            STOP as u32,     //0x8
        ]);

        let mut parser = Parser::new(input);
        let instruction_sections = parser.get_instruction_sets();
        assert_eq!(instruction_sections.len(), 3);
        //// Not all jumps can be resolved initially, since we need to resolve a bit deeper first.
        //assert!(!parser.all_jumps_resolved());
        //parser.resolve_jumps();
        //assert!(parser.all_jumps_resolved());

        //let jumps = parser.get_all_jumps();
        //assert_eq!(jumps.len(), 2);
        //let section = jumps
        //    .iter()
        //    .find(|&jump| jump.source == Hex(0) && get_option_value(&jump.target) == Some(Hex(5)));
        //assert!(section.is_some());

        //let section = jumps
        //    .iter()
        //    .find(|&jump| jump.source == Hex(5) && get_option_value(&jump.target) == Some(Hex(7)));
        //assert!(section.is_some());
    }

    #[test]
    fn jump_push_in_other_section_nested() {
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

        let mut parser = Parser::new(input);
        let instruction_sections = parser.get_instruction_sets();
        assert_eq!(instruction_sections.len(), 4);

        //parser.resolve_jumps();
        //let jumps = parser.get_all_jumps();
        //assert_eq!(jumps.len(), 3);
        //assert_ne!(jumps.iter().find(|jump| jump.source == Hex(0)), None);
        //assert_ne!(
        //    jumps
        //        .iter()
        //        .find(|jump| get_option_value(&jump.target) == Some(Hex(0x7))),
        //    None
        //);

        //assert_ne!(jumps.iter().find(|jump| jump.source == Hex(7)), None);
        //assert_ne!(
        //    jumps
        //        .iter()
        //        .find(|jump| get_option_value(&jump.target) == Some(Hex(0x9))),
        //    None
        //);

        //assert_ne!(jumps.iter().find(|jump| jump.source == Hex(9)), None);
        //assert_ne!(
        //    jumps
        //        .iter()
        //        .find(|jump| get_option_value(&jump.target) == Some(Hex(0xb))),
        //    None
        //);
    }

    fn get_option_value(target: &Option<StackElement>) -> Option<Hex> {
        if let Some(target) = target {
            Some(target.value)
        } else {
            None
        }
    }

    #[test]
    fn jump_push_in_other_section_nested2() {
        // same as jump_push_in_other_section_nested, but in a different sequence
        let input = Vec::from([
            PUSH1 as u32,
            0xb, //0x0,0x1
            PUSH1 as u32,
            0x7, // 0x4, 0x5, jumping to second section
            PUSH1 as u32,
            0x9,             //0x2,0x3
            JUMP as u32,     //0x6
            JUMPDEST as u32, //0x7
            JUMP as u32,     //0x8
            JUMPDEST as u32, //0x9
            JUMP as u32,     //0x1
            JUMPDEST as u32, //0xb
            STOP as u32,     //0xc
        ]);

        let mut parser = Parser::new(input);
        let instruction_sections = parser.get_instruction_sets();
        assert_eq!(instruction_sections.len(), 4);

        //parser.resolve_jumps();
        //let jumps = parser.get_all_jumps();
        //assert_eq!(jumps.len(), 3);
        //assert_ne!(jumps.iter().find(|jump| jump.source == Hex(0)), None);
        //assert_ne!(
        //    jumps.iter().find(|jump| jump.target
        //        == Some(StackElement {
        //            value: Hex(7),
        //            origin: Hex(2),
        //            size: 1
        //        })),
        //    None
        //);

        //assert_ne!(jumps.iter().find(|jump| jump.source == Hex(7)), None);
        //assert_ne!(
        //    jumps
        //        .iter()
        //        .find(|jump| if let Some(target) = &jump.target {
        //            Some(target.value)
        //        } else {
        //            None
        //        } == Some(Hex(9))),
        //    None
        //);

        //assert_ne!(jumps.iter().find(|jump| jump.source == Hex(9)), None);
        //assert_ne!(
        //    jumps
        //        .iter()
        //        .find(|jump| if let Some(target) = &jump.target {
        //            Some(target.value)
        //        } else {
        //            None
        //        } == Some(Hex(0xb))),
        //    None
        //);
    }

    #[test]
    fn conditional_jump() {
        let input = Vec::from([
            PUSH1 as u32,
            0x2, // 0x0, 0x1
            PUSH1 as u32,
            0x1,             //0x2, 0x3
            JUMPI as u32,    //0x4
            JUMP as u32,     // 0x5
            JUMPDEST as u32, // 0x6
            STOP as u32,     //0x7
            JUMPDEST as u32, //0x8
            STOP as u32,     //0x9
        ]);
        let mut parser = Parser::new(input);
        let instruction_sets = parser.get_instruction_sets();
        assert_eq!(instruction_sets.len(), 3);
    }
}
