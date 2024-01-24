
fn main() {
    let x: (u32, u32) = (1u32, 2u32); // tuple
    let x: (u32, &str) = (1u32, "hello"); // mixed tuple
    let x: (u32,) = (1,); // single tuple
    let x: ((i32, i32), (i32, i32)) = ((1, 2), (3, 4)); // nested tuple
    println!("x is {:?}", x); // can print a tuple with less than or equal to 12 elements

    let (x, y, z) = (1, 2, 3); // destructuring
    println!("x is {}", x);

    let x = (1, 2, 3);
    println!("x.0 is {}", x.0); // accessing tuple elements
}