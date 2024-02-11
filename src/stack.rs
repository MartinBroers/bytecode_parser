use crate::hex::Hex;

#[derive(Debug, Clone, PartialEq)]
pub struct StackElement {
    pub value: Hex,
    pub origin: Hex,
    pub size: usize,
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

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
