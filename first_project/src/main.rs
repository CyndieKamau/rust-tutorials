/*

Rust tutorial for practice
By Cyndie :)
 */
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

    let hello2 = "hello once more";
    println!("{}", hello2);
}

