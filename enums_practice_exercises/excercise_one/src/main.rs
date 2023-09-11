//Simple exercises for building on pattern matching in enums in Rust

//1. Define an enum `Color` with three variants, `Red`, `Green`, `Blue`.
//Use `match` to write a function `to_hex` that takes a `Color` and returns its hex rep as strings
//Use `if let` to write a function `is_red` that returns `true` if the color is Red, `false` otherwise

#[derive(Debug)]
enum Color {

    Red,
    Green,
    Blue,
}


impl Color {

    fn to_hex(&self) -> &'static str {      //has a static lifetime since I have hardcoded the hex reps

        match self {

        Color::Red => "#880808",
        Color::Green => "#7FFFD4",
        Color::Blue => "#00FFFF",
        }
    }

    fn is_red(&self) -> bool {

        if let Color::Red = self {

            true
        } else {

        false
        }
    }

}




fn main() {
    let colour = Color::Red;
    println!("{:?}", colour.to_hex());


    let col2 = Color::Green;
    println!("{:?}", col2.is_red());

}
