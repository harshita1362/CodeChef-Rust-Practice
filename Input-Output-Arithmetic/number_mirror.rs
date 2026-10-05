// Number Mirror
// Description: Write a program that takes a number N as input and prints it to the output.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.trim());
}
