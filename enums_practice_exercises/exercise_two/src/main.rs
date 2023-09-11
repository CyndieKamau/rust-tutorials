//2. Define an enum `Animal` with three variants;
// `Dog` with a name,
// `Cat` with age,
//`Bird`
//use `match` -> function `describe` that prints out dog's name, cat's age, or "it's a bird!" for bird
//use `if let` to check if animal is a cat. If not, print "Not a cat!"


#[derive(Debug)]
enum Animal {

    Dog(String),
    Cat(i32),
    Bird,

}


impl Animal {

    fn describe(&self) -> String {

        match self {

            Animal::Dog(name) => format!("Dog name: {}", name),
            Animal::Cat(age) => format!("Cat age: {}", age),
            Animal::Bird => "It's a bird!".to_string(),

        }
    }
    
    fn is_cat(&self) -> &'static str {

       if let Animal::Cat(_) = self {

           "Is a cat!"
       } else {

           "Not a cat!"
       }
    }

}


fn main() {
    let animal1 = Animal::Dog("Jimmy".to_string());
    println!("{}", animal1.describe());
    println!("{:?}", animal1.is_cat());

    let animal2 = Animal::Cat(5);
    println!("{}", animal2.describe());
    println!("{:?}", animal2.is_cat());

}
