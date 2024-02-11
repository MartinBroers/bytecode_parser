use crate::{hex::Hex, stack::StackElement};

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryElement {
    pub value: Hex,
    pub origin: Option<Hex>,
}

#[derive(Debug, Clone)]
pub struct Memory {
    elements: Vec<MemoryElement>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            elements: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn get_contents(&self) -> &Vec<MemoryElement> {
        &self.elements
    }

    pub fn mload(&mut self, offset: Hex) -> MemoryElement {
        let offset = offset.0 as usize;
        if self.elements.len() < (offset + 32) {
            for _ in self.elements.len()..(offset + 32) {
                self.elements.push(MemoryElement {
                    value: Hex(0),
                    origin: None,
                });
            }
        }
        let mut result: MemoryElement = MemoryElement {
            value: Hex(0),
            origin: None,
        };
        for i in offset..(offset + 32) {
            let element = self.elements.get(i).unwrap();
            result.value = result.value << Hex(8);
            result.value += element.value;
            if let Some(origin) = element.origin {
                result.origin = Some(origin);
            }
        }
        result
    }

    pub fn mstore(&mut self, element: StackElement, offset: Hex, index: Hex) {
        let offset = offset.0 as usize;
        // ensure the memory array is lengthy enough.
        while self.elements.len() < (offset + 32) {
            for _ in 0..32 {
                self.elements.push(MemoryElement {
                    value: Hex(0),
                    origin: None,
                });
            }
        }

        // prepent the new value with enough 0's
        for i in offset..((offset + 32) - element.size) {
            self.elements[i] = MemoryElement {
                value: Hex(0),
                origin: Some(index),
            };
        }

        // Store the new value.
        let mut value = element.value;
        for i in (((offset + 32) - element.size)..(offset + 32)).rev() {
            let tmp_value = value >> Hex(8);
            let hex = value & Hex(0xff);
            value = tmp_value;
            self.elements[i] = MemoryElement {
                value: hex,
                origin: Some(element.origin),
            };
        }
    }
}
