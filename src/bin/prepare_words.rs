use core::panic;
use std::{
    fs::{self, read_to_string},
    io::Write,
};

// from a single file containing polish words of lengths
// from 2 to 21 we want to create a file structure of a shape:
//
// 2_letters/
//      |- a.txt
//      |  b.txt
//      |- c.tx
// ...
// 3_letters/
// ...

fn add_word_to_file(w: String) -> Result<(), std::io::Error> {
    let len = w.len();
    let first_letter = match w.chars().next() {
        Some(first_letter) => first_letter,
        None => panic!("empty word!"),
    };

    fs::create_dir_all(format!("assets/{}_letters", len))?;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("assets/{}_letters/{}.txt", len, first_letter))?;

    writeln!(file, "{}", w)?;

    Ok(())
}

fn main() {
    println!("Cleaning up the assets");

    if let Err(s) = fs::remove_dir_all("./assets") {
        println!("{:?}", s);
    }
    if let Err(s) = fs::create_dir("./assets") {
        println!("{:?}", s);
    }

    println!("creating words file structure");

    read_to_string("slowa.txt")
        .expect("Could not read slowa.txt")
        .lines()
        .for_each(|w| {
            add_word_to_file(w.to_string())
                .map_err(|e| {
                    println!("{:?}", e);
                })
                .unwrap();
        })
}
