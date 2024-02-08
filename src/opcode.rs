use core::fmt;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use std::collections::HashMap;

use crate::{
    instruction::{Hex, Instruction, JumpInstruction, JumpType},
    stack::{Stack, StackElement},
};

pub enum OpCodeResult {
    JumpInstruction(JumpInstruction),
    ConditionalJumpInstruction(JumpInstruction),
    End,
    Ok,
}

trait Parse {
    fn parse(stack: Vec<Hex>, instruction_pointer: usize) -> (Vec<Hex>, usize);
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct OpCode {
    pub code: OpCodes,
    pub input_arguments: u32,

    // SWAP1 is 1, SWAP5 is 5 etc
    pub operator_index: usize,

    pub stack_inputs: u32,
    pub stack_outputs: u32,
    pub short_name: String,
    //pub parse: fn(
    //    bytecode: OpCodes:: &HashMap<Hex, Instruction>,
    //    instruction: &Instruction,
    //    pc: &mut Hex,
    //    stack: &mut Vec<Hex>,
    //    memory: &mut Vec<Hex>,
    //) -> Result<ParsedInstruction, Error>,
}

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(0x{:02x})", self.short_name, self.code)
    }
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
        });
        Ok(OpCodeResult::Ok)
    }
    fn mul(&self, stack: &mut Stack) -> Result<OpCodeResult, ()> {
        let left = stack.pop().ok_or(())?;
        let right = stack.pop().ok_or(())?;
        stack.push(StackElement {
            value: left.value * right.value,
            origin: self.index,
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
    fn pushx(
        &self,
        num_push: usize,
        stack: &mut Stack,
        pc: &mut Hex,
        input_arguments: &Vec<Hex>,
    ) -> Result<OpCodeResult, ()> {
        *pc += Hex(num_push.try_into().unwrap());
        assert!(input_arguments.len() == num_push);
        for element in input_arguments {
            stack.push(StackElement {
                value: *element,
                origin: self.index,
            });
        }
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

    // Parses the opcode and returns the stack
    pub fn parse(
        &self,
        stack: &mut Stack,
        pc: &mut Hex,
        input_arguments: &Vec<Hex>,
    ) -> Result<OpCodeResult, ()> {
        match self.opcode.code {
            OpCodes::STOP => self.stop(stack),
            OpCodes::ADD => self.add(stack),
            OpCodes::MUL => self.mul(stack),
            OpCodes::SUB => todo!(),
            OpCodes::DIV => todo!(),
            OpCodes::SDIV => todo!(),
            OpCodes::MOD => todo!(),
            OpCodes::SMOD => todo!(),
            OpCodes::ADDMOD => todo!(),
            OpCodes::MULMOD => todo!(),
            OpCodes::EXP => todo!(),
            OpCodes::SIGNEXTEND => todo!(),
            OpCodes::LT => todo!(),
            OpCodes::GT => todo!(),
            OpCodes::SLT => todo!(),
            OpCodes::SGT => todo!(),
            OpCodes::EQ => todo!(),
            OpCodes::ISZERO => todo!(),
            OpCodes::AND => todo!(),
            OpCodes::OR => todo!(),
            OpCodes::XOR => todo!(),
            OpCodes::NOT => todo!(),
            OpCodes::BYTE => todo!(),
            OpCodes::CALLDATALOAD => todo!(),
            OpCodes::CALLDATASIZE => todo!(),
            OpCodes::CALLDATACOPY => todo!(),
            OpCodes::CODESIZE => todo!(),
            OpCodes::CODECOPY => todo!(),
            OpCodes::SHL => todo!(),
            OpCodes::SHR => todo!(),
            OpCodes::SAR => todo!(),
            OpCodes::POP => self.pop(stack),
            OpCodes::MLOAD => todo!(),
            OpCodes::MSTORE => todo!(),
            OpCodes::MSTORE8 => todo!(),
            OpCodes::JUMP => self.jump(stack),
            OpCodes::JUMPI => self.jumpi(stack),
            OpCodes::PC => todo!(),
            OpCodes::MSIZE => todo!(),
            OpCodes::JUMPDEST => self.jumpdest(),
            OpCodes::PUSH0 => todo!(),
            OpCodes::PUSH1 => self.pushx(1, stack, pc, input_arguments),
            OpCodes::PUSH2 => todo!(),
            OpCodes::PUSH3 => todo!(),
            OpCodes::PUSH4 => todo!(),
            OpCodes::PUSH5 => todo!(),
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
            OpCodes::RETURN => todo!(),
            OpCodes::REVERT => todo!(),
            OpCodes::INVALID => todo!(),
            OpCodes::EOFMAGIC => todo!(),
            OpCodes::SHA3 => todo!(),
            OpCodes::ADDRESS => todo!(),
            OpCodes::BALANCE => todo!(),
            OpCodes::SELFBALANCE => todo!(),
            OpCodes::BASEFEE => todo!(),
            OpCodes::ORIGIN => todo!(),
            OpCodes::CALLER => todo!(),
            OpCodes::CALLVALUE => todo!(),
            OpCodes::GASPRICE => todo!(),
            OpCodes::EXTCODESIZE => todo!(),
            OpCodes::EXTCODECOPY => todo!(),
            OpCodes::EXTCODEHASH => todo!(),
            OpCodes::RETURNDATASIZE => todo!(),
            OpCodes::RETURNDATACOPY => todo!(),
            OpCodes::BLOCKHASH => todo!(),
            OpCodes::COINBASE => todo!(),
            OpCodes::TIMESTAMP => todo!(),
            OpCodes::NUMBER => todo!(),
            OpCodes::DIFFICULTY => todo!(),
            OpCodes::GASLIMIT => todo!(),
            OpCodes::SLOAD => todo!(),
            OpCodes::SSTORE => todo!(),
            OpCodes::GAS => todo!(),
            OpCodes::LOG0 => todo!(),
            OpCodes::LOG1 => todo!(),
            OpCodes::LOG2 => todo!(),
            OpCodes::LOG3 => todo!(),
            OpCodes::LOG4 => todo!(),
            OpCodes::CREATE => todo!(),
            OpCodes::CALL => todo!(),
            OpCodes::CALLCODE => todo!(),
            OpCodes::DELEGATECALL => todo!(),
            OpCodes::CREATE2 => todo!(),
            OpCodes::STATICCALL => todo!(),
            OpCodes::SELFDESTRUCT => todo!(),
            OpCodes::CHAINID => todo!(),
        }
    }
}

impl Default for OpCode {
    fn default() -> Self {
        Self {
            code: OpCodes::INVALID,
            input_arguments: Default::default(),
            operator_index: Default::default(),
            stack_inputs: Default::default(),
            stack_outputs: Default::default(),
            short_name: Default::default(),
        }
    }
}

pub fn opcodes() -> HashMap<OpCodes, OpCode> {
    let mut map = HashMap::new();
    map.insert(
        OpCodes::STOP,
        OpCode {
            code: OpCodes::STOP,
            short_name: "STOP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::ADD,
        OpCode {
            code: OpCodes::ADD,
            short_name: "ADD".to_string(),
            stack_inputs: 2,
            stack_outputs: 1,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MUL,
        OpCode {
            code: OpCodes::MUL,
            short_name: "MUL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SUB,
        OpCode {
            code: OpCodes::SUB,
            short_name: "SUB".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DIV,
        OpCode {
            code: OpCodes::DIV,
            short_name: "DIV".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SDIV,
        OpCode {
            code: OpCodes::SDIV,
            short_name: "SDIV".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MOD,
        OpCode {
            code: OpCodes::MOD,
            short_name: "MOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SMOD,
        OpCode {
            code: OpCodes::SMOD,
            short_name: "SMOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::ADDMOD,
        OpCode {
            code: OpCodes::ADDMOD,
            short_name: "ADDMOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MULMOD,
        OpCode {
            code: OpCodes::MULMOD,
            short_name: "MULMOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::EXP,
        OpCode {
            code: OpCodes::EXP,
            short_name: "EXP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SIGNEXTEND,
        OpCode {
            code: OpCodes::SIGNEXTEND,
            short_name: "SIGNEXTEND".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::LT,
        OpCode {
            code: OpCodes::LT,
            short_name: "LT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::GT,
        OpCode {
            code: OpCodes::GT,
            short_name: "GT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SLT,
        OpCode {
            code: OpCodes::SLT,
            short_name: "SLT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SGT,
        OpCode {
            code: OpCodes::SGT,
            short_name: "SGT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::EQ,
        OpCode {
            code: OpCodes::EQ,
            short_name: "EQ".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::ISZERO,
        OpCode {
            code: OpCodes::ISZERO,
            short_name: "ISZERO".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::AND,
        OpCode {
            code: OpCodes::AND,
            short_name: "AND".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::OR,
        OpCode {
            code: OpCodes::OR,
            short_name: "OR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::XOR,
        OpCode {
            code: OpCodes::XOR,
            short_name: "XOR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::NOT,
        OpCode {
            code: OpCodes::NOT,
            short_name: "NOT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::BYTE,
        OpCode {
            code: OpCodes::BYTE,
            short_name: "BYTE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SHL,
        OpCode {
            code: OpCodes::SHL,
            short_name: "SHL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SHR,
        OpCode {
            code: OpCodes::SHR,
            short_name: "SHR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SAR,
        OpCode {
            code: OpCodes::SAR,
            short_name: "SAR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SHA3,
        OpCode {
            code: OpCodes::SHA3,
            short_name: "SHA3".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::ADDRESS,
        OpCode {
            code: OpCodes::ADDRESS,
            short_name: "ADDRESS".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::BALANCE,
        OpCode {
            code: OpCodes::BALANCE,
            short_name: "BALANCE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::ORIGIN,
        OpCode {
            code: OpCodes::ORIGIN,
            short_name: "ORIGIN".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALLER,
        OpCode {
            code: OpCodes::CALLER,
            short_name: "CALLER".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALLVALUE,
        OpCode {
            code: OpCodes::CALLVALUE,
            short_name: "CALLVALUE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALLDATALOAD,
        OpCode {
            code: OpCodes::CALLDATALOAD,
            short_name: "CALLDATALOAD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALLDATASIZE,
        OpCode {
            code: OpCodes::CALLDATASIZE,
            short_name: "CALLDATASIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALLDATACOPY,
        OpCode {
            code: OpCodes::CALLDATACOPY,
            short_name: "CALLDATACOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CODESIZE,
        OpCode {
            code: OpCodes::CODESIZE,
            short_name: "CODESIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CODECOPY,
        OpCode {
            code: OpCodes::CODECOPY,
            short_name: "CODECOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::GASPRICE,
        OpCode {
            code: OpCodes::GASPRICE,
            short_name: "GASPRICE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::EXTCODESIZE,
        OpCode {
            code: OpCodes::EXTCODESIZE,
            short_name: "EXTCODESIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::EXTCODECOPY,
        OpCode {
            code: OpCodes::EXTCODECOPY,
            short_name: "EXTCODECOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::RETURNDATASIZE,
        OpCode {
            code: OpCodes::RETURNDATASIZE,
            short_name: "RETURNDATASIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::RETURNDATACOPY,
        OpCode {
            code: OpCodes::RETURNDATACOPY,
            short_name: "RETURNDATACOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::EXTCODEHASH,
        OpCode {
            code: OpCodes::EXTCODEHASH,
            short_name: "EXTCODEHASH".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::BLOCKHASH,
        OpCode {
            code: OpCodes::BLOCKHASH,
            short_name: "BLOCKHASH".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::COINBASE,
        OpCode {
            code: OpCodes::COINBASE,
            short_name: "COINBASE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::TIMESTAMP,
        OpCode {
            code: OpCodes::TIMESTAMP,
            short_name: "TIMESTAMP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::NUMBER,
        OpCode {
            code: OpCodes::NUMBER,
            short_name: "NUMBER".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DIFFICULTY,
        OpCode {
            code: OpCodes::DIFFICULTY,
            short_name: "DIFFICULTY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::GASLIMIT,
        OpCode {
            code: OpCodes::GASLIMIT,
            short_name: "GASLIMIT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CHAINID,
        OpCode {
            code: OpCodes::CHAINID,
            short_name: "CHAINID".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SELFBALANCE,
        OpCode {
            code: OpCodes::SELFBALANCE,
            short_name: "SELFBALANCE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::BASEFEE,
        OpCode {
            code: OpCodes::BASEFEE,
            short_name: "BASEFEE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::POP,
        OpCode {
            code: OpCodes::POP,
            short_name: "POP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MLOAD,
        OpCode {
            code: OpCodes::MLOAD,
            short_name: "MLOAD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MSTORE,
        OpCode {
            code: OpCodes::MSTORE,
            short_name: "MSTORE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MSTORE8,
        OpCode {
            code: OpCodes::MSTORE8,
            short_name: "MSTORE8".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SLOAD,
        OpCode {
            code: OpCodes::SLOAD,
            short_name: "SLOAD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SSTORE,
        OpCode {
            code: OpCodes::SSTORE,
            short_name: "SSTORE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::JUMP,
        OpCode {
            code: OpCodes::JUMP,
            short_name: "JUMP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::JUMPI,
        OpCode {
            code: OpCodes::JUMPI,
            short_name: "JUMPI".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PC,
        OpCode {
            code: OpCodes::PC,
            short_name: "PC".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::MSIZE,
        OpCode {
            code: OpCodes::MSIZE,
            short_name: "MSIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::GAS,
        OpCode {
            code: OpCodes::GAS,
            short_name: "GAS".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::JUMPDEST,
        OpCode {
            code: OpCodes::JUMPDEST,
            short_name: "JUMPDEST".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH1,
        OpCode {
            code: OpCodes::PUSH1,
            short_name: "PUSH1".to_string(),
            input_arguments: 1,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH2,
        OpCode {
            code: OpCodes::PUSH2,
            short_name: "PUSH2".to_string(),
            input_arguments: 2,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH3,
        OpCode {
            code: OpCodes::PUSH3,
            short_name: "PUSH3".to_string(),
            input_arguments: 3,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH4,
        OpCode {
            code: OpCodes::PUSH4,
            short_name: "PUSH4".to_string(),
            input_arguments: 4,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH5,
        OpCode {
            code: OpCodes::PUSH5,
            short_name: "PUSH5".to_string(),
            input_arguments: 5,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH6,
        OpCode {
            code: OpCodes::PUSH6,
            short_name: "PUSH6".to_string(),
            input_arguments: 6,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH7,
        OpCode {
            code: OpCodes::PUSH7,
            short_name: "PUSH7".to_string(),
            input_arguments: 7,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH8,
        OpCode {
            code: OpCodes::PUSH8,
            short_name: "PUSH8".to_string(),
            input_arguments: 8,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH9,
        OpCode {
            code: OpCodes::PUSH9,
            short_name: "PUSH9".to_string(),
            input_arguments: 9,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH10,
        OpCode {
            code: OpCodes::PUSH10,
            short_name: "PUSH10".to_string(),
            input_arguments: 10,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH11,
        OpCode {
            code: OpCodes::PUSH11,
            short_name: "PUSH11".to_string(),
            input_arguments: 11,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH12,
        OpCode {
            code: OpCodes::PUSH12,
            short_name: "PUSH12".to_string(),
            input_arguments: 12,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH13,
        OpCode {
            code: OpCodes::PUSH13,
            short_name: "PUSH13".to_string(),
            input_arguments: 13,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH14,
        OpCode {
            code: OpCodes::PUSH14,
            short_name: "PUSH14".to_string(),
            input_arguments: 14,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH15,
        OpCode {
            code: OpCodes::PUSH15,
            short_name: "PUSH15".to_string(),
            input_arguments: 15,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH16,
        OpCode {
            code: OpCodes::PUSH16,
            short_name: "PUSH16".to_string(),
            input_arguments: 16,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH17,
        OpCode {
            code: OpCodes::PUSH17,
            short_name: "PUSH17".to_string(),
            input_arguments: 17,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH18,
        OpCode {
            code: OpCodes::PUSH18,
            short_name: "PUSH18".to_string(),
            input_arguments: 18,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH19,
        OpCode {
            code: OpCodes::PUSH19,
            short_name: "PUSH19".to_string(),
            input_arguments: 19,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH20,
        OpCode {
            code: OpCodes::PUSH20,
            short_name: "PUSH20".to_string(),
            input_arguments: 20,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH21,
        OpCode {
            code: OpCodes::PUSH21,
            short_name: "PUSH21".to_string(),
            input_arguments: 21,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH22,
        OpCode {
            code: OpCodes::PUSH22,
            short_name: "PUSH22".to_string(),
            input_arguments: 22,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH23,
        OpCode {
            code: OpCodes::PUSH23,
            short_name: "PUSH23".to_string(),
            input_arguments: 23,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH24,
        OpCode {
            code: OpCodes::PUSH24,
            short_name: "PUSH24".to_string(),
            input_arguments: 24,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH25,
        OpCode {
            code: OpCodes::PUSH25,
            short_name: "PUSH25".to_string(),
            input_arguments: 25,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH26,
        OpCode {
            code: OpCodes::PUSH26,
            short_name: "PUSH26".to_string(),
            input_arguments: 26,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH27,
        OpCode {
            code: OpCodes::PUSH27,
            short_name: "PUSH27".to_string(),
            input_arguments: 27,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH28,
        OpCode {
            code: OpCodes::PUSH28,
            short_name: "PUSH28".to_string(),
            input_arguments: 28,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH29,
        OpCode {
            code: OpCodes::PUSH29,
            short_name: "PUSH29".to_string(),
            input_arguments: 29,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH30,
        OpCode {
            code: OpCodes::PUSH30,
            short_name: "PUSH30".to_string(),
            input_arguments: 30,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH31,
        OpCode {
            code: OpCodes::PUSH31,
            short_name: "PUSH31".to_string(),
            input_arguments: 31,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::PUSH32,
        OpCode {
            code: OpCodes::PUSH32,
            short_name: "PUSH32".to_string(),
            input_arguments: 32,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP1,
        OpCode {
            code: OpCodes::DUP1,
            short_name: "DUP1".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP2,
        OpCode {
            code: OpCodes::DUP2,
            short_name: "DUP2".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP3,
        OpCode {
            code: OpCodes::DUP3,
            short_name: "DUP3".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP4,
        OpCode {
            code: OpCodes::DUP4,
            short_name: "DUP4".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP5,
        OpCode {
            code: OpCodes::DUP5,
            short_name: "DUP5".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP6,
        OpCode {
            code: OpCodes::DUP6,
            short_name: "DUP6".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP7,
        OpCode {
            code: OpCodes::DUP7,
            short_name: "DUP7".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP8,
        OpCode {
            code: OpCodes::DUP8,
            short_name: "DUP8".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP9,
        OpCode {
            code: OpCodes::DUP9,
            short_name: "DUP9".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP10,
        OpCode {
            code: OpCodes::DUP10,
            short_name: "DUP10".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP11,
        OpCode {
            code: OpCodes::DUP11,
            short_name: "DUP11".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP12,
        OpCode {
            code: OpCodes::DUP12,
            short_name: "DUP12".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP13,
        OpCode {
            code: OpCodes::DUP13,
            short_name: "DUP13".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP14,
        OpCode {
            code: OpCodes::DUP14,
            short_name: "DUP14".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP15,
        OpCode {
            code: OpCodes::DUP15,
            short_name: "DUP15".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DUP16,
        OpCode {
            code: OpCodes::DUP16,
            short_name: "DUP16".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP1,
        OpCode {
            code: OpCodes::SWAP1,
            short_name: "SWAP1".to_string(),
            operator_index: 1,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP2,
        OpCode {
            code: OpCodes::SWAP2,
            short_name: "SWAP2".to_string(),
            operator_index: 2,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP3,
        OpCode {
            code: OpCodes::SWAP3,
            short_name: "SWAP3".to_string(),
            operator_index: 3,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP4,
        OpCode {
            code: OpCodes::SWAP4,
            short_name: "SWAP4".to_string(),
            operator_index: 4,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP5,
        OpCode {
            code: OpCodes::SWAP5,
            short_name: "SWAP5".to_string(),
            operator_index: 5,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP6,
        OpCode {
            code: OpCodes::SWAP6,
            short_name: "SWAP6".to_string(),
            operator_index: 6,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP7,
        OpCode {
            code: OpCodes::SWAP7,
            short_name: "SWAP7".to_string(),
            operator_index: 7,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP8,
        OpCode {
            code: OpCodes::SWAP8,
            short_name: "SWAP8".to_string(),
            operator_index: 8,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP9,
        OpCode {
            code: OpCodes::SWAP9,
            short_name: "SWAP9".to_string(),
            operator_index: 9,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP10,
        OpCode {
            code: OpCodes::SWAP10,
            short_name: "SWAP10".to_string(),
            operator_index: 10,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP11,
        OpCode {
            code: OpCodes::SWAP11,
            short_name: "SWAP11".to_string(),
            operator_index: 11,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP12,
        OpCode {
            code: OpCodes::SWAP12,
            short_name: "SWAP12".to_string(),
            operator_index: 12,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP13,
        OpCode {
            code: OpCodes::SWAP13,
            short_name: "SWAP13".to_string(),
            operator_index: 13,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP14,
        OpCode {
            code: OpCodes::SWAP14,
            short_name: "SWAP14".to_string(),
            operator_index: 14,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP15,
        OpCode {
            code: OpCodes::SWAP15,
            short_name: "SWAP15".to_string(),
            operator_index: 15,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::SWAP16,
        OpCode {
            code: OpCodes::SWAP16,
            short_name: "SWAP16".to_string(),
            operator_index: 16,
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::LOG0,
        OpCode {
            code: OpCodes::LOG0,
            short_name: "LOG0".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::LOG1,
        OpCode {
            code: OpCodes::LOG1,
            short_name: "LOG1".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::LOG2,
        OpCode {
            code: OpCodes::LOG2,
            short_name: "LOG2".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::LOG3,
        OpCode {
            code: OpCodes::LOG3,
            short_name: "LOG3".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::LOG4,
        OpCode {
            code: OpCodes::LOG4,
            short_name: "LOG4".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CREATE,
        OpCode {
            code: OpCodes::CREATE,
            short_name: "CREATE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALL,
        OpCode {
            code: OpCodes::CALL,
            short_name: "CALL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CALLCODE,
        OpCode {
            code: OpCodes::CALLCODE,
            short_name: "CALLCODE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::RETURN,
        OpCode {
            code: OpCodes::RETURN,
            short_name: "RETURN".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::DELEGATECALL,
        OpCode {
            code: OpCodes::DELEGATECALL,
            short_name: "DELEGATECALL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::CREATE2,
        OpCode {
            code: OpCodes::CREATE2,
            short_name: "CALLBLACKBOX".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::STATICCALL,
        OpCode {
            code: OpCodes::STATICCALL,
            short_name: "STATICCALL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::REVERT,
        OpCode {
            code: OpCodes::REVERT,
            short_name: "REVERT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::INVALID,
        OpCode {
            code: OpCodes::INVALID,
            short_name: "INVALID".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OpCodes::EOFMAGIC,
        OpCode {
            code: OpCodes::EOFMAGIC,
            short_name: "EOFMAGIC".to_string(),
            ..Default::default()
        },
    );

    map.insert(
        OpCodes::SELFDESTRUCT,
        OpCode {
            code: OpCodes::SELFDESTRUCT,
            short_name: "SELFDESTRUCT".to_string(),
            ..Default::default()
        },
    );
    map
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
pub enum OpCodes {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0a,
    SIGNEXTEND = 0x0b,
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    SHL = 0x1b,
    SHR = 0x1c,
    SAR = 0x1d,
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    JUMPDEST = 0x5b,
    PUSH0 = 0x5f,
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6a,
    PUSH12 = 0x6b,
    PUSH13 = 0x6c,
    PUSH14 = 0x6d,
    PUSH15 = 0x6e,
    PUSH16 = 0x6f,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7a,
    PUSH28 = 0x7b,
    PUSH29 = 0x7c,
    PUSH30 = 0x7d,
    PUSH31 = 0x7e,
    PUSH32 = 0x7f,
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8a,
    DUP12 = 0x8b,
    DUP13 = 0x8c,
    DUP14 = 0x8d,
    DUP15 = 0x8e,
    DUP16 = 0x8f,
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9a,
    SWAP12 = 0x9b,
    SWAP13 = 0x9c,
    SWAP14 = 0x9d,
    SWAP15 = 0x9e,
    SWAP16 = 0x9f,
    RETURN = 0xf3,
    REVERT = 0xfd,
    INVALID = 0xfe,
    EOFMAGIC = 0xef,
    SHA3 = 0x20,
    ADDRESS = 0x30,
    BALANCE = 0x31,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    GASPRICE = 0x3a,
    EXTCODESIZE = 0x3b,
    EXTCODECOPY = 0x3c,
    EXTCODEHASH = 0x3f,
    RETURNDATASIZE = 0x3d,
    RETURNDATACOPY = 0x3e,
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    SLOAD = 0x54,
    SSTORE = 0x55,
    GAS = 0x5a,
    LOG0 = 0xa0,
    LOG1 = 0xa1,
    LOG2 = 0xa2,
    LOG3 = 0xa3,
    LOG4 = 0xa4,
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xf5,
    STATICCALL = 0xfa,
    SELFDESTRUCT = 0xff,
    CHAINID = 0x46,
}

impl fmt::LowerHex for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", ToPrimitive::to_u32(self).unwrap())
    }
}
