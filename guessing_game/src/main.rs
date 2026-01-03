use std::io;
use rand::Rng;


fn main(){
    println!("Guess the number !!");
    println!("input your guess:");
    let mut guess = String::new();

    let num = rand::rng().random_range(1..=100);

    println!("The secret number is: {num}");


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line !!");

    println!("You guessed {guess}");
}