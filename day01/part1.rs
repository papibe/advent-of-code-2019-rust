use std::fs;

fn solution(filename: &str) -> i32 {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // cycle over lines and add fuel according to formula
    let mut total_fuel = 0;
    for module in data.lines() {
        let mass = module.parse::<i32>().unwrap();
        total_fuel += mass / 3 - 2;
    }
    total_fuel
}

fn main() {
        println!("{}", solution("./src/day01/input.txt"));    // 3268951
}
