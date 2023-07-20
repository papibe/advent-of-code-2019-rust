use std::fs;

const END: i32 = 99;
const SUM: i32 = 1;
const MUL: i32 = 2;

fn solution(filename: &str) -> i32 {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // convert file content to a vector of integers
    let mut program: Vec<i32> = data.split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    // patching
    program[1] = 12;
    program[2] = 2;

    // run program
    let mut pointer: usize = 0;
    while program[pointer] != END {
        let op1_idx = program[pointer + 1] as usize;
        let op2_idx = program[pointer + 2] as usize;
        let res_idx = program[pointer + 3] as usize;

        if program[pointer] == SUM {
            program[res_idx] = program[op1_idx] + program[op2_idx];
        } else if program[pointer] == MUL {
            program[res_idx] = program[op1_idx] * program[op2_idx];
        }
        pointer += 4;
    }
    program[0]
}

fn main() {
    println!("{}", solution("./input.txt"));    // 4570637
}
