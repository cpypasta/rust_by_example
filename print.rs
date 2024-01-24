fn main() {
    println!("{} days", 31); // {} is replaced
    println!("{0} happens before {1} and after {0}", "Alice", "Bob"); // can order values
    println!("{person} is {age} years old", person="Brian", age=23); // can name values
    println!("integer {first:>5}, float {second:>3.2}", 
             first=23, 
             second=1.235); // supports f-string like formatting
    println!("Base 16 (hex): {:X}", 1234567); // supports hex
    println!("YOu can even use a named parameter with f-strings: {number:0>width$}", number=1, width=4);

    // can capture argument from a surrounding variable
    let number: f64 = 3.141592;
    let width: usize = 3;
    println!("Pi is roughly {number:.width$}");
}