//Creating a simple Cli based calculator
//will handle basic operations add, subtract multiply divide etc
//Welcome to calculator! If add press 1, if substract press 2 if divide press 3 if multiply press 4
//exit press 5

use std::{io, num::ParseIntError};

fn parse_user_input() -> u32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u32 = user_input.trim().parse().expect("This is not a number");

    user_input
}

fn main() {
    println!("Hello, welcome to simple calculator!");

    println!("Addition: Press 1\nSubtraction: Press 2\nMultiplication: Press 3\nDidivsion: Press 4\nExit: Press 5");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    //user_input is a string, convert to int
    let user_input: Result<u32, ParseIntError> = user_input.trim().parse();

    match user_input {
        Ok(x) => match x {
            1 => {
                println!("Type all the numbers you need to add");

                let x = parse_user_input();

                println!("Input your second number");

                let y = parse_user_input();

                let z: u32 = x + y;

                println!("{x} + {y} = {z}");
            }
            2 => {
                println!("Input your first number");

                let x = parse_user_input();

                println!("Input your second number");

                let y = parse_user_input();

                if x >= y {
                    let z = x - y;

                    println!("{x} - {y} = {z}");
                } else {
                    println!("Your second no is greater than first!");
                }
            }
            3 => {
                println!("Input your first number");

                let x = parse_user_input();

                println!("Input your second number");

                let y = parse_user_input();

                let z = x * y;

                println!("{x} * {y} = {z}");
            }
            4 => {
                println!("Input your first number");

                let x = parse_user_input();

                println!("Input your second number");

                let y = parse_user_input();

                let z = x / y;

                println!("{x} / {y} = {z}");
            }
            5 => 'block: {
                println!("Thank you for using the calculator");
                break 'block;
            }
            _ => {
                println!("Please choose a number between 1 and 5");
            }
        },
        Err(_) => {}
    }
}

