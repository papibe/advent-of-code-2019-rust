use std::fs;

fn solution(filename: &str) -> i32 {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // cycle over lines and add fuel according to formula
    data.lines()
        .map(|module| {
            let mass = module.parse::<i32>().unwrap();
            mass / 3 - 2
        })
        .sum()
}

fn main() {
        println!("{}", solution("./input.txt"));    // 3268951
}
