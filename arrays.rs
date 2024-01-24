use std::mem;

fn main() {
    let x: [i32; 3] = [9; 3]; // array of 3 elements with all values of 9
    let x: [i32; 3] = [1, 2, 3]; // array of fixed 3 values
    println!("x is {:?}", x);
    println!("x[0] is {}", x[0]); // accessing array at index
    println!("x length is {}", x.len()); // array length
    println!("x occupies {} bytes", mem::size_of_val(&x)); // array size fixed on stack
    println!("borrow whole array as slice: {:?}", &x);
    println!("borrow whole array as slice: {:?}", &x[..]); // all elements
    println!("borrow part of array as slice: {:?}", &x[0..1]); // first element
    println!("borrow part of array as slice: {:?}", &x[..2]); // first two elements
    println!("borrow part of array as slice: {:?}", &x[1..]); // last two elements

    let a: Option<&i32> = x.get(1); // get value as Option
    match a {
        Some(val) => println!("a is {}", val),
        None => println!("a is None"),
    }
}