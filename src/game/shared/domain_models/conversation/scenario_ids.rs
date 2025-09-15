use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScenarioID(String);

impl ScenarioID {
    pub fn new(value: &str) -> Self {
        ScenarioID(value.to_string())
    }
}

impl Deref for ScenarioID {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct ScenarioIDs;

impl ScenarioIDs {
    pub(crate) fn section1_start() -> ScenarioID {
        ScenarioID::new("section1_start")
    }
}
