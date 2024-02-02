use std::collections::HashMap;

use log::info;

use crate::instruction::{Hex, JumpInstruction};

#[derive(Debug, Clone)]
pub struct ParsedInstructionSet {
    pub start: Hex,
    pub end: Hex,
    pub target: Option<Hex>,

    pub jump: Option<JumpInstruction>,

    pub stack: Vec<Hex>,
}

#[derive(Clone, Debug)]
pub struct Flow {
    steps: HashMap<Hex, ParsedInstructionSet>,
}

impl Flow {
    pub fn new(start: ParsedInstructionSet) -> Flow {
        let mut steps = HashMap::new();
        steps.insert(start.start, start);
        Flow { steps }
    }

    pub fn add_step(&mut self, step: ParsedInstructionSet) {
        self.steps.insert(step.start, step);
    }

    // Return the target for which we have no continuation.
    pub fn get_last_step(&self) -> Option<ParsedInstructionSet> {
        for (_, step) in &self.steps {
            if let Some(target) = step.target {
                if self.steps.get(&target).is_none() {
                    return Some(step.clone());
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn print(&self) {
        let mut target = Some(Hex(0));
        while let Some(t) = target {
            if let Some(step) = self.steps.get(&t) {
                println!("step start {:?}, jumping to {:?}", step.start, step.target);
                target = step.target;
            } else {
                break;
            }
        }
    }
}
