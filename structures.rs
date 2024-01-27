#![allow(dead_code)] // allow unused code

// DIFFERENT TYPE OF STRUCTS

// unit struct (no fields)
#[derive(Debug)]
struct Name;

// tuple struct
#[derive(Debug)]
struct Age(String, u8);

// field struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// nested struct
#[derive(Debug)]
struct Entry {
    person: Person,
    address: String,
}

fn main() {
    // instantiate unit struct
    let name = Name;
    println!("Name: {:?}", name);

    // instantiate tuple struct
    let age = Age(String::from("John"), 32);
    println!("Age: {:?}, {}", age, age.1); // like a normal tuple reference

    // instantiate field struct
    let _person = Person {
        name: String::from("John"),
        age: 32,
    };
    let name = String::from("John");
    let age = 32;
    let person = Person { name, age }; // shorthand; variable names must match field names
    println!("Person: {:?}, {}", person, person.age);

    // instantiate nested struct
    let entry = Entry {
        person: person,
        address: String::from("123 Main St."),
    };
    println!("Entry: {:?}, {}", entry, entry.person.age);

    // destructured struct
    let Person { name: my_name, age: my_age } = Person { name: "Brian".to_string(), age: 54 };
    println!("Destructured Person: {}, {}", my_name, my_age);
    let Age(name, age) = Age(String::from("John"), 32);
    println!("Destructured Age: {}, {}", name, age);

    // struct update syntax
    let other_person = Person { name: "John".to_string(), age: 32 };
    let person = Person { name: "Brian".to_string(), ..other_person };
    println!("Updated Person: {:?}", person);
}