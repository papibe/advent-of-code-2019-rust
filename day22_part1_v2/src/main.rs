use regex::Regex;
use std::fs;

#[derive(Debug)]
enum ShuffleTechnique {
    NewStack,
    Cut,
    Deal,
}

fn parse(filename: &str) -> Vec<(ShuffleTechnique, i32)> {
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    let new_stack_re = Regex::new(r"^deal into new stack$").unwrap();
    let stack_incr_re = Regex::new(r"^deal with increment (?<value>\d+)$").unwrap();
    let cut_re = Regex::new(r"^cut (?<value>-{0,1}\w+)$").unwrap();

    let mut instructions: Vec<(ShuffleTechnique, i32)> = vec![];

    for line in data.lines() {
        let (instruction, value) = if let Some(_ins) = new_stack_re.captures(line) {
            (ShuffleTechnique::NewStack, 0)
        } else if let Some(ins) = stack_incr_re.captures(line) {
            (ShuffleTechnique::Deal, ins["value"].parse::<i32>().unwrap())
        } else if let Some(ins) = cut_re.captures(line) {
            (ShuffleTechnique::Cut, ins["value"].parse::<i32>().unwrap())
        } else {
            panic!("what the what!");
        };

        instructions.push((instruction, value));
    }

    instructions
}

fn solve(instructions: &Vec<(ShuffleTechnique, i32)>, size: usize, card: usize) -> i32 {
    let mut card_position: usize = card;

    for (instruction, value) in instructions {
        match instruction {
            ShuffleTechnique::NewStack => card_position = size - 1 - card_position,
            ShuffleTechnique::Deal => card_position = (card_position * (*value as usize)) % size,
            ShuffleTechnique::Cut => {
                let increment: usize = if *value < 0 {
                    size - value.abs() as usize
                } else {
                    *value as usize
                };
                if card_position < increment as usize {
                    card_position = size - increment + card_position;
                } else {
                    card_position = card_position - increment;
                }
            }
        }
    }
    card_position as i32
}

fn solution(filename: &str, size: usize, card: usize) -> i32 {
    let instructions = parse(filename);
    solve(&instructions, size, card)
}

fn main() {
    println!("{}", solution("./input.txt", 10007, 2019)); // 3074
}
