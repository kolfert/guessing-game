use std::io;
use rand::Rng;


fn main() 
{
    println!("Guess a number!");

    let secret_num = rand::thread_rng().gen_range(1,101);
    println!("The secret number is : {}", secret_num);

    println!("Input your guess, numbers only, 0-100");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
