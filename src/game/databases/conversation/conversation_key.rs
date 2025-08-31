use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConversationKey(String);

impl From<&str> for ConversationKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Deref for ConversationKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
