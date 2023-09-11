//4. Define an enum `Fruit` with three variants `Apple`, `Orange`, `Banana`
//Each variant is associated with `u32` -> number of fruits
//Implement method `total` returning the total number of fruits
//Implement method `describe` -> describes fruit and its count


#[derive(Debug)]
enum Fruit {

    Apple(i32),
    Orange(i32),
    Banana(i32),
}

impl Fruit {

    fn value(&self) -> i32 {

        match self {
            //we deference variants as no is a reference to the `i32`, not the actual `i32`
            //value borrows reference to `self`, returning an instance of `Fruit` and its associated values, leading to `&i32`
            Fruit::Apple(no) => *no,
            Fruit::Orange(no) => *no,
            Fruit::Banana(no) => *no,
        }        
    }

    fn total(n1: &Fruit, n2: &Fruit, n3: &Fruit) -> i32 {

        n1.value() + n2.value() + n3.value()

    }

    fn describe(&self) -> String {

        match self {

            Fruit::Apple(no) => format!("There are {} apples", no),
            Fruit::Orange(no) => format!("There are {} oranges", no),
            Fruit::Banana(no) => format!("There are {} bananas", no),
        }
    }

}



fn main() {
    let x = Fruit::Apple(20);
    let y = Fruit::Orange(14);
    let z = Fruit::Banana(61);
    println!("{:?}", Fruit::total(&x, &y, &z));

    println!("{}", x.describe());
    println!("{}", y.describe());
    println!("{}", z.describe());
}
