//Guessing Game Tutorial

extern crate rand;

use std::io;
use std::cmp::Ordering;
use std::thread;
use std::sync::{Mutex,Arc};
use rand::Rng;

fn main() {
    let var = "Hello again";
    //println!("Hello, world! {},{}", var, var);
    //secret_number_game();
    philosophers_problem();
    //println!("Program Over!");
}

fn secret_number_game(){

    let rand_num = rand::thread_rng().gen_range(1,500);

    println!("Guess the Number Game!");
    println!("The Secret Number is: {}",rand_num);

    loop {
        println!("Please now Guess the Secret Number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).ok().expect("Failed to Read Line");
//        let guess: u32 = guess.trim().parse()
//            .ok()
//            .expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };



        println!();
        println!("You Guessed: {}", guess);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }

    return;
}






struct Philosopher{
    name: String,
    left: usize,
    right: usize,
}
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

struct Table{
    forks: Vec<Mutex<()>>,
}




fn philosophers_problem(){
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    let philosophers = vec![
        Philosopher::new("Ben",0,1),
        Philosopher::new("David",1,2),
        Philosopher::new("John",2,3),
        Philosopher::new("Jack",3,4),
        Philosopher::new("Matt",4,0),
    ];


    //Make them Eat!
    //for p in &philosophers{
    //    p.eat();
    //}

    //Make them Eat at the same time!!!!
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();
    for h in handles {
        h.join().unwrap();
    }




    return;
}
