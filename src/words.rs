use rand::seq::IteratorRandom;
use std::{collections::HashSet, fs};

use anyhow::{Context, Result};

fn get_words_set(len: usize, first_letter: char) -> Result<HashSet<String>, std::io::Error> {
    let mut acceptable_words = HashSet::new();
    fs::read_to_string(format!("assets/{}_letters/{}.txt", len, first_letter)).map(|s| {
        s.lines().for_each(|s| {
            acceptable_words.insert(s.to_owned());
        });
    })?;

    Ok(acceptable_words)
}

pub fn check_if_valid_word(w: &String) -> Result<bool> {
    let len = w.len();
    let first_letter = w.chars().next().context("Could not get first letter.")?;
    let set = get_words_set(len, first_letter)?;
    match set.get(w) {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub fn get_random_word(len: u8) -> Result<String> {
    let mut rng = rand::thread_rng();
    fs::read_to_string("assets/game_words.txt")
        .context("Could not open the game_words.txt file!")?
        .lines()
        .filter(|s| s.len() as u8 == len)
        .choose(&mut rng)
        .context("Could not choose random word")
        .map(|a| a.to_string())
}
