use core::fmt;
use std::num::Wrapping;
use std::{
    num::ParseIntError,
    ops::{Add, AddAssign, BitAnd, Mul, Rem, Shl, Shr, Sub},
};

#[derive(Clone, Default, PartialEq, Copy, PartialOrd, Hash, Eq)]
pub struct Hex(pub u64);
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
impl Shl<Hex> for Hex {
    type Output = Self;

    fn shl(self, shift: Hex) -> Self::Output {
        let wrapped_self = Wrapping(self.0);
        Self(wrapped_self.shl(shift.0 as usize).0)
    }
}
impl Shr<Hex> for Hex {
    type Output = Self;

    fn shr(self, shift: Hex) -> Self::Output {
        let wrapped_self = Wrapping(self.0);
        Self(wrapped_self.shr(shift.0 as usize).0)
    }
}

impl BitAnd for Hex {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs & rhs.0)
    }
}

impl Sub for Hex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs - rhs.0)
    }
}

impl Rem for Hex {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs % rhs.0)
    }
}

impl From<usize> for Hex {
    fn from(value: usize) -> Self {
        Hex(value as u64)
    }
}
impl From<i32> for Hex {
    fn from(value: i32) -> Self {
        Hex(value as u64)
    }
}
impl From<u32> for Hex {
    fn from(value: u32) -> Self {
        Hex(value as u64)
    }
}
impl TryFrom<&String> for Hex {
    type Error = ParseIntError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Hex(value.parse::<u64>()?))
    }
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
