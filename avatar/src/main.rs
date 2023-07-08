
/*
Fun avatar game to learn more on Rust!

Created by Cyndie.

The game is created on the go; no thought process!

Have fun, but most importantly, learn Rust! :)
 */


use std::io; //prelude, if a type you want is not in the prelude you have to bring that type into scope explicitly using the "use" statement

//helper functions - to be used inside our main logic function, dividing the problem into sub modules

fn welcome() {

    /*
    This function greets you,asks for your name, stores it in `your_name` var.
    io::Result<()> -> fn returns a Result type from the std::io module.
    The Result type is an enum that is used for error handling in Rust. It can either be Ok,or Err. 
        */        


    println!("Hey there! What is your name?");

    //Initialize an empty string to store the user's input, `your_name`
    let mut your_name = String::new();

    let stdin = io::stdin();

    // Read a line from the standard input and store it in `your_name`.
    stdin.read_line(&mut your_name)
            /*returns a  result value, either Ok or Err, Ok indicates 
            Operation was successful, Err failed
            if "Ok", you will get back the value the user typed if ERR you will get an error messages that you defined in the expect
            if you dont call expect you will get a warning
            */

        .expect("Failed to read line");
    println!("Welcome to the game {} ", your_name);





    }



fn choose_samurai(){

    /*
    This function is for outputting info about our 4 samurais; Ada, Vita, Uzi and Doba.

    */

    // Stored info about the samurais as vectors so we can add more later. Arrays in rust are fixed length.
    let samurais: Vec<&str> = vec!["Ada", "Vita", "Uzi", "Doa"];
    let samurais_ages: Vec<i32> = vec![30, 25, 19, 27 ];
    let about_samurais: Vec<&str> = vec![
        r#"Ada is the squad leader.
    She is a fierce fighter known as the queen of death"#,
        r#"Vita was once the squad leader, but got kicked out.
    She is now a lone wolf; legendary mercenary."#,
        r#"Uzi grew up, trained on the streets.
    She is called the Queen of the Underworld."#,
        r#"Doa lives and breathes in the shadows.
    She strikes when no one is watching"#,
    ];

    // Begin an infinite loop. The user can only exit this loop by typing "exit".
    loop {

        // Print the list of samurais and prompt for user input.
        println!("");
        println!("There are 4 samurai warriors currently. Choose one to learn about them.");
        println!("Type 'exit' to quit");

            // Iterate over the `samurais` vector using `enumerate`.
            // We use `(index+1)` so user sees `1. Ada`, instead of index[0] Ada -> 0. Ada
        for (index, samurai) in samurais.iter().enumerate() {
            println!("{}. {}", index + 1, samurai);
        }

        let mut your_choice = String::new();

        let stdin = io::stdin();

        stdin.read_line(&mut your_choice)

            /*returns a  result value, either Ok or Err, Ok indicates Operation was successful, Err failed
            if "Ok", you will get back the value the user typed if ERR you will get an error messages that you defined in the expect
            if you dont call expect you will get a warning
            */
            .expect("Error reading line");

        if your_choice.trim() == "exit" {
            println!("See you again, buddy!");
            break;
        }

        // Try to parse the user's choice as a `usize` (a non-negative integer). This is necessary 
        // because the input comes in as a string, but we need an integer to index into our vector.

        match your_choice.trim().parse::<usize>() {

            // If the parsing succeeds and the parsed no is between 1 and 4 (no of samurais),
            // print samurai's info. Note the use of `n-1` as the index because 
            // the user input is 1-based while the indexing is 0-based.
            Ok(n) if n > 0 && n <= samurais.len() => {

                println!("You chose {}: Age {}, About: {}", samurais[n-1], samurais_ages[n-1], about_samurais[n-1]);

            },

            _ => println!("Invalid choice! Please enter a number between 1 and {} :(", samurais.len()),

        }

    }




}



fn main() {
    //now we use the helper functions above in the main logic    
    welcome();
    choose_samurai();

}


