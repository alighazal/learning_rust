use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // to read docs, you can use cargo doc --open 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect( "failed to read line" );
    
        println!("you guessed: {guess}");

        // this is called variable shadowing 
        let guess : i32 = match guess.trim().parse::<i32>() {
            Err(_) => {
                println!("incorrect guess! please enter a number!");
                continue;
            },
            Ok(num) => { num + 1 }
        };
    
        match guess.cmp(&secret_number){
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => {
                println!("Too big!");
            }
        };
    }

}
