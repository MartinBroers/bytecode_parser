use std::collections::HashMap;

#[cfg(not(test))]
use log::{error, info, warn}; // Use log crate when building application

#[cfg(test)]
use std::{println as info, println as warn, println as error}; // Workaround to use prinltn! for logs.

use crate::{
    instruction::{Hex, Instruction, InstructionSet, JumpInstruction, JumpType, ParsedInstruction},
    opcode::{self, opcodes},
    utils::find_sequence,
};

pub struct Parser {
    _bytecode: Vec<u32>,
    _cbor_part: Vec<u32>,
    instructions: HashMap<Hex, Instruction>,
}

const NO_LABEL: &str = "";
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
        Parser {
            instructions: bytecode_to_instructions(input.to_vec()),
            _bytecode: input,
            _cbor_part: cbor_part,
        }
    }

    pub fn create_instruction_sets(&mut self) -> HashMap<Hex, InstructionSet> {
        let mut instrution_sets: HashMap<Hex, InstructionSet> = HashMap::new();

        let mut stack_pointer = 0x0.into();
        while let Some(instruction_set) =
            parse_instruction_section(stack_pointer, &self.instructions, None)
        {
            info!("instruction_set: {:?}", instruction_set);
            instrution_sets.insert(stack_pointer, instruction_set.clone());
            stack_pointer = instruction_set.end + Hex(1);
        }
        instrution_sets
    }

    fn get_jumps(&self, instruction_sets: &HashMap<Hex, InstructionSet>) -> Vec<JumpInstruction> {
        let mut jumps = Vec::new();
        // First, clone all jumps into a single array, so we can iterate over them
        for (_, instruction_set) in instruction_sets.iter() {
            for jump in &instruction_set.jumps {
                jumps.push(jump.clone());
            }
        }
        jumps
    }

    fn find_calling_sections(
        &self,
        dest: &Hex,
        jumps: &Vec<JumpInstruction>,
    ) -> Vec<JumpInstruction> {
        let mut origins = Vec::new();
        for origin in jumps.iter() {
            if let Some(target) = origin.target {
                if target == *dest {
                    origins.push(origin.clone());
                }
            }
        }
        origins
    }

    fn jump_target_search(
        &self,
        target_instruction: &Hex,
        instruction_sets: &HashMap<Hex, InstructionSet>,
        jumps: &Vec<JumpInstruction>,
    ) -> (Vec<JumpInstruction>, Vec<InstructionSet>) {
        println!("jump_target_search {:?}", target_instruction);
        let mut jump_result = Vec::new();
        let mut instruction_set_results = Vec::new();
        // we could not get the jump dest from this local stack, so we need to enlarge the
        // stack. We have to look from which location(s) to this point a jump is
        // originated. So, we have to find all jumps with target = jump.start.
        let origins = self.find_calling_sections(&target_instruction, &jumps);
        info!(
            "Found jumps {:?} jumping to us ({:?})",
            origins, target_instruction
        );
        // Now, we can extract the stack at the end of the previous section (only when
        // unconditional?) and append that to our stack, see if we can obtain a more
        // complete stack and see if we can obtain a target for this jump.
        for origin in origins {
            if let Some(instruction_set) = instruction_sets.get(&origin.source) {
                let stack = &instruction_set.stack;
                info!("Stack: {:?}", stack);
                let new_instruction_section = parse_instruction_section(
                    origin.target.unwrap(),
                    &self.instructions,
                    Some(stack.to_vec()),
                );
                if let Some(new_instruction_section) = new_instruction_section {
                    for new_jump in &new_instruction_section.jumps {
                        if new_jump.target.is_none() {
                            panic!("Could not find target for jump even after increasing stack.");
                        } else {
                            jump_result.push(new_jump.clone());
                            instruction_set_results.push(new_instruction_section.clone());
                        }
                    }
                } else {
                    panic!("parsing existing section failed. Please report.");
                }
            } else {
                panic!("Could not find source destination set.");
            }
        }
        (jump_result, instruction_set_results)
    }

    fn jump_target_depth_search(
        &self,
        target_instruction: &Hex,
        instruction_sets: &HashMap<Hex, InstructionSet>,
        jumps: &Vec<JumpInstruction>,
        depth: usize,
    ) -> Vec<JumpInstruction> {
        println!("jump_target_depth_search: {:?}", target_instruction);
        let mut result = Vec::new();
        // find the first set
        let (jumps, instructions) =
            self.jump_target_search(target_instruction, instruction_sets, jumps);
        println!("Found jumps:{:?}, instructions: {:?}", jumps, instructions);
        for jump in &jumps {
            println!(
                "target_instruction: {:?}, jump_target: {:?}",
                target_instruction, jump.target
            );
            if let Some(_) = jump.target {
                result.push(jump.clone());
            } else {
                // level 1 search didn't find anything.
                if depth > 0 {
                    // we could not reconstruct the stack enough, so we need to look a bit farther
                    // back. First, we need a list of instruction_sets which point to here. Then,
                    // we will look which instruction_sets jump to those and append their stack to
                    // them to see if we can reconstruct the jumps we cannot construct currently.
                    // We have all instructions pointing here.
                    for instruction_set in &instructions {
                        println!("SEARCHING DEEPER");
                        let instruction_new_section = parse_instruction_section(
                            instruction_set.start,
                            &self.instructions,
                            None,
                        )
                        .unwrap();
                        let mut new_instruction_sets = instruction_sets.clone();
                        let _ = new_instruction_sets
                            .insert(instruction_new_section.start, instruction_new_section)
                            .expect("Replaced new instruction section, but got none back");
                        return self.jump_target_depth_search(
                            target_instruction,
                            &new_instruction_sets,
                            &jumps,
                            depth - 1,
                        );
                    }
                } else {
                    panic!("Stopping search due to depth (stack) limit");
                }
            }
        }
        result
    }

    pub fn parse_jumps(
        &mut self,
        instruction_sets: &mut HashMap<Hex, InstructionSet>,
    ) -> Vec<JumpInstruction> {
        let mut result = Vec::new();
        let jumps = self.get_jumps(&instruction_sets);

        println!("jumps: {:?}", jumps);
        for jump in jumps.iter() {
            if jump.target.is_some() {
                result.push(jump.clone());
            } else if jump.target.is_none() {
                let results =
                    self.jump_target_depth_search(&jump.source, instruction_sets, &jumps, 5);
                println!("Results: {:?}", results);
                result.extend(results);
            }
        }
        println!("parse_jumps: {:?}", result);

        result
    }
}

