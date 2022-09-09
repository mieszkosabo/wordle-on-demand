use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameId(String);

impl GameId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().as_hyphenated().to_string())
    }
}

impl Default for GameId {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<str> for GameId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
