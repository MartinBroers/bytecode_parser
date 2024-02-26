use std::fmt::Debug;

use crate::hex::Hex;

#[derive(Clone, PartialEq)]
pub struct StackElement {
    pub value: Hex,
    pub origin: Hex,
    pub size: usize,
}

impl Debug for StackElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Stack {
    elements: Vec<StackElement>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> Option<StackElement> {
        self.elements.pop()
    }

    pub fn push(&mut self, element: StackElement) {
        self.elements.push(element);
    }

    pub fn extend(&mut self, elements: Stack) {
        self.elements.extend(elements.elements);
    }

    pub fn get(&self, index: usize) -> Option<&StackElement> {
        self.elements.get(index)
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.elements.swap(a, b)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
