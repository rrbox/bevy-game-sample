use bevy::utils::HashMap;

use crate::game::game_flow::flow::StepID;

// MARK: - Senario

pub struct Scenario {
    start: PassageID,
    passages: HashMap<PassageID, Passage>,
}

impl Scenario {
    pub fn new(start: PassageID, passages: Vec<(PassageID, Passage)>) -> Self {
        Self {
            start,
            passages: passages.into_iter().collect(),
        }
    }
}

pub struct Passage {
    teller: String,
    text: String,
    flow: Flow,
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

pub enum Flow {
    Branch { decisions: Box<[Decision]> },
    Linear { next: PassageID },
    End { next_step: StepID },
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

pub struct Decision {
    text: String,
    decision_id: PassageID,
}

impl Decision {
    pub fn new(text: &str, decision_id: PassageID) -> Self {
        Self {
            text: text.to_string(),
            decision_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
