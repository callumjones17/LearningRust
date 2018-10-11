extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let var = "Hello again";
    println!("Hello, world! {},{}", var, var);
    test();
    println!("Program Over!");
}

fn test(){

    let rand_num = rand::thread_rng().gen_range(1,500);

    println!("Guess the Number Game!");
    println!("The Secret Number is: {}",rand_num);
    println!("Please now Guess the Secret Number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).ok().expect("Failed to Read Line");
    let guess: u32 = guess.trim().parse()
        .ok()
        .expect("Please type a number!");


    println!();
    println!("You Guessed: {}",guess);

    match guess.cmp(&rand_num){
        Ordering::Less => println!("Too Small"),
        Ordering::Equal => println!("You Win"),
        Ordering::Greater => println!("Too Big"),
    }

    return;
}
