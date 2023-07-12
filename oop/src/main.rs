use std::io;

fn student() -> (String, String) {

    println!("Hello what is your name?");

    let mut your_name = String::new();

    io::stdin().read_line(&mut your_name).expect("Unable to read file");

    your_name = your_name.trim().to_string();


    println!("Which house are you in? ");

    let mut your_house = String::new();

    io::stdin().read_line(&mut your_house).expect("Unable to read file");

    your_house = your_house.trim().to_string();

    (your_name, your_house)



}



fn main() {
    let (your_name, your_house) = student();

    println!("Your name is {} from {}", your_name, your_house);
    
}
