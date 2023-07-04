/*

Rust tutorial for practice
By Cyndie :)
 */
#![allow(unused)]

const GLOBAL: &str = "constants are global vars";
const BIRTHDAY: i32 = 1_i32;

fn main() {
    println!("Hello, world!");  // 1. println! is a macro

    // 2. PRIMITIVES -> Scalar types; int, float, bool, char
    // int -> unsigned int(u8, u16, u32..) signed int(i8, i16, i32..). DEfault is i32
    println!("Max number of u32: {}", u32::MAX);
    println!("Max number of u64: {}", u64::MAX);

    //float -> f64 is default, but slower than f32.
    println!("Max number of f32: {}", f32::MAX);
    println!("Max number of f64: {}", f64::MAX);

    let x = true;   //bool type can be written as just "true" or "false"
    println!("is x true or false? {}", x);


    // VARIABLES

    //vars are immutable in rust
    let hello = "hello world";
    println!("{}", hello);     // prints the string without newline character at end

    // mut is to ensure vars are mutable
    let mut hello2 = "hello world again";
    println!("{}", hello2);

    hello2 = "hello once more";
    println!("{}", hello2);

    // CONSTANTS -> have to be in UPPERCASE. Are global (occur outside of function)

    println!("{}", GLOBAL);


    // SUFFIXES -> u32, u64, u8 etc

    let x = 20_u32;
    let y = 1_000;

    println!("{} + {} = {}", x, y, x+y);

    // BIRTHDAY QUIZ

    birthday();




}

 /* Challenge 1 - Build a program that has the following:

    1) Has a global constant integer named 'birthday' with a value of 1
    2) Has a local string variable named 'my_name' with your name as the value
    3) Has a local string variable named 'my_birthday' with your birth month/day (no year) as the value
    4) Has a local mutable integer variable named 'age' with your current age as the value
    5) Has a local integer variable named 'new_age' with your age after your birthday as the value
    6) Prints out 'My name is X and I am X years old. I will turn X on X' 

*/

fn birthday() {
    let my_name = "Cyndie";
    let my_birthday: &str = "13th October";
    let mut age = 23_i32;
    let new_age = age + BIRTHDAY;

    println!("My name is {} and I am {} years old. I will turn {} on {} this year", my_name, age, new_age, my_birthday);

}
