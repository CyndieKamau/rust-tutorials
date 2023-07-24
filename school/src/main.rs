use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Parent {
    name: String,
    age: u32,
    child_name: String,
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    class: u32,
    parent_name: String,
}

fn main() {
    let mut parents: HashMap<String, Parent> = HashMap::new();
    let mut students: HashMap<String, Student> = HashMap::new();

    loop {
        println!("1. Create a parent\n2. Create a student\n3. Exit");
        let choice: u32 = read_input("Enter your choice").trim().parse().unwrap();
        match choice {
            1 => {
                let name = read_input("Enter parent's name");
                let age: u32 = read_input("Enter parent's age").trim().parse().unwrap();
                let child_name = read_input("Enter child's name");
                let parent = Parent { name: name.clone(), age, child_name };
                parents.insert(name, parent);
            }
            2 => {
                let name = read_input("Enter student's name");
                let age: u32 = read_input("Enter student's age").trim().parse().unwrap();
                let class: u32 = read_input("Enter student's class").trim().parse().unwrap();
                let parent_name = read_input("Enter parent's name");
                let student = Student { name: name.clone(), age, class, parent_name };
                students.insert(name, student);
            }
            3 => break,
            _ => println!("Invalid choice!"),
        }
    }

    println!("Parents: {:?}", parents);
    println!("Students: {:?}", students);
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
