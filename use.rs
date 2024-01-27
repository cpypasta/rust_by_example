#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Hired,
    Fired,
}
#[derive(Debug)]
enum Role {
    Manager,
    Engineer,
}

fn main() {
    use crate::Status::{Hired, Fired};
    use crate::Role::*;

    println!("Status: {:?}, Role: {:?}", Hired, Manager);
}