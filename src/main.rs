/*
Author:     Max DeSantis
Info:       Implementation of popular game WORDLE in Rust, as a learning exercise.
*/
extern crate rand;

use std::io::{self, Write, Read, BufRead, BufReader};
use std::time;
use std::thread;
use std::fs;

fn print_intro() {
    println!("Welcome to WORDLE\n");

    println!("A random 5-letter word has been selected. You have 6 tries to guess the correct word.
    When you make a guess, correctly-placed letters will be highlighted \x1b[92mgreen\x1b[0m. Letters that appear in the word but are not in the right location will be highlighted \x1b[93myellow\x1b[0m.
    Letters not in the word will remain white.\n");
}

fn grab_new_word() -> String {

    let buf = BufReader::new(fs::File::open("./word_list.txt").expect("Failed to read word list!"));
    
    let lines: Vec<_> = buf.lines().collect();
    let line_count = lines.len();

    let random_line_number = rand::random::<usize>() % line_count;

    let new_word:String = lines.into_iter().nth(random_line_number).expect("Unable to find word").unwrap();

    return new_word;
}
// Handles player input for each guess. Returns whether they succeeded in guessing the word.
fn game_loop(word:&str) -> bool {
    let guess_count = 6;
    let mut guess = String::new();

    for i in 1..guess_count {
        // Ask for user input
        println!("\r\nGuess #{}: ", i);

        loop {
            // Get new input
            guess.clear();
            io::stdin().read_line(&mut guess)
                .expect("Failed to read input...");
            guess = guess.trim().to_string();
        
            if guess.len() != 5 {
                println!("\x1b[91mThat isn't 5 letters!\x1b[0m Try again: ");
            } else if !guess.chars().all(char::is_alphabetic) {
                println!("\x1b[91mLetters only!\x1b[0m Try again: ")
            } else {
                break;
            }
        }
        
        // Reprint user's input
        println!("You guessed: {}", guess);


        // Parse user's guess and output green letter for correct placement, yellow for incorrect placement, and normal for wrong letter.
        for (i, c) in guess.chars().enumerate() {
            if c == word.as_bytes()[i] as char {    // Letter in correct position
                print!("\x1b[92m{}\x1b[0m ", c);
            } else if word.contains(c){             // Letter in wrong position, but is present
                print!("\x1b[95m{}\x1b[0m ", c);
            } else {
                print!("{} ", c);                   // Letter is not present
            }
            let now = time::Instant::now();
            thread::sleep(time::Duration::from_millis(500));
            assert!(now.elapsed() >= time::Duration::from_millis(500));
            io::stdout().flush().unwrap();
        }

        // Guessed the correct word!
        if guess.eq(word) {
            return true;
        }
    }

    // Evaluate victory condition and return
    return false
}

// Victory/defeat message and asks the player if they want to play again
fn game_over_prompt(victorious: bool) -> bool{
    if victorious {
        println!("\r\nCongratulations! You've guessed the word.\n");
    } else {
        println!("\r\nDefeat!\n")
    }

    println!("Would you like to try again? [yes/no]");
    let mut play_again = String::new();

    io::stdin().read_line(&mut play_again)
        .expect("Failed to read input");
    
    if play_again.trim().to_lowercase().eq("yes") {
        return true;
    } else {
        return false;
    }
}


// Handles game entry point and loop if playing again
fn main() {
    print_intro();

    loop {
        let random_word = grab_new_word();
        let game_result = game_loop(&random_word);

        // Ask if the user wants to play again. If so, run game_loop. If not, exit.
        let play_again: bool = game_over_prompt(game_result);
        if !play_again {
            break;
        }
    }
}
