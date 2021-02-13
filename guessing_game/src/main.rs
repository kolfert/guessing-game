use std::io;


fn main() 
{
    println!("Guess a number!");
    println!("Input your guess, numbers only, 0-100");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
