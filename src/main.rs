use std::{io, cmp::Ordering} ;
use rand::Rng;

fn main() {
    println!("Guess a Number between 1 to 100 !");
    
    let secret_number= rand::thread_rng().gen_range(1..101);

    
    
loop {
    
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess: i32 = guess.trim().parse().expect("Please type in a number");

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("you guessed too big {} ", guess),
        Ordering::Less => println!("you guessed too small {}", guess),
        Ordering::Equal => {
            println!("you guessed it right WOOOOHOOO {}  you win!", guess);
            break;
        
            }   
        }
    }
}
