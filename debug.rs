// derive the fmt::Debug implementation
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("Now {:?} will print!", Structure(23));

    let tracy = Person { name: "Tracy", age: 23 };
    println!("Pretty printed: {:#?}", tracy)
}