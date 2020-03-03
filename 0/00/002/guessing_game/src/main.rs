use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    // -Create a mutable variable
    //  that is currently bound to a new,
    //  empty instance of a String.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);

    // stdin is in the io module
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}

// Initialize project
// > cargo new guessing_game
// > cd guessing_game

// Build and Run
// > cargo run
