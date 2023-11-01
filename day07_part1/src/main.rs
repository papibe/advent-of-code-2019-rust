use std::collections::VecDeque;

use intcode::{parse, IntcodeComputer};


fn permutations(numbers: &mut Vec<i64>) -> Vec<Vec<i64>> {
    
    fn dfs(index: usize, numbers: &mut Vec<i64>, result: &mut Vec<Vec<i64>>) {
        if index == numbers.len() {
            result.push(numbers.clone());
        }
        for i in index..numbers.len()  {
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

fn solution(filename: &str) -> i64 {
    // let mut computer = IntcodeComputer::new(parse(filename));
    let mut phases: Vec<i64> = vec![0, 1, 2, 3, 4];

    let perms: Vec<Vec<i64>> = permutations(&mut phases);
    let mut max_output: i64 = -1;
    for phases in perms {
        let mut previous_output: i64 = 0;
        for phase in phases {
            let mut computer = IntcodeComputer::new(parse(filename));
            let mut input: VecDeque<i64> = VecDeque::from([phase, previous_output]);
            let output = computer.run(&mut input);
            previous_output = *output.last().unwrap();
        }
        if previous_output > max_output {
            max_output = previous_output;
        }
    }

    max_output
}

fn main() {
    println!("{:?}", solution("./example1.txt")); // 43210
    println!("{:?}", solution("./input.txt")); // 34852
}
