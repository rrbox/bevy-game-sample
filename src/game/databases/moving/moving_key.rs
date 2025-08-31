use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovingKey(String);

impl From<&str> for MovingKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Deref for MovingKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
