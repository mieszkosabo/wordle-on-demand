use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameId(uuid::Uuid);

impl GameId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for GameId {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Uuid> for GameId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
