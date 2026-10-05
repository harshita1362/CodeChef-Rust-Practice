use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let values: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let x = values[0];
    let y = values[1];
    let h = values[2];

    // X is charged for the first hour.
    // Y is charged for each remaining hour.
    println!("{}", x + y * (h - 1));
}
