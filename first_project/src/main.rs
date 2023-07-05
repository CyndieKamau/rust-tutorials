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



// COMPOUND TYPES -> Tuples, Arrays

// 1. Tuples -> store multiple values in a var
// N.B Max value to be stored in a tuple is 12

let student_a = ("Cyndie", 'A', 3.57);

//let name_of_student_a = student_a.0;
//let grade_of_student_a = student_a.1;
//let gpa_of_student_a = student_a.2;

// To write the let statements above in one line of code;

let(name_of_student_a, grade_of_student_a, gpa_of_student_a) = student_a;

println!("{} had a GPA of {} after scoring an {}", name_of_student_a, gpa_of_student_a, grade_of_student_a);

// can also use dot notation

println!("{} had a GPA of {} after scoring an {}", student_a.0, student_a.2, student_a.1);



// ARRAYS -> Store upto 32, can only store same data types (int, str, char or float) uses bracket notation

let students = ["Cyndie", "John", "Anita", "Sam"];
let students_grades = ['A', 'D', 'B','C' ];

println!("The first student in our class is {} with a grade of {}.", students[0], students_grades[0]);


// using a for loop to iterate 

for student in students.iter()  {
    println!("The students are {}", student);

}

// VECTORS -> Re-sizeable arrays;

let mut students2 = vec!["Cyndie", "John", "Anita", "Sam"];
let mut students2_grades = vec!['A', 'D', 'B', 'C'];
students2.push("David");
students2_grades.push('E');

//using a for loop;

for (student2, student2_grade) in students2.iter().zip(students2_grades.iter()) {

    println!("Student {} got this grade: {}", student2, student2_grade);
}








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
