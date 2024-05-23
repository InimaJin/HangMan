use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::process;

mod drawing;
use drawing::Canvas;

fn main() {
    let word: Vec<char> = secret_word("words.txt")
        .unwrap_or_else(|e| {
            println!("Failed to read file.");
            process::exit(1);
        })
        .chars()
        .collect();
    let mut word_hidden: Vec<char> = word.iter().map(|_| '_').collect();
    let mut guessed_letters = Vec::new();
    let mut chars_left_count = word_hidden.len();
    let mut correct_guess: bool;

    let mut canvas = Canvas::build("visuals.txt").unwrap_or_else(|err| {
        println!("Unable to prepare image drawing: {:?}", err);
        process::exit(1);
    });
    let mut wrong_guesses_count: usize = 0;
    let max_wrong_guesses = canvas.images.len();

    canvas.draw("LET'S PLAY HANGMAN!", &word_hidden, wrong_guesses_count);
    loop {
        correct_guess = false;

        let user_guess = get_guess();
        if let Err(err_msg) = user_guess {
            let message = format!("Invalid input. {}", err_msg);
            canvas.draw(&message, &word_hidden, wrong_guesses_count);
            continue;
        }
        let user_guess = user_guess.unwrap();
        if guessed_letters.contains(&user_guess) {
            //println!("You have already guessed '{}'.", user_guess);
            let message = format!("YOU HAVE ALREADY GUESSED '{}'.", user_guess);
            canvas.draw(&message, &word_hidden, wrong_guesses_count);
            continue;
        }

        for (i, ch) in word.iter().enumerate() {
            if ch.to_string() == user_guess {
                word_hidden[i] = *ch;
                chars_left_count -= 1;
                correct_guess = true;
            }
        }
        guessed_letters.push(user_guess);

        if !correct_guess {
            //println!("Letter not in my word.");
            wrong_guesses_count += 1;
            canvas.draw("LETTER NOT IN MY WORD.", &word_hidden, wrong_guesses_count);
            if wrong_guesses_count == max_wrong_guesses {
                let word_formatted: String = word.iter().collect();
                println!("You lose! My word was {}", word_formatted);
                break;
            }
            continue;
        } else {
            canvas.draw("Good.", &word_hidden, wrong_guesses_count);

            if chars_left_count == 0 {
                println!("GOOD");
                break;
            }
        }
    }
}

fn secret_word(wordfile: &str) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();
    let mut filehandle = File::open(wordfile)?;
    filehandle.read_to_string(&mut contents);

    let lines: Vec<&str> = contents.lines().collect();
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..lines.len());
    let word = lines[random_index];

    Ok(String::from(word))
}

fn get_guess() -> Result<String, &'static str> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read from stdin");
    let guess = guess.trim();

    if guess.len() != 1 {
        return Err("Should be one character.");
    }

    Ok(String::from(guess).to_lowercase())
}
