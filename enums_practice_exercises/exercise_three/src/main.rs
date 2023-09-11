//3. Define an enum `Transport` with four variants;
// `Car` -> speed(i32), brand(String),
// `Plane` -> altitude(i32),
// `Boat` -> no of passengers(i32)
//use match to implement method `info` on all four variants of the enum
//use if let -> method `is_plane` to check if `Transport` is plane or not  


#[derive(Debug)]
enum Transport {

    Car(i32, String),
    Plane(i32),
    Boat(i32),
    Foot,
}


impl Transport {

    fn info(&self) -> String {

        match self {

            Transport::Car(speed, brand) => format!("Car of {} brand was traveling at {} km/hr", brand, speed),
            Transport::Plane(altitude) => format!("This plane is flying at {} feet", altitude),
            Transport::Boat(passengers) => format!("This boat has {} passengers", passengers),
            Transport::Foot => "You are walking".to_string(),
        }
    }

    fn is_plane(&self) -> &'static str {

        if let Transport::Plane(_) = self {

            "It's a Plane!"
        } else {

            "Not a plane!"
        }
    }
}    

fn main() {
    let car = Transport::Car(80, "Toyota".to_string());
    println!("{}", car.info());
    println!("{:?}", car.is_plane());

    let plane = Transport::Plane(1000);
    println!("{}", plane.info());
    println!("{:?}", plane.is_plane());

    let boat = Transport::Boat(49);
    println!("{}", boat.info());

    let walk = Transport::Foot;
    println!("{:?}", walk.info());
}
