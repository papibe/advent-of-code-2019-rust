use std::fs;

const END: i32 = 99;
const SUM: i32 = 1;
const MUL: i32 = 2;
const CPY: i32 = 3;
const OUT: i32 = 4;

const POSITION_MODE: i32 = 0;
const IMMEDIATE_MODE: i32 = 1;

fn solution(filename: &str, input: i32) -> i32 {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // convert file content to a vector of integers
    let mut program: Vec<i32> = data.split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let mut output: Vec<i32> = Vec::new();
    let mut counter: i32 = 0;

    // run program
    let mut pointer: usize = 0;
    while program[pointer] != END {
        let operation: i32 = program[pointer] % 100;
        let parameters: i32 = program[pointer] / 100;

        let first_parameter_mode: i32 = parameters % 10;
        let parameters: i32 = parameters / 10;
        let second_parameter_mode: i32 = parameters % 10;
        let parameters: i32 = parameters / 10;
        let third_parameter_mode: i32 = parameters % 10;

        // sanity check
        if third_parameter_mode == IMMEDIATE_MODE {
            println!("Something's wrong!");
        }

        counter += 1;
        if counter == 100000 {
            break;
        }

        if operation == CPY {
            let dest_idx = program[pointer + 1] as usize;
            program[dest_idx] = input;

            pointer += 2;
        } else if operation == OUT {
            let op_idx = program[pointer + 1] as usize;
            output.push(program[op_idx]);

            pointer += 2;
        } else {
            let operand1: i32;
            let operand2: i32;
            let result_idx: usize;

            if first_parameter_mode == POSITION_MODE {
                let op1_idx = program[pointer + 1] as usize;
                operand1 = program[op1_idx];
            } else {
                operand1 = program[pointer + 1];
            }
            if second_parameter_mode == POSITION_MODE {
                let op2_idx = program[pointer + 2] as usize;
                operand2 = program[op2_idx];
            } else {
                operand2 = program[pointer + 2];
            }
            if third_parameter_mode == POSITION_MODE {
                result_idx = program[pointer + 3] as usize;
            } else {
                println!("something went wrong!");
                return output[output.len() - 1];
            }
            
            if operation == SUM {
                program[result_idx] = operand1 + operand2;
            } else if operation == MUL {
                program[result_idx] = operand1 * operand2;
            } else {
                println!("bad operation {}", operation);
                return output[output.len() - 1];
            }
            pointer += 4;
        }
    }

    output[output.len() - 1]
}

fn main() {
    println!("{:?}", solution("./input.txt", 1));   // 16574641
}
