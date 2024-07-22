use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){
    println!("Guess The Number");

    let random_num = rand::thread_rng().gen_range(1..=100);  //we call the rand::thread_rng function that gives us the particular random number generator
    println!("The secret number is: {random_num}");
    
    println!("Enter The Number: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) //we call the stdin function from the io module, which will allow us to handle user input
        .expect("Failed to read line");
    //could have been written as
    //io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please Type Number"); //u32 maens 32bit integer

    println!("You Guessed {guess}");

    //comparing the guess with random number
    match guess.cmp(&random_num) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win"),
    }
}