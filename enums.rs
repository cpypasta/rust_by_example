#![allow(dead_code)]

// an enum can have any valid struct type; values are referred to as variants
#[derive(Debug)]
enum Employee {
    Person(String, u8),
    Address { city: String, state: String },
    Fired,
    Hired,
}
type E = Employee; // alias for enum

// methods can be attached to an enum
impl Employee {
    fn is_active(&self) -> bool {
        match self {
            Employee::Fired => false,
            Employee::Hired => true,
            _ => false,
        }
    }
}

// each enum variant also has a discriminator (value)
enum Number {
    Zero, // implicit starting at 0
    One,
    Two,
}
enum Color {
    Red = 0xff0000, // explicit
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let status = Employee::Hired;
    let new_status = Employee::Fired;
    let person = Employee::Person("John".to_string(), 32);
    let address = Employee::Address { city: "Boston".to_string(), state: "MA".to_string() };
    println!("Status: {:?}, Person: {:?}, Address: {:?}", status, person, address);

    match new_status {
        Employee::Fired => println!("Fired"),
        Employee::Hired => println!("Hired"),
        _ => println!("Unknown"),
    }

    println!("alias {:?}", E::Hired);
    
    println!("is active? {}", status.is_active());

    println!("zero is {}", Number::Zero as i32);
    println!("roses are #{:06x}", Color::Red as i32);
}