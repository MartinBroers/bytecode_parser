use std::io::{Error, ErrorKind};

use crate::{hex::Hex, stack::StackElement};

pub struct CallData {
    value: String,
}

impl CallData {
    pub fn new(value: String) -> CallData {
        CallData { value }
    }

    pub fn get(&self, offset: usize) -> Result<StackElement, std::io::Error> {
        println!("CALLDATA: {}", self.value);
        println!("len: {}", self.value.len());
        println!("offset: {}", offset);
        let substr = self.value.get(offset..offset + 32).unwrap().to_string();
        println!("substr: {}", substr);
        let value: Hex = match Hex::try_from(&substr) {
            Ok(v) => v,
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
        };
        println!("value: {}", value);
        let value = StackElement {
            value,
            origin: Hex(0),
            //size: (format!("{:x}", value.0).len() + 1) / 2 as usize,
            size: 32,
        };
        println!("Calldata: {:?}", value);
        Ok(value)
    }
    pub fn size(&self) -> StackElement {
        let size = self.value.len() / 2;
        StackElement {
            value: size.into(),
            origin: Hex(0),
            size: 1,
        }
    }
}
