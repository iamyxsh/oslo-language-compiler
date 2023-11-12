use crate::scanner::scan_code;
use crate::scanner::token::Token;
use std::io;
use std::io::Read;

mod scanner;
mod utils;

fn main() {
    let mut input = String::new();
    println!("Enter an expression to evaluate");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    if input.eq_ignore_ascii_case("exit") {
        println!("Exiting øslo. Goodbye!");
    } else {
        match scan_code(input) {
            Ok(tokens) => println!("{:?}", tokens),
            Err(err) => println!("Something went wrong \n {err}"),
        };
    }
}
