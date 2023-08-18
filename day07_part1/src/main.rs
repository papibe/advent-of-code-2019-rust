use std::fs;

const SUM: i32 = 1;
const MUL: i32 = 2;
const CPY: i32 = 3;
const OUT: i32 = 4;
const JIT: i32 = 5;
const JIF: i32 = 6;
const LTH: i32 = 7;
const EQL: i32 = 8;
const END: i32 = 99;

const POSITION_MODE: i32 = 0;
const IMMEDIATE_MODE: i32 = 1;

fn parse(filename: &str) -> Vec<i32> {
    // read file
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    // convert content into a vector of integers
    let program: Vec<i32> = data
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    program
}

fn parse_instruction(instruction: i32) -> (i32, i32, i32, i32) {
    let operation: i32 = instruction % 100;
    let parameters: i32 = instruction / 100;

    let first_parameter_mode: i32 = parameters % 10;
    let parameters: i32 = parameters / 10;
    let second_parameter_mode: i32 = parameters % 10;
    let parameters: i32 = parameters / 10;
    let third_parameter_mode: i32 = parameters % 10;

    (
        operation,
        first_parameter_mode,
        second_parameter_mode,
        third_parameter_mode,
    )
}

fn solve(program: &mut Vec<i32>, input: Vec<i32>) -> i32 {
    let mut output: Vec<i32> = Vec::new();
    let mut pointer: usize = 0;
    let mut input_pointer: usize = 0;

    // run program
    while program[pointer] != END {
        let operation: i32;
        let first_parameter_mode: i32;
        let second_parameter_mode: i32;
        let third_parameter_mode: i32;

        (
            operation,
            first_parameter_mode,
            second_parameter_mode,
            third_parameter_mode,
        ) = parse_instruction(program[pointer]);

        // sanity check
        if third_parameter_mode == IMMEDIATE_MODE {
            println!("blah!");
        }

        let operand1: i32;
        let operand2: i32;
        let result_idx: usize;

        // operations with 1 parameter: CPY and OUT
        if [CPY, OUT].contains(&operation) {
            let dest_idx = program[pointer + 1];
            let to_print: i32;

            if operation == CPY {
                program[dest_idx as usize] = input[input_pointer];
                input_pointer += 1;
            } else if operation == OUT {
                if first_parameter_mode == POSITION_MODE {
                    to_print = program[dest_idx as usize];
                } else {
                    to_print = dest_idx;
                }
                // println!("pointer at output: {}", pointer);
                output.push(to_print);
            }
            pointer += 2;
            continue;
        }

        // operations with 2 paramters: JIT and JIF
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

        if operation == JIT {
            if operand1 != 0 {
                pointer = operand2 as usize;
                continue;
            }
            pointer += 3;
            continue;
        } else if operation == JIF {
            if operand1 == 0 {
                pointer = operand2 as usize;
                continue;
            }
            pointer += 3;
            continue;
        }

        result_idx = program[pointer + 3] as usize;

        // operations with 3 parameters: SUM, MUL, LTH, and EQL
        if operation == SUM {
            program[result_idx] = operand1 + operand2;
        } else if operation == MUL {
            program[result_idx] = operand1 * operand2;
        } else if operation == LTH {
            if operand1 < operand2 {
                program[result_idx] = 1;
            } else {
                program[result_idx] = 0;
            }
        } else if operation == EQL {
            if operand1 == operand2 {
                program[result_idx] = 1;
            } else {
                program[result_idx] = 0;
            }
        }
        pointer += 4;
    }
    // println!("pointer at the end {}", pointer);
    output[output.len() - 1]
}

fn permutations(numbers: &mut Vec<i32>) -> Vec<Vec<i32>> {
    
    fn dfs(index: usize, numbers: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == numbers.len() {
            // println!("{:?}", numbers);
            result.push(numbers.clone());
        }
        for i in index..numbers.len()  {
            let temp: i32 = numbers[index];
            numbers[index] = numbers[i];
            numbers[i] = temp;

            dfs(index + 1, numbers, result);

            let temp: i32 = numbers[index];
            numbers[index] = numbers[i];
            numbers[i] = temp;
        }
    }
    
    let mut result = Vec::new();
    dfs(0, numbers, &mut result);
    result
}

fn solution(filename: &str) -> i32 {
    // parse file
    let mut program: Vec<i32> = parse(filename);

    let mut phases: Vec<i32> = [0, 1, 2, 3, 4].to_vec();
    let perms: Vec<Vec<i32>> = permutations(&mut phases);
    let mut max_output: i32 = -1;
    for phases in perms {
        let mut previous_output: i32 = 0;
        for phase in phases {
            let input: Vec<i32> = [phase, previous_output].to_vec();
            previous_output = solve(&mut program, input);
        }
        if previous_output > max_output {
            max_output = previous_output;
        }
    }

    max_output
}

fn main() {
    println!("{:?}", solution("./example1.txt"));   // 43210
    println!("{:?}", solution("./input.txt"));  // 34852
}
