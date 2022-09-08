use std::fs;
use std::io;

use std::collections::HashSet;
use std::time::Instant;

fn get_words_set(len: i32, first_letter: char) -> Result<HashSet<String>, std::io::Error> {
    let mut acceptable_words = HashSet::new();
    fs::read_to_string(format!("assets/{}_letters/{}.txt", len, first_letter)).map(|s| {
        s.lines().for_each(|s| {
            acceptable_words.insert(s.to_owned());
        });
    })?;

    Ok(acceptable_words)
}

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.clear();
        if stdin.read_line(input).is_err() {
            panic!("Error, while reading user input!");
        }
        let now = Instant::now();

        let query = input.trim();
        let len = query.len() as i32;
        let first_letter = match query.chars().next() {
            Some(first_letter) => first_letter,
            None => panic!("empty word!"),
        };
        let acceptable_words = match get_words_set(len, first_letter) {
            Ok(words) => words,
            Err(_) => panic!("could not create a words set"),
        };

        println!("{:?}", acceptable_words.get(query));
        println!("done in {}", now.elapsed().as_millis());
    }
}
