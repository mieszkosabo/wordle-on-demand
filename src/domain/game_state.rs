use anyhow::Result;

use crate::words::get_random_word;
use multiset::HashMultiSet;

use super::GameId;

#[derive(Debug, PartialEq, Eq)]
pub struct UserGameState {
    pub game_id: GameId,
    pub word_len: u8,
    pub inputs: Vec<Vec<CheckedLetter>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CheckedLetter {
    CorrectPlace(char),
    Misplaced(char),
    Incorrect(char),
}

#[derive(Debug)]
pub struct ServerGameState {
    pub game_id: GameId,
    pub word_len: u8,
    pub word: String,
    pub user_choices: Vec<String>,
}

impl ServerGameState {
    pub fn new(word_len: u8) -> Result<Self> {
        let word = get_random_word(word_len)?;
        Ok(Self {
            game_id: GameId::new(),
            word_len,
            user_choices: vec![],
            word,
        })
    }
}

impl From<ServerGameState> for UserGameState {
    fn from(sgs: ServerGameState) -> Self {
        let w = sgs.word;
        let inputs = sgs
            .user_choices
            .iter()
            .map(|s| input_to_checked_letters(&w, s))
            .collect();
        UserGameState {
            game_id: sgs.game_id,
            word_len: sgs.word_len,
            inputs,
        }
    }
}

// assumes w.len() == input.len()
// FIXME: express in types
fn input_to_checked_letters(w: &str, input: &str) -> Vec<CheckedLetter> {
    let mut result = vec![None; w.len()];
    let mut ms = HashMultiSet::new();
    w.chars().for_each(|c| ms.insert(c));

    // first pass - check for correctly placed chars
    input.chars().enumerate().for_each(|(idx, c)| {
        if Some(c) == w.chars().nth(idx) {
            ms.remove(&c);
            result[idx] = Some(CheckedLetter::CorrectPlace(c));
        }
    });

    // second pass, do the rest
    input.chars().enumerate().for_each(|(idx, c)| {
        if result[idx].is_none() {
            if w.contains(c) && ms.contains(&c) {
                ms.remove(&c);
                result[idx] = Some(CheckedLetter::Misplaced(c));
            } else {
                result[idx] = Some(CheckedLetter::Incorrect(c));
            }
        }
    });

    result.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use crate::domain::GameId;

    use super::{input_to_checked_letters, CheckedLetter, ServerGameState, UserGameState};

    #[test]
    fn test_input_to_checked_letter() {
        assert_eq!(
            input_to_checked_letters("abcde", "atena"),
            vec![
                CheckedLetter::CorrectPlace('a'),
                CheckedLetter::Incorrect('t'),
                CheckedLetter::Misplaced('e'),
                CheckedLetter::Incorrect('n'),
                CheckedLetter::Incorrect('a'),
            ],
        );

        assert_eq!(
            input_to_checked_letters("edcba", "atena"),
            vec![
                CheckedLetter::Incorrect('a'),
                CheckedLetter::Incorrect('t'),
                CheckedLetter::Misplaced('e'),
                CheckedLetter::Incorrect('n'),
                CheckedLetter::CorrectPlace('a'),
            ],
        );

        assert_eq!(
            input_to_checked_letters("edaba", "atena"),
            vec![
                CheckedLetter::Misplaced('a'),
                CheckedLetter::Incorrect('t'),
                CheckedLetter::Misplaced('e'),
                CheckedLetter::Incorrect('n'),
                CheckedLetter::CorrectPlace('a'),
            ],
        )
    }

    #[test]
    fn test_from_server_gs_to_user_gs() {
        let game_id = GameId::new();
        let sgs = ServerGameState {
            game_id: game_id.clone(),
            word_len: 5,
            word: "abcde".into(),
            user_choices: vec!["atena".into(), "mocne".into(), "katan".into()],
        };

        assert_eq!(
            UserGameState::from(sgs),
            UserGameState {
                game_id,
                word_len: 5,
                inputs: vec![
                    vec![
                        CheckedLetter::CorrectPlace('a'),
                        CheckedLetter::Incorrect('t'),
                        CheckedLetter::Misplaced('e'),
                        CheckedLetter::Incorrect('n'),
                        CheckedLetter::Incorrect('a'),
                    ],
                    vec![
                        CheckedLetter::Incorrect('m'),
                        CheckedLetter::Incorrect('o'),
                        CheckedLetter::CorrectPlace('c'),
                        CheckedLetter::Incorrect('n'),
                        CheckedLetter::CorrectPlace('e'),
                    ],
                    vec![
                        CheckedLetter::Incorrect('k'),
                        CheckedLetter::Misplaced('a'),
                        CheckedLetter::Incorrect('t'),
                        CheckedLetter::Incorrect('a'),
                        CheckedLetter::Incorrect('n'),
                    ],
                ]
            }
        )
    }
}
