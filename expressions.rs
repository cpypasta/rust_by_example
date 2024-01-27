#![allow(unused_variables)]
#![allow(unused_must_use)]

fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    // block expression
    let x = {
        let y = 1;
        let z = 2;
        y + z // no semi-colon here so it returns this value
    }; // semi-colon here
}