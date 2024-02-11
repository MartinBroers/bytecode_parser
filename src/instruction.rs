use core::fmt;
use log::{info, warn};

use crate::{
    hex::Hex,
    memory::Memory,
    opcode::{OpCode, OpCodeResult, OpCodes},
    stack::{Stack, StackElement},
    CALLVALUE,
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
    fn swapx(&self, num_swap: u32, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let mut swaps = Vec::new();
        for _ in 0..=num_swap {
            swaps.push(match stack.pop() {
                Some(v) => v,
                None => return Err(()),
            });
        }
        swaps.reverse();
        for _ in 0..=num_swap {
            stack.push(swaps.pop().unwrap());
        }
        Ok(OpCodeResult::Ok)
    }
    fn pushx(&self, num_push: usize, stack: &mut Stack, pc: &mut Hex) -> Result<OpCodeResult, ()> {
        *pc += Hex(1);
        assert!(self.args.len() == num_push);
        warn!("pushx: num_push: {:?}, self: {:?}", num_push, self);
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
    fn jumpdest(&self) -> Result<OpCodeResult, ()> {
        Ok(OpCodeResult::Ok)
    }
    fn jumpi(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let target = stack.pop();
        let condition = stack.pop();
        println!("jumpi: target: {:?}, condition: {:?}", target, condition);
        let jump_instruction = JumpInstruction {
            instruction: self.clone(),
            jump_type: JumpType::Conditional,
            target,
            condition,
            source: self.index,
        };

        Ok(OpCodeResult::ConditionalJumpInstruction(jump_instruction))
    }
    fn jump(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let jump_instruction = JumpInstruction {
            instruction: self.clone(),
            jump_type: JumpType::Unconditional,
            target: stack.pop(),
            condition: None,
            source: self.index,
        };

        Ok(OpCodeResult::JumpInstruction(jump_instruction))
    }

    fn mstore(&self, stack: &mut Stack, memory: &mut Memory) -> Result<OpCodeResult, ()> {
        let offset = stack.pop().ok_or(())?;
        let value = stack.pop().ok_or(())?;
        memory.mstore(value, offset.value, self.index);
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
            OpCodes::CALLDATALOAD => todo!(),
            OpCodes::CALLDATASIZE => todo!(),
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
            OpCodes::DUP1 => todo!(),
            OpCodes::DUP2 => todo!(),
            OpCodes::DUP3 => todo!(),
            OpCodes::DUP4 => todo!(),
            OpCodes::DUP5 => todo!(),
            OpCodes::DUP6 => todo!(),
            OpCodes::DUP7 => todo!(),
            OpCodes::DUP8 => todo!(),
            OpCodes::DUP9 => todo!(),
            OpCodes::DUP10 => todo!(),
            OpCodes::DUP11 => todo!(),
            OpCodes::DUP12 => todo!(),
            OpCodes::DUP13 => todo!(),
            OpCodes::DUP14 => todo!(),
            OpCodes::DUP15 => todo!(),
            OpCodes::DUP16 => todo!(),
            OpCodes::EOFMAGIC => todo!(),
            OpCodes::EQ => todo!(),
            OpCodes::EXP => todo!(),
            OpCodes::EXTCODECOPY => todo!(),
            OpCodes::EXTCODEHASH => todo!(),
            OpCodes::EXTCODESIZE => todo!(),
            OpCodes::GAS => todo!(),
            OpCodes::GASLIMIT => todo!(),
            OpCodes::GASPRICE => todo!(),
            OpCodes::GT => todo!(),
            OpCodes::INVALID => todo!(),
            OpCodes::ISZERO => todo!(),
            OpCodes::JUMP => self.jump(stack),
            OpCodes::JUMPDEST => self.jumpdest(),
            OpCodes::JUMPI => self.jumpi(stack),
            OpCodes::LOG0 => todo!(),
            OpCodes::LOG1 => todo!(),
            OpCodes::LOG2 => todo!(),
            OpCodes::LOG3 => todo!(),
            OpCodes::LOG4 => todo!(),
            OpCodes::LT => todo!(),
            OpCodes::MLOAD => todo!(),
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
            OpCodes::PUSH2 => todo!(),
            OpCodes::PUSH3 => todo!(),
            OpCodes::PUSH4 => todo!(),
            OpCodes::PUSH5 => self.pushx(5, stack, pc),
            OpCodes::PUSH6 => todo!(),
            OpCodes::PUSH7 => todo!(),
            OpCodes::PUSH8 => todo!(),
            OpCodes::PUSH9 => todo!(),
            OpCodes::PUSH10 => todo!(),
            OpCodes::PUSH11 => todo!(),
            OpCodes::PUSH12 => todo!(),
            OpCodes::PUSH13 => todo!(),
            OpCodes::PUSH14 => todo!(),
            OpCodes::PUSH15 => todo!(),
            OpCodes::PUSH16 => todo!(),
            OpCodes::PUSH17 => todo!(),
            OpCodes::PUSH18 => todo!(),
            OpCodes::PUSH19 => todo!(),
            OpCodes::PUSH20 => todo!(),
            OpCodes::PUSH21 => todo!(),
            OpCodes::PUSH22 => todo!(),
            OpCodes::PUSH23 => todo!(),
            OpCodes::PUSH24 => todo!(),
            OpCodes::PUSH25 => todo!(),
            OpCodes::PUSH26 => todo!(),
            OpCodes::PUSH27 => todo!(),
            OpCodes::PUSH28 => todo!(),
            OpCodes::PUSH29 => todo!(),
            OpCodes::PUSH30 => todo!(),
            OpCodes::PUSH31 => todo!(),
            OpCodes::PUSH32 => todo!(),
            OpCodes::RETURN => todo!(),
            OpCodes::RETURNDATACOPY => todo!(),
            OpCodes::RETURNDATASIZE => todo!(),
            OpCodes::REVERT => todo!(),
            OpCodes::SAR => todo!(),
            OpCodes::SDIV => todo!(),
            OpCodes::SELFBALANCE => todo!(),
            OpCodes::SELFDESTRUCT => todo!(),
            OpCodes::SGT => todo!(),
            OpCodes::SHA3 => todo!(),
            OpCodes::SHL => todo!(),
            OpCodes::SHR => todo!(),
            OpCodes::SIGNEXTEND => todo!(),
            OpCodes::SLOAD => todo!(),
            OpCodes::SLT => todo!(),
            OpCodes::SMOD => todo!(),
            OpCodes::SSTORE => todo!(),
            OpCodes::STATICCALL => todo!(),
            OpCodes::STOP => self.stop(stack),
            OpCodes::SUB => todo!(),
            OpCodes::SWAP1 => self.swapx(1, stack),
            OpCodes::SWAP2 => todo!(),
            OpCodes::SWAP3 => todo!(),
            OpCodes::SWAP4 => todo!(),
            OpCodes::SWAP5 => todo!(),
            OpCodes::SWAP6 => todo!(),
            OpCodes::SWAP7 => todo!(),
            OpCodes::SWAP8 => todo!(),
            OpCodes::SWAP9 => todo!(),
            OpCodes::SWAP10 => todo!(),
            OpCodes::SWAP11 => todo!(),
            OpCodes::SWAP12 => todo!(),
            OpCodes::SWAP13 => todo!(),
            OpCodes::SWAP14 => todo!(),
            OpCodes::SWAP15 => todo!(),
            OpCodes::SWAP16 => todo!(),
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

impl ParsedInstruction {
    pub fn new(instruction: Instruction, stack: Stack) -> ParsedInstruction {
        ParsedInstruction {
            instruction,
            used_arg: None,
        }
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
    pub target: Option<StackElement>,
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
    pub instructions: Vec<ParsedInstruction>,
    pub start: Hex,
    pub end: Hex,

    pub jumps: Vec<JumpInstruction>,

    pub stack: Stack,
}
impl std::fmt::Debug for InstructionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "start: {:04x}", self.start).unwrap();
        for instruction in &self.instructions {
            writeln!(
                f,
                "{:04x}\t{:?}\t{:?}\t{:?}",
                instruction.instruction.index,
                instruction.instruction.opcode,
                instruction.used_arg,
                instruction.instruction.args,
            )
            .unwrap();
        }
        writeln!(
            f,
            "start: {}, end: {}, jumps: {:?}, stack: {:?}",
            self.start, self.end, self.jumps, self.stack
        )
        .unwrap();
        Ok(())
    }
}

impl InstructionSet {
    pub fn push(&mut self, value: ParsedInstruction) {
        self.instructions.push(value);
    }
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        hex::Hex,
        memory::{Memory, MemoryElement},
        opcode::{
            opcodes, OpCode, OpCodeResult,
            OpCodes::{self, PUSH1},
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
        assert_eq!(pc, Hex(1));
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
}
