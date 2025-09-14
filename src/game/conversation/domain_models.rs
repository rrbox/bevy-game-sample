use bevy::utils::HashMap;

use crate::game::game_flow::flow::StepID;

// MARK: - Senario

#[derive(Debug, Clone)]
pub struct Scenario {
    pub start: PassageID,
    pub passages: HashMap<PassageID, Passage>,
}

impl Scenario {
    pub fn new(start: PassageID, passages: Vec<(PassageID, Passage)>) -> Self {
        Self {
            start,
            passages: passages.into_iter().collect(),
        }
    }

    pub fn get_passage(&self, id: &PassageID) -> Option<&Passage> {
        self.passages.get(id)
    }
}

#[derive(Debug, Clone)]
pub struct Passage {
    pub teller: String,
    pub text: String,
    pub flow: Flow,
}

impl Passage {
    pub fn new(teller: &str, text: &str, flow: Flow) -> Self {
        Passage {
            teller: teller.to_string(),
            text: text.to_string(),
            flow,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Flow {
    Branch {
        decisions: Box<[Decision]>,
    },
    Linear {
        next: PassageID,
    },
    End {
        next_step: StepID,
    },
}

impl Flow {
    pub fn branch(decisions: Vec<Decision>) -> Self {
        Self::Branch {
            decisions: decisions.into_boxed_slice(),
        }
    }

    pub fn linear(next: PassageID) -> Self {
        Self::Linear { next }
    }

    pub fn end(next_step: StepID) -> Self {
        Self::End { next_step }
    }
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub text: String,
    pub decision_id: PassageID,
}

impl Decision {
    pub fn new(text: &str, decision_id: PassageID) -> Self {
        Self {
            text: text.to_string(),
            decision_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PassageID(u64);

impl PassageID {
    pub const fn new(value: u64) -> Self {
        PassageID(value)
    }
}

impl From<u64> for PassageID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
