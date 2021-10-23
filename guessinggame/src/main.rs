use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess a number game");

    loop{
        
        println!("Please input a number");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(new) => new,
            Err(_)  => continue,
        };

        // generate a secret number 
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("You guessed {}", guess);

        println!("The secret number is {}", secret_number);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}