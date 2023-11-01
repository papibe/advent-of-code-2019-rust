use std::collections::{HashMap, VecDeque};

use intcode::{parse, IntcodeComputer};

const N_AMPLIFIERS: usize = 5;

fn get_permutations(numbers: &mut Vec<i64>) -> Vec<Vec<i64>> {
    fn dfs(index: usize, numbers: &mut Vec<i64>, result: &mut Vec<Vec<i64>>) {
        if index == numbers.len() {
            result.push(numbers.clone());
        }
        for i in index..numbers.len() {
            let mut temp: i64 = numbers[index];
            numbers[index] = numbers[i];
            numbers[i] = temp;

            dfs(index + 1, numbers, result);

            temp = numbers[index];
            numbers[index] = numbers[i];
            numbers[i] = temp;
        }
    }

    let mut result: Vec<Vec<i64>> = Vec::new();
    dfs(0, numbers, &mut result);
    result
}

fn all_amps_halted(amplifiers: &Vec<IntcodeComputer>) -> bool {
    for amp in amplifiers {
        if !amp.halted {
            return false;
        }
    }
    true
}

fn solution(filename: &str) -> i64 {
    // parse file
    let original_program: HashMap<i64, i64> = parse(filename);
    // let amp_names: Vec<char> = Vec::from(['A', 'B', 'C', 'D', 'E']);

    let mut phases: Vec<i64> = vec![5, 6, 7, 8, 9];
    let permutations: Vec<Vec<i64>> = get_permutations(&mut phases);
    let mut buffers: Vec<VecDeque<i64>> = Vec::new();
    let mut max_output: i64 = 0;

    let mut amplifiers: Vec<IntcodeComputer> = Vec::new();
    for phase_perms in permutations {
        // reset amplifiers
        amplifiers.clear();
        for _ in 0..N_AMPLIFIERS {
            amplifiers.push(IntcodeComputer::new(original_program.clone()));
        }

        // reset buffers
        buffers.clear();
        for i in 0..N_AMPLIFIERS {
            buffers.push(VecDeque::from([phase_perms[i]]));
        }
        buffers[0].push_back(0);

        let mut amp_pointer: usize = 0;
        while !all_amps_halted(&amplifiers) {
            let output = amplifiers[amp_pointer].run(&mut buffers[amp_pointer]);

            // copy output to input of next amplifier
            for value in &output {
                buffers[(amp_pointer + 1) % N_AMPLIFIERS].push_back(*value);
            }

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
    // println!("{}", solution("./example1.txt")); // 139629729
    // println!("{}", solution("./example2.txt")); // 18216
    println!("{}", solution("./input.txt")); // 44282086
}
