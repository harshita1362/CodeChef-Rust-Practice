// CodeChef Learn Problem Solving
// For each language, there are 2 courses.
// Therefore, for N languages, the total number of courses is 2 * N.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    println!("{}", 2 * n);
}
