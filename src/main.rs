use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

const STAGES: [&str; 7] = [
    r"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
=========
",
    r"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
=========
",
    r"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
=========
",
    r"
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========
",
    r"
  +---+
  |   |
  O   |
  |   |
      |
      |
=========
",
    r"
  +---+
  |   |
  O   |
      |
      |
      |
=========
",
    r"
  +---+
  |   |
      |
      |
      |
      |
=========
",
];

fn read_file(path: &str, bank: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut header = true;

    for line in reader.lines() {
        let line = line?;

        if header {
            header = false;
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let (category, word) = line
            .split_once(',')
            .map(|(c, w)| (c.trim(), w.trim()))
            .expect("invalid word");

        if bank.contains_key(category) {
            bank.get_mut(category).unwrap().push(word.to_string());
        } else {
            bank.insert(category.to_string(), vec![word.to_string()]);
        }
    }

    Ok(())
}

fn print_blanks(word: &str) -> Vec<String> {
    if word.is_empty() {
        return Vec::new();
    }

    // collect will create a vector
    word.chars().map(|_| "_".to_string()).collect()
}

fn char_index(word: &str, letter: char) -> Option<Vec<usize>> {
    let mut indexes = Vec::new();

    for (i, c) in word.chars().enumerate() {
        if c == letter {
            indexes.push(i);
        }
    }

    if indexes.is_empty() {
        return None;
    }
    Some(indexes)
}

fn check_letter_in_word(word: &str, letter: char) -> bool {
    for c in word.chars() {
        if c == letter {
            return true;
        }
    }

    false
}

fn update_blanks(blanks: &mut Vec<String>, letter: char, indexes: Option<Vec<usize>>) {
    if let Some(index_list) = indexes {
        for &index in index_list.iter() {
            if let Some(blank) = blanks.get_mut(index) {
                *blank = letter.to_string();
            }
        }
    }
}

fn check_win(blanks: &Vec<String>) -> bool {
    for letter in blanks {
        if letter == "_" {
            return false;
        }
    }

    true
}

fn display_lives(life_index: usize) {
    println!("{}", STAGES.get(life_index).unwrap_or(&""));
}

fn main() {
    let mut lives = 6;
    let mut word_bank: HashMap<String, Vec<String>> = HashMap::new();
    let file_path = "/Users/nishansubba/Desktop/Codes/projects/hangman-rust/src/words.txt";

    read_file(&file_path, &mut word_bank).unwrap();

    for (key, value) in word_bank.iter() {
        println!("{}: {:#?}", key, value);
    }

    let word = "banana";

    let mut word_vec = print_blanks(word);

    println!("Blanks: {:?}", word_vec);

    if check_letter_in_word(word, 'h') {
        let index = char_index(word, 'h');
        println!("Letter exist at index {:?}", index)
    } else {
        println!("Letter does not exist")
    }

    if check_letter_in_word(word, 'a') {
        let indexes = char_index(word, 'a');
        println!("Letter exist at index {:?}", indexes);
        update_blanks(&mut word_vec, 'a', indexes)
    } else {
        println!("Letter does not exist")
    }

    println!("Blanks: {:?}", word_vec);

    display_lives(lives);
}
