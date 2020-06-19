use rand::Rng;
use std::io;

fn main() {
    let mut lives = 6;
    let mut done = false;
    let random_word = get_random_word();
    let mut char_vec: Vec<char> = random_word.chars().collect();
    let mut underscored_array = vec!['_'; random_word.len()];
    
    while !done {
        println!("Welcome to Hangman\n\n");
        println!("Lives: {}", lives);
        println!("{}", underscored_array.iter().cloned().collect::<String>());
        let guess = get_guess();
        let mut more_matches = true;

        if char_vec.contains(&guess){ 
            while more_matches{
                if char_vec.contains(&guess){
                    println!("You guessed a correct character!");
                    let index = char_vec.iter().position(|&x| x == guess);
                    match index {
                        None => println!("Character in string but no index found"),
                        Some(value) => {
                            underscored_array[value] = char_vec[value];
                            char_vec[value] = ' ';
                        }
                    }
                } else {
                    println!("This character is not in the word.");
                    more_matches = false;
                }
            }
        } else {
            lives -= 1;
        }

        print!("\x1B[2J");

        if !underscored_array.contains(&'_') {
            print!("\x1B[2J");
            println!("\nYou won!\n");
            done = true;
        }

        if lives < 1 {
            print!("\x1B[2J");
            println!("\nOh no! You lost!");
            println!("The word was: {}\n", random_word);
            done = true;
        }
    }
}

fn get_guess() -> char {
    println!("Please input your guess: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: char = guess.trim().parse()
        .expect("Please type a character!");
        println!("You guessed: {}", guess);

    return guess;
}

fn get_random_word() -> String {
    let word_file_bytes = include_bytes!("./words.txt");
    let words = String::from_utf8_lossy(word_file_bytes);
    let word_vector: Vec<&str> = words.split("\n").collect();
    let word_vector_length = word_vector.len();
    let random_index = rand::thread_rng().gen_range(0, word_vector_length);
    let random_word = word_vector[random_index];
    return String::from(random_word);
}