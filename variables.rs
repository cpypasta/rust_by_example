fn main() {
    let my_number = 10; // variable type is inferred
    let _unused_value = "value"; // prefix with _ to avoid unused variable warning
    let mut my_new_number = 5; // mutable variable
    my_new_number = 6;

    // variables are constrained or available in a block which is referred to as their scope
    {
        let my_name = "John";
        println!("my name is {}", my_name);
    } // end of scope, and the variable my_name is no longer available

    // variables can be reusued inside of a block, which is referred to as variable shadowing
    let my_name = "Jane";
    {
        let my_name = "Frank";
        println!("my name inside the block is {}", my_name);
    }
    println!("my name outside the block is {}", my_name);

    // you can declare a variable without assigning a value (not usually done)
    let my_name;
    my_name = "John";
}