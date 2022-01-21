
// 1. Import words randomly from stored list.

// 2. Print introduction information to player

// 3. Ask for guess

// 4. Compare guess to word. Reprint with highlighted letters.

// 5. If guess word is correct, winner!

use std::io;

fn print_intro() {
    println!("Welcome to WORDLE\n");

    println!("A random 5-letter word has been selected. You have 6 tries to guess the correct word.
    When you make a guess, correctly-placed letters will be highlighted \x1b[92mgreen\x1b[0m. Letters that appear in the word but are not in the right location will be highlighted \x1b[93myellow\x1b[0m.
    Letters not in the word will remain white.\n");
}


// Handles player input for each guess. Returns whether they succeeded in guessing the word.
fn game_loop() -> bool {
    let guess_count = 6;
    let mut guess = String::new();

    for i in 1..guess_count {
        // Ask for user input
        println!("Guess #{}: ", i);

        loop {
            // Get new input
            guess.clear();
            io::stdin().read_line(&mut guess)
                .expect("Failed to read input...");
            guess = guess.trim().to_string();
        
            if guess.len() != 5 {
                println!("That isn't 5 letters! Try again: ");
            } else if !guess.chars().all(char::is_alphabetic) {
                println!("Uh oh! That entry has more than letters. Try again: ")
            } else {
                break;
            }
        }
        
        // Reprint user's input
        println!("You guessed: {}", guess);

        // Compute usage


    }

    // Evaluate victory condition and return
    true
}

// Victory/defeat message and asks the player if they want to play again
fn game_over_prompt(victorious: bool) -> bool{
    if victorious {
        println!("Congratulations! You've guessed the word.\n");
    } else {
        println!("Defeat!\n")
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



fn main() {
    print_intro();

    loop {
        let game_result = game_loop();

        // Ask if the user wants to play again. If so, run game_loop. If not, exit.
        let play_again: bool = game_over_prompt(game_result);
        if !play_again {
            break;
        }
    }
}
