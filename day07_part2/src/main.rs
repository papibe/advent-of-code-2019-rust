use std::collections::VecDeque;
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

const N_AMPLIFIERS: usize = 5;

// Amplifier 'class'
struct Amplifier {
    _name: char,
    program: Vec<i32>,
    pointer: usize,
    halted: bool,
}

impl Amplifier {
    fn reset(&mut self, original_program: &Vec<i32>) {
        self.program.copy_from_slice(&original_program);
        self.pointer = 0;
        self.halted = false;
    }

    fn run(&mut self, buffers: &mut Vec<VecDeque<i32>>, amp_pointer: usize) {
        let input: usize = amp_pointer;
        let output: usize = (amp_pointer + 1) % N_AMPLIFIERS;

        // run program
        while self.program[self.pointer] != END {
            let operation: i32;
            let first_parameter_mode: i32;
            let second_parameter_mode: i32;
            let third_parameter_mode: i32;

            (
                operation,
                first_parameter_mode,
                second_parameter_mode,
                third_parameter_mode,
            ) = parse_instruction(self.program[self.pointer]);

            // sanity check
            if third_parameter_mode == IMMEDIATE_MODE {
                println!("blah!");
            }

            let operand1: i32;
            let operand2: i32;
            let result_idx: usize;

            // operations with 1 parameter: CPY and OUT
            if [CPY, OUT].contains(&operation) {
                let dest_idx = self.program[self.pointer + 1];
                let to_print: i32;

                if operation == CPY {
                    // check if there's something in the input
                    if buffers[input].len() == 0 {
                        return;
                    }
                    self.program[dest_idx as usize] = buffers[input].pop_front().unwrap();
                } else if operation == OUT {
                    if first_parameter_mode == POSITION_MODE {
                        to_print = self.program[dest_idx as usize];
                    } else {
                        to_print = dest_idx;
                    }
                    buffers[output].push_back(to_print);
                }
                self.pointer += 2;
                continue;
            }

            // operations with 2 paramters: JIT and JIF
            if first_parameter_mode == POSITION_MODE {
                let op1_idx = self.program[self.pointer + 1] as usize;
                operand1 = self.program[op1_idx];
            } else {
                operand1 = self.program[self.pointer + 1];
            }
            if second_parameter_mode == POSITION_MODE {
                let op2_idx = self.program[self.pointer + 2] as usize;
                operand2 = self.program[op2_idx];
            } else {
                operand2 = self.program[self.pointer + 2];
            }

            if operation == JIT {
                if operand1 != 0 {
                    self.pointer = operand2 as usize;
                    continue;
                }
                self.pointer += 3;
                continue;
            } else if operation == JIF {
                if operand1 == 0 {
                    self.pointer = operand2 as usize;
                    continue;
                }
                self.pointer += 3;
                continue;
            }

            result_idx = self.program[self.pointer + 3] as usize;

            // operations with 3 parameters: SUM, MUL, LTH, and EQL
            if operation == SUM {
                self.program[result_idx] = operand1 + operand2;
            } else if operation == MUL {
                self.program[result_idx] = operand1 * operand2;
            } else if operation == LTH {
                if operand1 < operand2 {
                    self.program[result_idx] = 1;
                } else {
                    self.program[result_idx] = 0;
                }
            } else if operation == EQL {
                if operand1 == operand2 {
                    self.program[result_idx] = 1;
                } else {
                    self.program[result_idx] = 0;
                }
            }
            self.pointer += 4;
        }
        self.halted = true;
    }
}

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

fn get_permutations(numbers: &mut Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(index: usize, numbers: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == numbers.len() {
            result.push(numbers.clone());
        }
        for i in index..numbers.len() {
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

fn all_amps_halted(amplifiers: &Vec<Amplifier>) -> bool {
    for amp in amplifiers {
        if !amp.halted {
            return false;
        }
    }
    true
}

fn solution(filename: &str) -> i32 {
    // parse file
    let original_program: Vec<i32> = parse(filename);
    let mut amplifiers: Vec<Amplifier> = Vec::new();
    let amp_names: Vec<char> = Vec::from(['A', 'B', 'C', 'D', 'E']);

    for name in amp_names {
        amplifiers.push(Amplifier {
            _name: name.clone(),
            program: original_program.clone(),
            pointer: 0,
            halted: false,
        });
    }

    let mut phases: Vec<i32> = [5, 6, 7, 8, 9].to_vec();
    let permutations: Vec<Vec<i32>> = get_permutations(&mut phases);
    let mut buffers: Vec<VecDeque<i32>> = Vec::new();
    let mut max_output: i32 = 0;

    for phase_perms in permutations {
        // reset amplifiers
        for amp in &mut amplifiers {
            amp.reset(&original_program);
        }
        // reset buffers
        buffers.clear();
        for i in 0..N_AMPLIFIERS {
            buffers.push(VecDeque::from([phase_perms[i]]));
        }
        buffers[0].push_back(0);

        let mut amp_pointer: usize = 0;
        while !all_amps_halted(&amplifiers) {
            amplifiers[amp_pointer].run(&mut buffers, amp_pointer);
            amp_pointer = (amp_pointer + 1) % N_AMPLIFIERS;
        }

        if buffers[0].len() >= 1 {
            let output = buffers[0].pop_front().unwrap();
            if output > max_output {
                max_output = output;
            }
        }
    }

    max_output
}

fn main() {
    println!("{}", solution("./example1.txt")); // 139629729
    println!("{}", solution("./example2.txt")); // 18216
    println!("{}", solution("./input.txt")); // 44282086
}
