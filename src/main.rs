mod day1;

use std::io::{stdin};

fn get_input(prompt: &str) -> Vec<String> {
    println!("{}", prompt);
    let mut inputs: Vec<String> = Vec::new();
    let mut line = String::new();
    while stdin().read_line(&mut line).is_ok() {
        let l = line.trim().to_string();
        if l.len() <= 0 || l == "\n" {
            break;
        }
        inputs.push(l);
        line = String::new();
    }

    return inputs;
}

fn main() {
    let s = get_input("enter input: ");
    let r = day1::part2(s);
    println!("result: {}", r);
}
