use std::fs;
use std::collections::HashMap;

fn is_valid(number: i32) -> bool {
    let s: String = number.to_string();
    let n: usize = s.len();
    let chars: Vec<_> = s.chars().collect();

    let mut counter = HashMap::from([
        ('0', 0),
        ('1', 0),
        ('2', 0),
        ('3', 0),
        ('4', 0),
        ('5', 0),
        ('6', 0),
        ('7', 0),
        ('8', 0),
        ('9', 0),
    ]);

    *counter.entry(chars[0]).or_insert(0) += 1;
    for index in 0..n - 1 {
        *counter.entry(chars[index + 1]).or_insert(0) += 1;
        if chars[index] as usize > chars[index + 1] as usize {
            return false;
        }
    }
    let mut adjacent_digits: i32 = 0;
    for (_digit, value) in &counter {
        if *value == 2 {
            adjacent_digits += 1;
        }
    }

    adjacent_digits >= 1
}

fn solution(filename: &str) -> i32 {
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));
    
    let range: Vec<i32> = data.split("-")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let start: i32 = range[0];
    let end: i32 = range[1];

    let mut valid_passwords: i32 = 0;
    for number in start..end + 1 {
        if is_valid(number) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn main() {
    println!("{}", solution("./input.txt"));    // 748
}
