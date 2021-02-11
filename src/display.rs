#[derive(Clone)]
pub enum CharacterDisplay {
  SNAKE,
  APPLE,
  TILE,
}

impl CharacterDisplay {
  pub fn as_char(&self) -> char {
    match &self {
      CharacterDisplay::SNAKE => '🟩',
      CharacterDisplay::APPLE => '🟥',
      CharacterDisplay::TILE => '⬜',
    }
  }
}
