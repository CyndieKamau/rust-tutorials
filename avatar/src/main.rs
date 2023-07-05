#![allow(unused)]
use std::io::{self, Write};
/*
Fun avatar game to learn more on Rust!

Created by Cyndie.

The game is created on the go; no thought process!

Have fun, but most importantly, learn Rust! :)
 */

fn main() {

    /*
    This function greets you,asks for your name, stores it in `your_name` var.
    io::Result<()> -> fn returns a Result type from the std::io module.
    The Result type is an enum that is used for error handling in Rust. It can either be Ok,or Err. 
     */


    fn welcome() -> io::Result<()> {
        println!("Hey there! What is your name?");

        //your_name var will store your input(name)
        let mut your_name = String::new();

        let stdin = io::stdin();

        stdin.read_line(&mut your_name);
        println!("Welcome to the game {} ", your_name);


        Ok(()) //fn ran properly without errors

    }
    
    welcome();

    fn choose_samurai() {
        let mut samurais: Vec<&str> = vec!["Ada", "Vita", "Uzi", "Doa"];
        let mut samurais_ages: Vec<int> = vec![30, 25, 19, 27 ];
        let mut about_samurais: Vec<&str> = vec![
            r#"Ada is the squad leader.
        She is a fierce fighter known as the queen of death"#,
            r#"Vita was once the squad leader, but got kicked out.
        She is now a lone wolf; legendary mercenary."#,
            r#"Uzi grew up, trained on the streets.
        She is called the Queen of the Underworld."#,
            r#"Doa lives and breathes in the shadows.
        She strikes when no one is watching"#,
        ];

        

        println!("There are 4 samurai warriors currently. Choose one to learn about them. ");

        for (index, samurai) in samurais.iter().enumerate() {
            println!("{}. {}", index + 1, samurai);
        }

    }

    choose_samurai();

    

}


