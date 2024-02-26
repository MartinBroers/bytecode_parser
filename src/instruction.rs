use core::fmt;
use std::num;

use crate::{
    hex::Hex,
    memory::Memory,
    opcode::{OpCode, OpCodeResult, OpCodes},
    stack::{Stack, StackElement},
    CALLDATA, CALLVALUE,
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub args: Vec<Hex>,
    pub opcode: OpCode,
    pub index: Hex,
}
impl Instruction {
    fn stop(&self, _stack: &mut Stack) -> Result<OpCodeResult, ()> {
        Ok(OpCodeResult::End)
    }
    fn pop(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        stack.pop();
        Ok(OpCodeResult::Ok)
    }
    fn add(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let left = stack.pop().ok_or(())?;
        let right = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: left.value + right.value,
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    fn sub(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let left = stack.pop().ok_or(())?;
        let right = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: left.value - right.value,
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    fn slt(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let right = stack.pop().ok_or(())?;
        let left = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: if left.value < right.value {
                Hex(1)
            } else {
                Hex(0)
            },
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    fn mul(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let left = stack.pop().ok_or(())?;
        let right = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: left.value * right.value,
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    fn lt(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let left = stack.pop().ok_or(())?;
        let right = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: if left.value < right.value {
                Hex(1)
            } else {
                Hex(0)
            },
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    fn swapx(&self, num_swap: u32, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let num_swap = num_swap as usize;
        assert!(stack.len() >= num_swap);
        stack.swap(stack.len() - num_swap - 1, stack.len() - 1);

        Ok(OpCodeResult::Ok)
    }
    fn pushx(&self, num_push: usize, stack: &mut Stack, pc: &mut Hex) -> Result<OpCodeResult, ()> {
        *pc += Hex(1) + Hex((num_push - 1) as u128);
        assert!(self.args.len() == num_push);
        let mut value = Hex(0);
        for element in &self.args {
            value = value << Hex(8);
            value += *element;
        }
        stack.push(StackElement {
            value,
            origin: self.index,
            size: num_push,
        });
        Ok(OpCodeResult::Ok)
    }

    fn dupx(&self, num_dup: usize, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let dup = stack.get(stack.len() - num_dup);
        assert!(dup.is_some());
        if let Some(value) = dup {
            stack.push(value.clone());
            Ok(OpCodeResult::Ok)
        } else {
            Err(())
        }
    }
    fn shr(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let shift = stack.pop().ok_or(())?;
        let value = stack.pop().ok_or(())?;

        stack.push(StackElement {
            value: value.value >> shift.value,
            origin: self.index,
            size: 1,
        });

        Ok(OpCodeResult::Ok)
    }

    fn sload(&self, stack: &mut Stack, memory: &Memory) -> Result<OpCodeResult, ()> {
        let key = stack.pop().ok_or(())?;
        stack.push(memory.sload(key.value));
        Ok(OpCodeResult::Ok)
    }

    fn jumpdest(&self) -> Result<OpCodeResult, ()> {
        Ok(OpCodeResult::Ok)
    }
    fn jumpi(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let target = stack.pop().ok_or(())?;
        let condition = stack.pop().ok_or(())?;
        println!("jumpi: target: {:?}, condition: {:?}", target, condition);
        let jump_instruction = JumpInstruction {
            instruction: self.clone(),
            jump_type: JumpType::Conditional,
            target,
            condition: Some(condition),
            source: self.index.clone(),
        };

        Ok(OpCodeResult::ConditionalJumpInstruction(jump_instruction))
    }
    fn jump(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let jump_instruction = JumpInstruction {
            instruction: self.clone(),
            jump_type: JumpType::Unconditional,
            target: stack.pop().ok_or(())?,
            condition: None,
            source: self.index.clone(),
        };

        Ok(OpCodeResult::JumpInstruction(jump_instruction))
    }

    fn mstore(&self, stack: &mut Stack, memory: &mut Memory) -> Result<OpCodeResult, ()> {
        let offset = stack.pop().ok_or(())?;
        let value = stack.pop().ok_or(())?;
        memory.mstore(value, offset.value, self.index);
        Ok(OpCodeResult::Ok)
    }
    fn mload(&self, stack: &mut Stack, memory: &mut Memory) -> Result<OpCodeResult, ()> {
        let offset = stack.pop().ok_or(())?;
        let result = memory.mload(offset.value);
        stack.push(StackElement {
            value: result.value,
            origin: if let Some(origin) = result.origin {
                origin
            } else {
                self.index
            },
            size: 32,
        });
        Ok(OpCodeResult::Ok)
    }

    fn calldataload(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let offset: usize = stack.pop().unwrap().value.0.try_into().unwrap();
        if let Some(calldata) = unsafe { &CALLDATA } {
            let value = calldata.get(offset).unwrap();
            stack.push(value);
        } else {
            stack.push(StackElement {
                origin: self.index,
                value: Hex(0),
                size: 1,
            });
        }
        Ok(OpCodeResult::Ok)
    }
    fn calldatasize(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        if let Some(calldata) = unsafe { &CALLDATA } {
            let value = calldata.size();
            stack.push(value);
        } else {
            stack.push(StackElement {
                origin: self.index,
                value: Hex(0),
                size: 1,
            });
        }
        Ok(OpCodeResult::Ok)
    }

    fn callvalue(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        if let Some(callvalue) = unsafe { &CALLVALUE } {
            stack.push(StackElement {
                origin: self.index,
                value: callvalue.value,
                size: callvalue.size,
            });
        } else {
            stack.push(StackElement {
                origin: self.index,
                value: Hex(0),
                size: 1,
            });
        }
        Ok(OpCodeResult::Ok)
    }
    fn eq(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let left = stack.pop().ok_or(())?;
        let right = stack.pop().ok_or(())?;

        stack.push(StackElement {
            value: if left.value == right.value {
                Hex(1)
            } else {
                Hex(0)
            },
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    fn is_zero(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let value = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: if value.value == Hex(0) {
                Hex(1)
            } else {
                Hex(0)
            },
            origin: self.index,
            size: 1,
        });
        Ok(OpCodeResult::Ok)
    }
    // Parses the opcode and returns the stack
    pub fn parse(
        &self,
        stack: &mut Stack,
        pc: &mut Hex,
        memory: &mut Memory,
    ) -> Result<OpCodeResult, ()> {
        match self.opcode.code {
            OpCodes::ADD => self.add(stack),
            OpCodes::ADDMOD => todo!(),
            OpCodes::ADDRESS => todo!(),
            OpCodes::AND => todo!(),
            OpCodes::BALANCE => todo!(),
            OpCodes::BASEFEE => todo!(),
            OpCodes::BLOCKHASH => todo!(),
            OpCodes::BYTE => todo!(),
            OpCodes::CALL => todo!(),
            OpCodes::CALLCODE => todo!(),
            OpCodes::CALLDATACOPY => todo!(),
            OpCodes::CALLDATALOAD => self.calldataload(stack),
            OpCodes::CALLDATASIZE => self.calldatasize(stack),
            OpCodes::CALLER => todo!(),
            OpCodes::CALLVALUE => self.callvalue(stack),
            OpCodes::CHAINID => todo!(),
            OpCodes::CODECOPY => todo!(),
            OpCodes::CODESIZE => todo!(),
            OpCodes::COINBASE => todo!(),
            OpCodes::CREATE => todo!(),
            OpCodes::CREATE2 => todo!(),
            OpCodes::DELEGATECALL => todo!(),
            OpCodes::DIFFICULTY => todo!(),
            OpCodes::DIV => todo!(),
            OpCodes::DUP1 => self.dupx(1, stack),
            OpCodes::DUP2 => self.dupx(2, stack),
            OpCodes::DUP3 => self.dupx(3, stack),
            OpCodes::DUP4 => self.dupx(4, stack),
            OpCodes::DUP5 => self.dupx(5, stack),
            OpCodes::DUP6 => self.dupx(6, stack),
            OpCodes::DUP7 => self.dupx(7, stack),
            OpCodes::DUP8 => self.dupx(8, stack),
            OpCodes::DUP9 => self.dupx(9, stack),
            OpCodes::DUP10 => self.dupx(10, stack),
            OpCodes::DUP11 => self.dupx(11, stack),
            OpCodes::DUP12 => self.dupx(12, stack),
            OpCodes::DUP13 => self.dupx(13, stack),
            OpCodes::DUP14 => self.dupx(14, stack),
            OpCodes::DUP15 => self.dupx(15, stack),
            OpCodes::DUP16 => self.dupx(16, stack),
            OpCodes::EOFMAGIC => todo!(),
            OpCodes::EQ => self.eq(stack),
            OpCodes::EXP => todo!(),
            OpCodes::EXTCODECOPY => todo!(),
            OpCodes::EXTCODEHASH => todo!(),
            OpCodes::EXTCODESIZE => todo!(),
            OpCodes::GAS => todo!(),
            OpCodes::GASLIMIT => todo!(),
            OpCodes::GASPRICE => todo!(),
            OpCodes::GT => todo!(),
            OpCodes::INVALID => todo!(),
            OpCodes::ISZERO => self.is_zero(stack),
            OpCodes::JUMP => self.jump(stack),
            OpCodes::JUMPDEST => self.jumpdest(),
            OpCodes::JUMPI => self.jumpi(stack),
            OpCodes::LOG0 => todo!(),
            OpCodes::LOG1 => todo!(),
            OpCodes::LOG2 => todo!(),
            OpCodes::LOG3 => todo!(),
            OpCodes::LOG4 => todo!(),
            OpCodes::LT => self.lt(stack),
            OpCodes::MLOAD => self.mload(stack, memory),
            OpCodes::MOD => todo!(),
            OpCodes::MSIZE => todo!(),
            OpCodes::MSTORE => self.mstore(stack, memory),
            OpCodes::MSTORE8 => todo!(),
            OpCodes::MUL => self.mul(stack),
            OpCodes::MULMOD => todo!(),
            OpCodes::NOT => todo!(),
            OpCodes::NUMBER => todo!(),
            OpCodes::OR => todo!(),
            OpCodes::ORIGIN => todo!(),
            OpCodes::PC => todo!(),
            OpCodes::POP => self.pop(stack),
            OpCodes::PUSH0 => todo!(),
            OpCodes::PUSH1 => self.pushx(1, stack, pc),
            OpCodes::PUSH2 => self.pushx(2, stack, pc),
            OpCodes::PUSH3 => self.pushx(3, stack, pc),
            OpCodes::PUSH4 => self.pushx(4, stack, pc),
            OpCodes::PUSH5 => self.pushx(5, stack, pc),
            OpCodes::PUSH6 => self.pushx(6, stack, pc),
            OpCodes::PUSH7 => self.pushx(7, stack, pc),
            OpCodes::PUSH8 => self.pushx(8, stack, pc),
            OpCodes::PUSH9 => self.pushx(9, stack, pc),
            OpCodes::PUSH10 => self.pushx(10, stack, pc),
            OpCodes::PUSH11 => self.pushx(11, stack, pc),
            OpCodes::PUSH12 => self.pushx(12, stack, pc),
            OpCodes::PUSH13 => self.pushx(13, stack, pc),
            OpCodes::PUSH14 => self.pushx(14, stack, pc),
            OpCodes::PUSH15 => self.pushx(15, stack, pc),
            OpCodes::PUSH16 => self.pushx(16, stack, pc),
            OpCodes::PUSH17 => self.pushx(17, stack, pc),
            OpCodes::PUSH18 => self.pushx(18, stack, pc),
            OpCodes::PUSH19 => self.pushx(19, stack, pc),
            OpCodes::PUSH20 => self.pushx(20, stack, pc),
            OpCodes::PUSH21 => self.pushx(21, stack, pc),
            OpCodes::PUSH22 => self.pushx(22, stack, pc),
            OpCodes::PUSH23 => self.pushx(23, stack, pc),
            OpCodes::PUSH24 => self.pushx(24, stack, pc),
            OpCodes::PUSH25 => self.pushx(25, stack, pc),
            OpCodes::PUSH26 => self.pushx(26, stack, pc),
            OpCodes::PUSH27 => self.pushx(27, stack, pc),
            OpCodes::PUSH28 => self.pushx(28, stack, pc),
            OpCodes::PUSH29 => self.pushx(29, stack, pc),
            OpCodes::PUSH30 => self.pushx(30, stack, pc),
            OpCodes::PUSH31 => self.pushx(31, stack, pc),
            OpCodes::PUSH32 => self.pushx(32, stack, pc),
            OpCodes::RETURN => self.stop(stack),
            OpCodes::RETURNDATACOPY => todo!(),
            OpCodes::RETURNDATASIZE => todo!(),
            OpCodes::REVERT => self.stop(stack),
            OpCodes::SAR => todo!(),
            OpCodes::SDIV => todo!(),
            OpCodes::SELFBALANCE => todo!(),
            OpCodes::SELFDESTRUCT => todo!(),
            OpCodes::SGT => todo!(),
            OpCodes::SHA3 => todo!(),
            OpCodes::SHL => todo!(),
            OpCodes::SHR => self.shr(stack),
            OpCodes::SIGNEXTEND => todo!(),
            OpCodes::SLOAD => self.sload(stack, memory),
            OpCodes::SLT => self.slt(stack),
            OpCodes::SMOD => todo!(),
            OpCodes::SSTORE => todo!(),
            OpCodes::STATICCALL => todo!(),
            OpCodes::STOP => self.stop(stack),
            OpCodes::SUB => self.sub(stack),
            OpCodes::SWAP1 => self.swapx(1, stack),
            OpCodes::SWAP2 => self.swapx(2, stack),
            OpCodes::SWAP3 => self.swapx(3, stack),
            OpCodes::SWAP4 => self.swapx(4, stack),
            OpCodes::SWAP5 => self.swapx(5, stack),
            OpCodes::SWAP6 => self.swapx(6, stack),
            OpCodes::SWAP7 => self.swapx(7, stack),
            OpCodes::SWAP8 => self.swapx(8, stack),
            OpCodes::SWAP9 => self.swapx(9, stack),
            OpCodes::SWAP10 => self.swapx(10, stack),
            OpCodes::SWAP11 => self.swapx(11, stack),
            OpCodes::SWAP12 => self.swapx(12, stack),
            OpCodes::SWAP13 => self.swapx(13, stack),
            OpCodes::SWAP14 => self.swapx(14, stack),
            OpCodes::SWAP15 => self.swapx(15, stack),
            OpCodes::SWAP16 => self.swapx(16, stack),
            OpCodes::TIMESTAMP => todo!(),
            OpCodes::XOR => todo!(),
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ParsedInstruction {
    pub instruction: Instruction,
    pub used_arg: Option<Hex>,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:<10?}\t args: {:?}", self.opcode, self.args)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JumpType {
    Conditional,
    Unconditional,
}

#[derive(Clone, PartialEq)]
pub struct JumpInstruction {
    pub instruction: Instruction,
    pub jump_type: JumpType,
    pub target: StackElement,
    pub source: Hex,
    pub condition: Option<StackElement>,
}
impl std::fmt::Debug for JumpInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(type: {:?}, target {:?}, source: {:04x})",
            &self.jump_type, &self.target, &self.source
        )
    }
}

#[derive(Clone)]
pub struct InstructionSet {
    pub start: Hex,
    pub end: Hex,

    pub jump: Option<JumpInstruction>,

    pub stack: Stack,
}
impl std::fmt::Debug for InstructionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "start: {:04x}", self.start).unwrap();
        writeln!(
            f,
            "start: {}, end: {}, jumps: {:?}, stack: {:?}",
            self.start, self.end, self.jump, self.stack
        )
        .unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hex::Hex,
        memory::{Memory, MemoryElement},
        opcode::{
            opcodes, OpCodeResult,
            OpCodes::{self},
        },
        stack::{Stack, StackElement},
    };

    use super::Instruction;
    use test_log::test;

    #[test]
    fn pushx_1() {
        let input = Instruction {
            args: Vec::from([Hex(0xff)]),
            opcode: opcodes().get(&OpCodes::PUSH1).unwrap().clone(),
            index: Hex(0),
        };
        let mut stack = Stack::new();
        let mut memory = Memory::new();
        let mut pc = Hex(0);
        match input.parse(&mut stack, &mut pc, &mut memory) {
            Ok(v) => assert!(v == OpCodeResult::Ok),
            Err(_) => panic!("parse returned error where none was expected"),
        }
        assert!(stack.len() == 1);
        let elem = stack.pop().unwrap();
        assert_eq!(elem.value, Hex(0xff));
        assert_eq!(elem.size, 1);
        assert_eq!(elem.origin, Hex(0));
        assert_eq!(pc, Hex(1));
    }

    #[test]
    fn pushx_5() {
        let input = Instruction {
            args: Vec::from([Hex(0xff), Hex(0xee), Hex(0xdd), Hex(0xcc), Hex(0xbb)]),
            opcode: opcodes().get(&OpCodes::PUSH5).unwrap().clone(),
            index: Hex(0),
        };
        let mut stack = Stack::new();
        let mut memory = Memory::new();
        let mut pc = Hex(0);
        match input.parse(&mut stack, &mut pc, &mut memory) {
            Ok(v) => assert!(v == OpCodeResult::Ok),
            Err(_) => panic!("parse returned error where none was expected"),
        }
        assert!(stack.len() == 1);
        let elem = stack.pop().unwrap();
        assert_eq!(elem.value, Hex(0xffeeddccbb));
        assert_eq!(elem.size, 5);
        assert_eq!(elem.origin, Hex(0));
        assert_eq!(pc, Hex(5));
    }
    #[test]
    fn mstore_basic() {
        let input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::MSTORE).unwrap().clone(),
            index: Hex(2),
        };
        let mut stack = Stack::new();
        stack.push(StackElement {
            value: Hex(0xabcd),
            origin: Hex(0),
            size: 2,
        });
        stack.push(StackElement {
            value: Hex(0),
            origin: Hex(1),
            size: 1,
        });
        let mut memory = Memory::new();
        let mut pc = Hex(0);
        match input.parse(&mut stack, &mut pc, &mut memory) {
            Ok(v) => assert!(v == OpCodeResult::Ok),
            Err(_) => panic!("parse returned error where none was expected"),
        }
        assert!(stack.len() == 0);
        assert_eq!(memory.get_contents().len(), 32);
        assert_eq!(
            memory.get_contents().get(30),
            Some(&MemoryElement {
                value: Hex(0xab),
                origin: Some(Hex(0))
            })
        );
        assert_eq!(
            memory.get_contents().get(31),
            Some(&MemoryElement {
                value: Hex(0xcd),
                origin: Some(Hex(0))
            })
        );
    }
    #[test]
    fn multiple_mstores() {
        let first_input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::MSTORE).unwrap().clone(),
            index: Hex(2),
        };
        let second_input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::MSTORE).unwrap().clone(),
            index: Hex(5),
        };
        let mut stack = Stack::new();
        stack.push(StackElement {
            value: Hex(0x1234),
            origin: Hex(4),
            size: 2,
        });
        stack.push(StackElement {
            value: Hex(1),
            origin: Hex(3),
            size: 1,
        });
        stack.push(StackElement {
            value: Hex(0xabcd),
            origin: Hex(0),
            size: 2,
        });
        stack.push(StackElement {
            value: Hex(0),
            origin: Hex(1),
            size: 1,
        });
        let mut memory = Memory::new();
        let mut pc = Hex(0);
        match first_input.parse(&mut stack, &mut pc, &mut memory) {
            Ok(v) => assert!(v == OpCodeResult::Ok),
            Err(_) => panic!("parse returned error where none was expected"),
        }
        assert_eq!(
            memory.get_contents().get(31),
            Some(&MemoryElement {
                value: Hex(0xcd),
                origin: Some(Hex(0)),
            })
        );
        match second_input.parse(&mut stack, &mut pc, &mut memory) {
            Ok(v) => assert!(v == OpCodeResult::Ok),
            Err(_) => panic!("parse returned error where none was expected"),
        }
        assert!(stack.len() == 0);
        assert_eq!(memory.get_contents().len(), 64);
        assert_eq!(
            memory.get_contents().get(0),
            Some(&MemoryElement {
                value: Hex(0),
                origin: Some(Hex(2)),
            })
        );
        assert_eq!(
            memory.get_contents().get(31),
            Some(&MemoryElement {
                value: Hex(0x12),
                origin: Some(Hex(4)),
            })
        );
        assert_eq!(
            memory.get_contents().get(32),
            Some(&MemoryElement {
                value: Hex(0x34),
                origin: Some(Hex(4))
            })
        );
    }

    #[test]
    fn dup1() {
        let mut stack = Stack::new();
        stack.push(StackElement {
            value: Hex(1),
            origin: Hex(0),
            size: 1,
        });
        let input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::DUP1).unwrap().clone(),
            index: Hex(2),
        };
        let mut pc = Hex(0);
        let mut memory = Memory::new();
        input.parse(&mut stack, &mut pc, &mut memory).unwrap();
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.get(0), stack.get(1));
    }

    #[test]
    #[should_panic]
    fn dup1_empty_stack() {
        let mut stack = Stack::new();
        let input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::DUP1).unwrap().clone(),
            index: Hex(2),
        };
        let mut pc = Hex(0);
        let mut memory = Memory::new();
        assert!(input.parse(&mut stack, &mut pc, &mut memory).is_err());
    }
    #[test]
    fn dup2() {
        let mut stack = Stack::new();
        stack.push(StackElement {
            value: Hex(2),
            origin: Hex(1),
            size: 1,
        });
        stack.push(StackElement {
            value: Hex(1),
            origin: Hex(0),
            size: 1,
        });
        let input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::DUP2).unwrap().clone(),
            index: Hex(2),
        };
        let mut pc = Hex(0);
        let mut memory = Memory::new();
        input.parse(&mut stack, &mut pc, &mut memory).unwrap();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.get(0), stack.get(2));
    }
    #[test]
    fn shr() {
        let mut stack = Stack::new();
        stack.push(StackElement {
            value: Hex(0xf),
            origin: Hex(1),
            size: 1,
        });
        stack.push(StackElement {
            value: Hex(1),
            origin: Hex(0),
            size: 1,
        });
        let input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::SHR).unwrap().clone(),
            index: Hex(2),
        };
        let mut pc = Hex(0);
        let mut memory = Memory::new();
        input.parse(&mut stack, &mut pc, &mut memory).unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.get(0).unwrap().value, Hex(0x7));
    }
    #[test]
    fn shr_overflow() {
        let mut stack = Stack::new();
        stack.push(StackElement {
            value: Hex(0),
            origin: Hex(0),
            size: 1,
        });
        stack.push(StackElement {
            value: Hex(0xff),
            origin: Hex(0x1),
            size: 1,
        });
        let input = Instruction {
            args: Vec::new(),
            opcode: opcodes().get(&OpCodes::SHR).unwrap().clone(),
            index: Hex(2),
        };
        let mut pc = Hex(0);
        let mut memory = Memory::new();
        input.parse(&mut stack, &mut pc, &mut memory).unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.get(0).unwrap().value, Hex(0x0));
    }
}
