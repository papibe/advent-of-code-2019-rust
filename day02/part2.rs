use std::fs;

const END: i32 = 99;
const SUM: i32 = 1;
const MUL: i32 = 2;
const OUTPUT: i32 = 19690720;

fn run(program: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
    // patch program with noun and verb
    program[1] = noun;
    program[2] = verb;

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

fn solution(filename: &str) -> i32 {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // convert file content to vector of integers
    let program: Vec<i32> = data.split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    // look all possible values of noun and verb (0 to length of program)
    let len = program.len();
    for noun in 0..len {
        for verb in 0..len {
            if run(&mut program.to_vec(), noun as i32, verb as i32) == OUTPUT {
                return 100 * (noun as i32) + (verb as i32);
            }
        }
    }
    panic!("No soulion found for noun and verb in range 0 to {}", len);
}

fn main() {
    println!("{}", solution("./input.txt"));    // 5485
}
