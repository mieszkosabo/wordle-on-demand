use std::fs::{read_to_string, write};
use unicode_segmentation::{self, UnicodeSegmentation};

fn main() {
    let words = read_to_string("assets/game_words.txt")
        .expect("could not open game_words.txt")
        .lines()
        .filter(|w| w.graphemes(true).count() >= 3)
        .filter(|w| !w.contains(' '))
        .collect::<Vec<&str>>()
        .join("\n");

    write("assets/game_words.txt", words).expect("could not write to game_words.txt");
}
