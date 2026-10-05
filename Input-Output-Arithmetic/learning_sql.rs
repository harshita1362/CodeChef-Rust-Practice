// Learning SQL
// Chef has a table with R rows and C columns.
// He adds E extra rows and needs the final total number of cells.
// The final number of rows is R + E, so the answer is (R + E) * C.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let values: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let r = values[0];
    let c = values[1];
    let e = values[2];

    println!("{}", (r + e) * c);
}
