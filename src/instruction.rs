use core::fmt;
use std::ops::{Add, AddAssign};

use crate::opcode::OpCode;

#[derive(Clone, Default, PartialEq, Copy, PartialOrd, Hash, Eq)]
pub struct Hex(pub u32);
impl AddAssign for Hex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Add for Hex {
    type Output = Hex;

    fn add(self, rhs: Self) -> Self::Output {
        Hex(self.0 + rhs.0)
    }
}

impl From<usize> for Hex {
    fn from(value: usize) -> Self {
        Hex(value as u32)
    }
}
impl From<i32> for Hex {
    fn from(value: i32) -> Self {
        Hex(value as u32)
    }
}
impl From<u32> for Hex {
    fn from(value: u32) -> Self {
        Hex(value)
    }
}

#[derive(Clone, Default)]
pub struct Instruction {
    pub args: Vec<Hex>,
    pub opcode: OpCode,
    pub index: Hex,

    pub parsed: bool,
}

impl fmt::LowerHex for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        fmt::LowerHex::fmt(&val, f)
    }
}

impl fmt::Debug for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self)
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement custom formatting for Display.
        write!(f, "{:04x}", self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct BytecodeInstruction {
    pub instruction: Instruction,
    pub label: String,

    pub used_arg: Option<Hex>,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:<10?}\t args: {:?}", self.opcode, self.args)
    }
}

#[derive(Debug, Clone)]
pub enum JumpType {
    Conditional,
    Unconditional,
}

#[derive(Clone)]
pub struct JumpInstruction {
    pub instruction: Instruction,
    pub jump_type: JumpType,
    pub target: Hex,
    pub source: Hex,
}
impl std::fmt::Debug for JumpInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type: {:?}, target {:?}, source: {:04x}",
            &self.jump_type, &self.target, &self.source
        )
    }
}

#[derive(Clone)]
pub struct InstructionSet {
    pub instructions: Vec<BytecodeInstruction>,
    pub jumpdest: Hex,
    pub end: Hex,

    pub jumps: Vec<JumpInstruction>,

    pub stack: Vec<Hex>,
}
impl std::fmt::Debug for InstructionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "start: {:04x}", self.jumpdest).unwrap();
        for instruction in &self.instructions {
            writeln!(
                f,
                "{:04x}\t{:?}\t{:<10}\t{:?}\t{:?}",
                instruction.instruction.index,
                instruction.instruction.opcode,
                instruction.label,
                instruction.used_arg,
                instruction.instruction.args,
            )
            .unwrap();
        }
        Ok(())
    }
}

impl InstructionSet {
    pub fn push(&mut self, value: BytecodeInstruction) {
        self.instructions.push(value);
    }
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}