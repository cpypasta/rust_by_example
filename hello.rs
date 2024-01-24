// this is a comment
fn main() {
    println!("Hello World!"); // println! is a macro
    println!("I'm a Rustacean!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

// can be compiled with rustc hello.rs
// can be run with ./hello

/*
 * this is a multiline comment
 * 
 the * is optional
 */