#![allow(unused_variables)]
#![allow(dead_code)]

use std::convert::{From, Into, TryFrom, TryInto};
use std::str::FromStr;
use std::fmt;

fn main() {
    /* CASTING: Rust does not do implicit type conversion (coercion) */

    let x = 10u32; // using suffux to specify type
    let y = x as u8; // using 'as' keyword to explicitly cast type

    let x = 2.55f32;
    let y = x as u8; // can cast float to int, but will truncate the decimal
    println!("x: {}, y: {}", x, y);

    let x = 1000_u16; // can add _ before suffix if you want
    let y = x as u8; // can cast int to smaller int, but will reduce number to fit
    println!("x_u16: {}, y_u8: {}, u8_max: {}", x, y, u8::MAX);

    // casting strings into numbers
    let x: u8 = "100".parse().unwrap();
    let y = "100".parse::<u8>().unwrap();


    /* ALIASING */

    type Byte = u8; // alias should be in UpperCamelCase
    let x: Byte = 255;
    println!("x: Byte = {}", x);


    /* FROM and INTO: From and Into are traits that allow for conversions between types */

    // provided types are provided
    println!("My name is {}", String::from("John"));
    let name: String = "John".into();
    println!("My name is {}", name);

    // can also support your own types
    #[derive(Debug)]
    struct Age {
        value: u8
    }
    impl From<u8> for Age {
        fn from (item: u8) -> Self {
            Age { value: item }
        }   
    }
    // supports converting anything into a string
    impl fmt::Display for Age {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Age_u8: {}", self.value)
        }
    }
    // supports going from string into Age
    impl FromStr for Age {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse::<u8>() {
                Ok(age) => Ok(Age { value: age }),
                Err(_) => Err(()),
            }
        }
    }

    #[derive(Debug, PartialEq)] // PartialEq is for assert equality testing
    struct Name(String);
    impl TryFrom<String> for Name {
        type Error = ();
        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.len() > 0 {
                Ok(Name(value))
            } else {
                Err(())
            }
        }             
    }

    let my_age = Age::from(30);
    println!("{}", my_age);
    let my_age: Age = 30.into();
    println!("{}", my_age);
    println!("my_age_as_string: {}", Age::from(54).to_string()); // provided by fmt::Display trait

    assert_eq!(Name::try_from(String::from("John")), Ok(Name(String::from("John"))));
    assert_eq!(Name::try_from(String::from("")), Err(()));
    assert_eq!(String::from("John").try_into(), Ok(Name(String::from("John"))));
    println!("{:?}", Name::try_from(String::from("John")).unwrap()); // unwrap() the Result to get the value

    println!("{}", "20".parse::<Age>().unwrap());
}