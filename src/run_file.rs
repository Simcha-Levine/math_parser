use crate::tree::exe_expression;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::println;

fn run(path: &str) {
    for (index, line) in all_lines(path).unwrap().iter().enumerate() {
        match exe_expression(line) {
            Ok(num) => println!("{} = {num}", line),
            Err(err) => println!("\x1b[31mline {index} {err}\x1b[0m"),
        }
    }
}
fn all_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