fn parse_instruction_section(
    stack_pointer: Hex,
    instructions: &HashMap<Hex, Instruction>,
    input_stack: Option<Vec<Hex>>,
) -> Option<InstructionSet> {
    info!("Parsing {:?}", stack_pointer);
    let mut instructions_section: InstructionSet = InstructionSet {
        instructions: Vec::new(),
        start: stack_pointer,
        end: stack_pointer,
        jumps: Vec::new(),
        jump_origins: Vec::new(),
        stack: Vec::new(),
    };
    let mut stack: Vec<Hex> = input_stack.unwrap_or(Vec::new());
    let mut stack_pointer = stack_pointer;
    while let Some(instruction) = instructions.get(&stack_pointer) {
        info!(
            "Handling instruction {:?} at {:?}",
            instruction, &stack_pointer
        );
        let instruction = instruction.clone();
        match instruction.opcode.code {
            opcode::JUMPDEST => {
                let label = format!("LABEL_0x{:04x}", instruction.index);
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    label,
                    ..Default::default()
                })
            }
            opcode::PUSH0..=opcode::PUSH32 => {
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    label: NO_LABEL.to_string(),
                    ..Default::default()
                });
                for arg in &instruction.args {
                    stack.push(*arg);
                    stack_pointer += 1.into();
                }
            }
            opcode::JUMP => {
                let jumpdest = stack.pop();
                let mut label: String = "JUMP_".to_string();
                label.push_str(format!("{:?}", jumpdest).as_str());
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    label,
                    used_arg: jumpdest,
                });
                instructions_section.jumps.push(JumpInstruction {
                    instruction: instruction.clone(),
                    target: jumpdest,
                    source: instructions_section.start,
                    jump_type: JumpType::Unconditional,
                });

                break;
            }
            opcode::JUMPI => {
                let jumpdest = stack.pop();
                let _condition = stack.pop();
                let mut label: String = "JUMPI_".to_string();
                label.push_str(format!("{:?}", jumpdest).as_str());
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    label,
                    used_arg: jumpdest,
                });
                instructions_section.jumps.push(JumpInstruction {
                    instruction: instruction.clone(),
                    target: jumpdest,
                    source: instructions_section.start,
                    jump_type: JumpType::Conditional,
                });
                //let mut stack = stack.clone();
                //println!("Parsing JUMPI");
                //self.parse_instruction_section(
                //    jumpdest,
                //    instructions_section,
                //    labeled_instructions,
                //    &mut stack,
                //);
                //println!("Done");
            }
            opcode::REVERT => {
                let (_offset, _length) = (stack.pop(), stack.pop());
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    label: "END".to_string(),
                    ..Default::default()
                });
                break;
            }

            opcode::STOP => {
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    label: "END".to_string(),
                    ..Default::default()
                });
                break;
            }
            opcode::POP => {
                let arg = stack.pop();
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    used_arg: arg,
                    ..Default::default()
                })
            }
            _ => {
                println!("Parsing other opcode, let's check default arguments if any");
                for _ in 0..instruction.opcode.stack_inputs {
                    info!("Popping stack");
                    stack.pop();
                }
                for _ in 0..instruction.opcode.stack_outputs {
                    info!("Adding *some* value to the stack");
                    stack.push(Hex(0x0));
                }
                instructions_section.push(ParsedInstruction {
                    instruction: instruction.clone(),
                    ..Default::default()
                })
            } //opcode::SWAP1..=opcode::SWAP16 => {
              //    let operator_index = instruction.opcode.operator_index;
              //    stack.swap(0, operator_index);
              //    instructions_section.push(BytecodeInstruction {
              //        instruction: instruction.clone(),
              //        ..Default::default()
              //    })
              //}

              //elif opcode.startswith("DUP"):
              //    position = int(opcode[3:])
              //    stack.duplicate(position)
              //    instructions_section.append((index, opcode, args, "", list(stack.stack), None))
              //elif opcode == "MSTORE":
              //    stack.pop()
              //    stack.pop()
              //    instructions_section.append((index, opcode, args, "", list(stack.stack), None))
              //elif opcode == "CALLVALUE":
              //    stack.push("CALLVALUE")
              //    instructions_section.append((index, opcode, args, "", list(stack.stack), None))
              //else:
              //    instructions_section.append((index, opcode, args, "", list(stack.stack), None))
        }
        stack_pointer += 1.into();
    }
    if instructions_section.len() > 0 {
        instructions_section.stack = stack;
        instructions_section.end = stack_pointer;
        Some(instructions_section)
    } else {
        None
    }
}
fn bytecode_to_instructions(raw_bytecode: Vec<u32>) -> HashMap<Hex, Instruction> {
    let mut instructions: HashMap<Hex, Instruction> = HashMap::new();
    let opcodes = opcodes();

    //let mut iterator = self.raw_bytecode.iter();
    let mut iterator = raw_bytecode.iter().enumerate();
    while let Some((index, instruction)) = iterator.next() {
        // Check if instruction is a valid opcode
        if let Some(opcode) = opcodes.get(instruction) {
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
            //println!("{:04x}\t{:?}", index, instruction);
            instructions.insert(index.into(), instruction);
        } else {
            panic!("Found unknown instruction: {:x}, aborting", instruction);
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::Hex,
        opcode::{ADD, JUMP, JUMPDEST, POP, PUSH1, STOP},
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
    fn break_into_simple_instruction_sections() {
        let input = Vec::from([
            PUSH1, 0x3,      // 0x0, 0x1 argument for PUSH1
            JUMP,     // 0x2
            JUMPDEST, // 0x3
            STOP,     // 0x4
        ]);

        let mut parser = Parser::new(input);
        let instruction_sections = parser.create_instruction_sets();

        // we have two sections; one before the jump and one after the jump. From JUMPDEST to STOP.
        assert_eq!(instruction_sections.len(), 2);
        let jump_target = instruction_sections
            .get(&Hex(0x0))
            .unwrap()
            .jumps
            .first()
            .unwrap()
            .target;
        let jump_dest = instruction_sections.get(&Hex(0x3)).unwrap().start;
        assert_eq!(jump_target, Some(jump_dest));
    }
    #[test]
    fn instructions_between_jump_target_push_and_jump_instruction() {
        let input = Vec::from([
            PUSH1, 0x9, // 0x0, 0x1 argument for PUSH1
            PUSH1, 0x43, // 0x2 push arguments for ADD command
            PUSH1, 0x48,     // 0x4
            ADD,      // 0x6
            POP,      // 0x7
            JUMP,     // 0x8
            JUMPDEST, // 0x9
            STOP,     // 0xa
        ]);

        let mut parser = Parser::new(input);
        let instruction_sections = parser.create_instruction_sets();
        assert_eq!(instruction_sections.len(), 2);
        println!("instruction_sections: {:?}", instruction_sections);
        let jump_target = instruction_sections
            .get(&Hex(0x0))
            .unwrap()
            .jumps
            .first()
            .unwrap()
            .target;
        let jump_dest = instruction_sections.get(&Hex(0x9)).unwrap().start;
        assert_eq!(jump_target, Some(jump_dest));
    }

    #[test]
    fn find_related_jumps() {
        let input = Vec::from([
            PUSH1, 0x3,      // jumping to second section
            JUMP,     //0x2
            JUMPDEST, //0x3
            PUSH1, 0x7,      //0x4,0x5
            JUMP,     //0x6
            JUMPDEST, //0x7
            STOP,     //0x8
        ]);

        let mut parser = Parser::new(input);
        let mut instruction_sections = parser.create_instruction_sets();
        assert_eq!(instruction_sections.len(), 3);

        let jumps = parser.parse_jumps(&mut instruction_sections);
        assert_eq!(jumps.len(), 2);

        println!("jumps: {:?}", jumps);
        assert_ne!(jumps.iter().find(|jump| jump.source == Hex(0)), None);
        assert_ne!(jumps.iter().find(|jump| jump.target == Some(Hex(3))), None);

        assert_ne!(jumps.iter().find(|jump| jump.source == Hex(3)), None);
        assert_ne!(jumps.iter().find(|jump| jump.target == Some(Hex(7))), None);
    }

    // - Jump value pushed in different section
    #[test]
    fn jump_push_in_other_section() {
        let input = Vec::from([
            PUSH1, 0x7, //0x0,0x1
            PUSH1, 0x5,      // 0x2, 0x3, jumping to second section
            JUMP,     //0x4
            JUMPDEST, //0x5
            JUMP,     //0x6
            JUMPDEST, //0x7
            STOP,     //0x8
        ]);

        let mut parser = Parser::new(input);
        let mut instruction_sections = parser.create_instruction_sets();
        assert_eq!(instruction_sections.len(), 3);

        let jumps = parser.parse_jumps(&mut instruction_sections);
        assert_eq!(jumps.len(), 2);
        assert_eq!(jumps.first().unwrap().source, Hex(0));
        assert_eq!(jumps.first().unwrap().target, Some(Hex(5)));

        assert_eq!(jumps.last().unwrap().source, Hex(5));
        assert_eq!(jumps.last().unwrap().target, Some(Hex(7)));
    }

    #[test]
    fn jump_push_in_other_section_nested() {
        let input = Vec::from([
            PUSH1, 0xb, //0x0,0x1
            PUSH1, 0x9, //0x2,0x3
            PUSH1, 0x7,      // 0x4, 0x5, jumping to second section
            JUMP,     //0x6
            JUMPDEST, //0x7
            JUMP,     //0x8
            JUMPDEST, //0x9
            JUMP,     //0x1
            JUMPDEST, //0xb
            STOP,     //0xc
        ]);

        let mut parser = Parser::new(input);
        let mut instruction_sections = parser.create_instruction_sets();
        assert_eq!(instruction_sections.len(), 4);

        let jumps = parser.parse_jumps(&mut instruction_sections);
        assert_eq!(jumps.len(), 3);
        assert_ne!(jumps.iter().find(|jump| jump.source == Hex(0)), None);
        assert_ne!(jumps.iter().find(|jump| jump.target == Some(Hex(7))), None);

        assert_ne!(jumps.iter().find(|jump| jump.source == Hex(7)), None);
        assert_ne!(jumps.iter().find(|jump| jump.target == Some(Hex(9))), None);

        assert_ne!(jumps.iter().find(|jump| jump.source == Hex(9)), None);
        assert_ne!(
            jumps.iter().find(|jump| jump.target == Some(Hex(0xb))),
            None
        );
    }

    // - Jump to halfway in a section.
    // - Have a few sections refer circular jumps
    // - Have conditional jumps
}
