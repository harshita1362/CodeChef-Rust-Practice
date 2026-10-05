// Multiple Choice Question
// Description: What will the following Rust code return as the output?
// Answer: The code will lead to a compilation error.
// Reason: x is declared as i32, but "hello" is a string literal.

fn main() {
    let x: i32 = "hello";
    println!("{}", x);
}
