use std::io;
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
    println!("You Guessed {guess}");
}