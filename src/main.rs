use rand::Rng;
use std::collections::{HashMap, HashSet};
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

fn get_user_input() -> char {
    loop {
        println!("Enter a letter: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_trim = input.trim();

        if input_trim.chars().count() != 1 {
            println!("Please enter exactly ONE character.");
            continue;
        }

        let ch = input_trim.chars().next().unwrap();

        // the input must be a char letter

        if !ch.is_ascii_alphabetic() {
            println!("Please enter a LETTER (a-z).");
            continue;
        }
        return ch.to_ascii_lowercase();
    }
}

fn generate_rand_index(words: &Vec<String>) -> usize {
    let mut rng = rand::rng();

    let rand_index = rng.random_range(0..words.len());
    rand_index
}

fn get_word(word_bank: &HashMap<String, Vec<String>>, chosen_category: &mut String) -> String {
    for (i, key) in word_bank.keys().enumerate() {
        println!("{}: {}", i + 1, key);
    }

    let categories: Vec<&String> = word_bank.keys().collect();

    loop {
        println!("Enter a category choice: ");
        let mut input = String::new();

        let num_categories = word_bank.keys().len();

        io::stdin().read_line(&mut input).unwrap();

        let index: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if index >= 1 && index <= num_categories {
            *chosen_category = categories[index - 1].clone();
            let word_list = word_bank.get(chosen_category).unwrap();

            let rand_index = generate_rand_index(word_list);

            return word_list[rand_index].clone();
        }
        println!("Choice must be between 1 and {}.", num_categories);
    }
}

fn check_if_letter_is_picked(picked_vec: &HashSet<char>, letter: char) -> bool {
    picked_vec.contains(&letter)
}

fn play_game(word_bank: &HashMap<String, Vec<String>>) {
    let mut chosen_category = String::new();
    let mut picked: HashSet<char> = HashSet::new();
    let mut lives: usize = 6;

    // pick category + random word
    let word = get_word(word_bank, &mut chosen_category);
    let mut blanks = print_blanks(&word);

    println!("Chosen category: {}", chosen_category);

    loop {
        println!("\n==================================");
        display_lives(lives);
        println!("Word: {:?}", blanks);
        println!("Picked letters: {:?}", picked);
        println!("Lives: {}", lives);

        let guess = get_user_input();

        // Already guessed?
        if check_if_letter_is_picked(&picked, guess) {
            println!("You already guessed '{}', try another letter.", guess);
            continue;
        }

        picked.insert(guess);

        // check if letter in word
        if check_letter_in_word(&word, guess) {
            println!("Correct! '{}' is in the word.", guess);
            let indexes = char_index(&word, guess);
            update_blanks(&mut blanks, guess, indexes);

            if check_win(&blanks) {
                println!("\nYOU WIN! 🎉");
                println!("The word was: {}", word);
                break;
            }
        } else {
            println!("Wrong guess! '{}' is NOT in the word.", guess);
            if lives == 0 {
                println!("\nGAME OVER! 💀");
                println!("The word was: {}", word);
                break;
            }
            lives -= 1;
        }
    }
}

fn main() {
    let mut word_bank: HashMap<String, Vec<String>> = HashMap::new();

    // Load file
    let file_path = "assets/words.txt";
    read_file(file_path, &mut word_bank).expect("Failed to load word bank");

    loop {
        println!("========== HANGMAN MENU ==========");
        println!("1. Play Game");
        println!("2. Quit");
        println!("Choose an option: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => play_game(&word_bank),
            "2" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}
