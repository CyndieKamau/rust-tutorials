use std::io;

use std::cmp::Ordering;

use rand::Rng;



fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        print!("Type in your guess ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)

            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue

        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations you guessed it right!! The answer was {}!", secret_number);
                break;
            }
        }
    } 
}
