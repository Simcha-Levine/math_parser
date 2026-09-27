use std::io::{self, BufRead};
// use std::{
//     io::{self, Read},
//     println,
// };
use crate::tree::get_expression;
use std::println;

mod eval;
mod mid_token;
mod node;
mod run_file;
mod test;
mod token;
mod token_types;
mod tree;

fn main() {
    run_file::run("exp.txt");
}

#[allow(unused)]
fn io_run() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let expression = match line {
            Ok(line) => line.trim().to_string(),
            Err(error) => {
                eprintln!("Failed to read stdin: {error}");
                break;
            }
        };

        match get_expression(&expression) {
            Ok(tree) => println!("{}", tree.latex(false)),
            Err(error) => {
                eprintln!("{error}");
                println!("ERROR");
            }
        }
    }
}
