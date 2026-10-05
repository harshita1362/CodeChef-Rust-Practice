// Chef and Instant Noodles
// Chef has X stoves and each packet takes exactly 1 minute to cook.
// In Y minutes, each stove can cook Y packets.
// Therefore, the maximum number of packets (and customers) is X * Y.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let x: i32 = values.next().unwrap().parse().unwrap();
    let y: i32 = values.next().unwrap().parse().unwrap();

    println!("{}", x * y);
}
