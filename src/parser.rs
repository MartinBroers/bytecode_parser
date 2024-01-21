use std::collections::HashMap;

#[cfg(not(test))]
use log::{error, info, warn}; // Use log crate when building application

#[cfg(test)]
use std::{println as info, println as warn, println as error}; // Workaround to use prinltn! for logs.

use crate::{
    instruction::{
        BytecodeInstruction, Hex, Instruction, InstructionSet, JumpInstruction, JumpType,
    },
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
            instructions: parse(input.to_vec()),
            _bytecode: input,
            _cbor_part: cbor_part,
        }
    }

    fn parse_instruction_section(&mut self, stack_pointer: Hex) -> Option<InstructionSet> {
        info!("Parsing {:?}", stack_pointer);
        let mut instructions_section: InstructionSet = InstructionSet {
            instructions: Vec::new(),
            jumpdest: stack_pointer,
            end: stack_pointer,
            jumps: Vec::new(),
            stack: Vec::new(),
        };
        let mut stack: Vec<Hex> = Vec::new();
        let mut stack_pointer = stack_pointer;
        while let Some(instruction) = self.instructions.get(&stack_pointer) {
            info!(
                "Handling instruction {:?} at {:?}",
                instruction, &stack_pointer
            );
            let mut instruction = instruction.clone();
            instruction.parsed = true;
            self.instructions.insert(stack_pointer, instruction.clone());
            match instruction.opcode.code {
                opcode::JUMPDEST => {
                    let label = format!("LABEL_0x{:04x}", instruction.index);
                    instructions_section.push(BytecodeInstruction {
                        instruction: instruction.clone(),
                        label,
                        ..Default::default()
                    })
                }
                opcode::PUSH0..=opcode::PUSH32 => {
                    instructions_section.push(BytecodeInstruction {
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
                    let jumpdest = match stack.pop() {
                        Some(v) => v,
                        None => 0x0.into(),
                    };
                    let mut label: String = "JUMP_".to_string();
                    label.push_str(format!("{:?}", jumpdest).as_str());
                    instructions_section.push(BytecodeInstruction {
                        instruction: instruction.clone(),
                        label,
                        used_arg: Some(jumpdest),
                    });
                    instructions_section.jumps.push(JumpInstruction {
                        instruction: instruction.clone(),
                        target: jumpdest,
                        source: instructions_section.jumpdest,
                        jump_type: JumpType::Unconditional,
                    });

                    break;
                }
                opcode::JUMPI => {
                    let jumpdest = match stack.pop() {
                        Some(v) => v,
                        None => 0x0.into(),
                    };
                    let _condition = stack.pop();
                    let mut label: String = "JUMPI_".to_string();
                    label.push_str(format!("{:?}", jumpdest).as_str());
                    instructions_section.push(BytecodeInstruction {
                        instruction: instruction.clone(),
                        label,
                        used_arg: Some(jumpdest),
                    });
                    instructions_section.jumps.push(JumpInstruction {
                        instruction: instruction.clone(),
                        target: jumpdest,
                        source: instructions_section.jumpdest,
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
                    instructions_section.push(BytecodeInstruction {
                        instruction: instruction.clone(),
                        label: "END".to_string(),
                        ..Default::default()
                    });
                    break;
                }

                opcode::STOP => {
                    instructions_section.push(BytecodeInstruction {
                        instruction: instruction.clone(),
                        label: "END".to_string(),
                        ..Default::default()
                    });
                    break;
                }
                opcode::POP => {
                    let arg = stack.pop();
                    instructions_section.push(BytecodeInstruction {
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
                    instructions_section.push(BytecodeInstruction {
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
    pub fn label_instructions(&mut self) -> HashMap<Hex, InstructionSet> {
        let mut labeled_instructions: HashMap<Hex, InstructionSet> = HashMap::new();

        //for instruction in &self.instructions {
        let mut stack_pointer = 0x0.into();
        while let Some(instruction_set) = self.parse_instruction_section(stack_pointer) {
            info!("instruction_set: {:?}", instruction_set);
            if labeled_instructions.contains_key(&stack_pointer) {
                break;
            }
            labeled_instructions.insert(stack_pointer, instruction_set.clone());
            for jump in instruction_set.clone().jumps {
                match jump.jump_type {
                    JumpType::Conditional => {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            labeled_instructions.entry(jump.target)
                        {
                            if let Some(instruction_set) =
                                self.parse_instruction_section(jump.target)
                            {
                                stack_pointer = instruction_set.end + 1.into();
                                e.insert(instruction_set);
                            }
                        }
                    }
                    JumpType::Unconditional => {
                        if labeled_instructions.contains_key(&jump.target) {
                            break;
                        } else {
                            stack_pointer = jump.target;
                        }
                    }
                }
            }
        }
        labeled_instructions
    }

    pub fn relate_instructions(&mut self, labeled_instructions: HashMap<Hex, InstructionSet>) {
        //println!("labeled_instructions:");
        //for instruction_set in &labeled_instructions {
        //    //println!("{:?}", instruction_set);
        //    println!("{:?}", instruction_set);
        //}

        //for instruction_set in &labeled_instructions {
        //    println!("Jumps: {:?}", instruction_set.jumps);
        //    for jump in &instruction_set.jumps {
        //        let target = &jump.target;
        //        for possible_target in &self.instructions {
        //            if possible_target.index == 0x0.into() {
        //                println!("Jump: {:?} to {:?}", jump, possible_target);
        //                println!("{:?}", instruction_set);
        //            }
        //        }
        //    }
        //}

        println!("Finding unresolved combinations");
        labeled_instructions
            .iter()
            .for_each(|(_, instruction_set)| {
                // Find sets where the JUMP or JUMPI target is undefined
                for jump in &instruction_set.jumps {
                    if jump.target == 0x0.into() {
                        //println!("Undefined jump: {:?}", jump);
                        //println!("Instruction_set: {:?}", instruction_set);
                        let mut found = false;

                        // The next step is to determine if there are instructions pointing at this
                        // instruction set, so we can look up that stack to check if we can know where
                        // this jump jumps to.
                        labeled_instructions.iter().for_each(|(_, source_set)| {
                            for source_jump in &source_set.jumps {
                                if source_jump.target >= instruction_set.jumpdest
                                    && source_jump.target <= jump.instruction.index
                                {
                                    info!("{:?} jumps to {:?}", source_set, instruction_set);
                                    found = true;
                                }
                            }
                        });
                        if !found {
                            warn!("Could not find jump for {:?}", instruction_set);
                        }
                    }
                }
            });
    }
}

fn parse(raw_bytecode: Vec<u32>) -> HashMap<Hex, Instruction> {
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
                parsed: false,
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

    use super::{parse, Parser};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn simple_push() {
        init();
        let result = parse([0x60, 0x80].to_vec());
        assert_eq!(result.keys().len(), 1);
        let instruction = result.get(&Hex(0x0)).unwrap();
        assert_eq!(instruction.args.len(), 1);
        assert_eq!(*instruction.args.first().unwrap(), Hex(0x80));
        assert_eq!(instruction.opcode.code, PUSH1);
        assert_eq!(instruction.index, Hex(0x0));
        assert!(!instruction.parsed);
    }
    #[test]
    #[should_panic]
    fn simple_push_no_arg() {
        init();
        let _ = parse([0x60].to_vec());
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
        let instruction_sections = parser.label_instructions();

        // we have two sections; one before the jump and one after the jump. From JUMPDEST to STOP.
        assert_eq!(instruction_sections.len(), 2);
        let jump_target = instruction_sections
            .get(&Hex(0x0))
            .unwrap()
            .jumps
            .first()
            .unwrap()
            .target;
        let jump_dest = instruction_sections.get(&Hex(0x3)).unwrap().jumpdest;
        assert_eq!(jump_target, jump_dest);
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
        let instruction_sections = parser.label_instructions();
        assert_eq!(instruction_sections.len(), 2);
        let jump_target = instruction_sections
            .get(&Hex(0x0))
            .unwrap()
            .jumps
            .first()
            .unwrap()
            .target;
        let jump_dest = instruction_sections.get(&Hex(0x9)).unwrap().jumpdest;
        assert_eq!(jump_target, jump_dest);
    }
}
