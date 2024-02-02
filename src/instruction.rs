use core::fmt;
use std::ops::{Add, AddAssign, Mul};

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
impl Mul for Hex {
    type Output = Hex;

    fn mul(self, rhs: Self) -> Self::Output {
        Hex(self.0 * rhs.0)
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

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub args: Vec<Hex>,
    pub opcode: OpCode,
    pub index: Hex,
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
    pub fn new(instruction: Instruction, stack: Vec<Hex>) -> ParsedInstruction {
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JumpInstruction {
    pub instruction: Instruction,
    pub jump_type: JumpType,
    pub target: Option<Hex>,
    pub source: Hex,
    pub condition: Option<Hex>,
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InstructionSet {
    pub instructions: Vec<ParsedInstruction>,
    pub start: Hex,
    pub end: Hex,

    pub jumps: Vec<JumpInstruction>,

    pub stack: Vec<Hex>,
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
