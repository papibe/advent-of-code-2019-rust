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
    data.split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
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

fn get_first_parameter(program: &mut Vec<i32>, pointer: usize, first_parameter_mode: i32) -> i32 {
    if first_parameter_mode == POSITION_MODE {
        let index: i32 = program[pointer + 1];
        return program[index as usize];
    } else {
        return program[pointer + 1];
    }
}

fn get_second_parameter(program: &mut Vec<i32>, pointer: usize, second_parameter_mode: i32) -> i32 {
    if second_parameter_mode == POSITION_MODE {
        let index: i32 = program[pointer + 2];
        return program[index as usize];
    } else {
        return program[pointer + 2];
    }
}

fn cpy(program: &mut Vec<i32>, pointer: &mut usize, input: i32) {
    let index: i32 = program[*pointer + 1];
    program[index as usize] = input;
    *pointer += 2;
}

fn out(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    output: &mut Vec<i32>,
) {
    let operand: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    output.push(operand);
    *pointer += 2;
}

fn sum(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
) {
    let parameter1: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    let parameter2: i32 = get_second_parameter(program, *pointer, second_parameter_mode);
    let result_index: i32 = program[*pointer + 3];

    program[result_index as usize] = parameter1 + parameter2;
    *pointer += 4;
}

fn mul(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
) {
    let parameter1: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    let parameter2: i32 = get_second_parameter(program, *pointer, second_parameter_mode);
    let result_index: i32 = program[*pointer + 3];

    program[result_index as usize] = parameter1 * parameter2;
    *pointer += 4;
}

fn lth(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
) {
    let parameter1: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    let parameter2: i32 = get_second_parameter(program, *pointer, second_parameter_mode);
    let result_index: i32 = program[*pointer + 3];

    if parameter1 < parameter2 {
        program[result_index as usize] = 1;
    } else {
        program[result_index as usize] = 0;
    }
    *pointer += 4;
}

fn eql(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
) {
    let parameter1: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    let parameter2: i32 = get_second_parameter(program, *pointer, second_parameter_mode);
    let result_index: i32 = program[*pointer + 3];

    if parameter1 == parameter2 {
        program[result_index as usize] = 1;
    } else {
        program[result_index as usize] = 0;
    }
    *pointer += 4;
}

fn jit(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
) {
    let parameter1: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    let parameter2: i32 = get_second_parameter(program, *pointer, second_parameter_mode);

    if parameter1 != 0 {
        *pointer = parameter2 as usize;
    } else {
        *pointer += 3;
    }
}

fn jif(
    program: &mut Vec<i32>,
    pointer: &mut usize,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
) {
    let parameter1: i32 = get_first_parameter(program, *pointer, first_parameter_mode);
    let parameter2: i32 = get_second_parameter(program, *pointer, second_parameter_mode);

    if parameter1 == 0 {
        *pointer = parameter2 as usize;
    } else {
        *pointer += 3;
    }
}

fn solve(program: &mut Vec<i32>, input: i32) -> i32 {
    let mut output: Vec<i32> = Vec::new();
    let mut pointer: usize = 0;

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
            println!("This shouldn't happen!");
        }

        match operation {
            SUM => sum(program, &mut pointer, first_parameter_mode, second_parameter_mode),
            MUL => mul(program, &mut pointer, first_parameter_mode, second_parameter_mode),
            CPY => cpy(program, &mut pointer, input),
            OUT => out(program, &mut pointer, first_parameter_mode, &mut output),
            JIT => jit(program, &mut pointer, first_parameter_mode, second_parameter_mode),
            JIF => jif(program, &mut pointer, first_parameter_mode, second_parameter_mode),
            LTH => lth(program, &mut pointer, first_parameter_mode, second_parameter_mode),
            EQL => eql(program, &mut pointer, first_parameter_mode, second_parameter_mode),
            END => break,
            _ => println!("Unknown instruction!"),
        }
    }

    output[output.len() - 1]
}

fn solution(filename: &str, input: i32) -> i32 {
    // parse file
    let mut program: Vec<i32> = parse(filename);

    // run program with input
    solve(&mut program, input)
}

fn main() {
    println!("{:?}", solution("./input.txt", 5)); // 15163975
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_position_mode_equal_to_8_is_1() {
        let mut program: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 8), 1);
    }

    #[test]
    fn example1_position_mode_not_equal_to_8_is_0() {
        let mut program: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 0), 0);
        program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 7), 0);
        program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 9), 0);
    }

    #[test]
    fn example2_position_mode_less_than_8_is_1() {
        let mut program: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 2), 1);
        program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 7), 1);
    }

    #[test]
    fn example2_position_mode_not_less_than_8_is_0() {
        let mut program: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 8), 0);
        program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(solve(&mut program, 9), 0);
    }

    #[test]
    fn example3_immediate_mode_equal_to_8_is_1() {
        let mut program: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 8), 1);
    }

    #[test]
    fn example3_immediate_mode_not_equal_to_8_is_0() {
        let mut program: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 7), 0);
        program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 9), 0);
    }

    #[test]
    fn example4_immediate_mode_less_than_to_8_is_1() {
        let mut program: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 2), 1);
        program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 5), 1);
    }

    #[test]
    fn example4_immediate_mode_not_less_than_to_8_is_0() {
        let mut program: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 8), 0);
        program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(solve(&mut program, 9), 0);
    }

    #[test]
    fn example5_positon_mode_equal_to_0_is_0() {
        let mut program: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(solve(&mut program, 0), 0);
    }

    #[test]
    fn example5_positon_mode_not_equal_to_0_is_1() {
        let mut program: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(solve(&mut program, 1), 1);
        program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(solve(&mut program, 5), 1);
    }

    #[test]
    fn example6_immediate_mode_equal_to_0_is_0() {
        let mut program: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(solve(&mut program, 0), 0);
    }

    #[test]
    fn example6_immediate_mode_not_equal_to_0_is_1() {
        let mut program: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(solve(&mut program, 1), 1);
        program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(solve(&mut program, 5), 1);
    }

    #[test]
    fn example7_less_than_8_is_999() {
        let mut program: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(solve(&mut program, 1), 999);
        program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(solve(&mut program, 7), 999);
    }

    #[test]
    fn example7_equal_to_8_is_1000() {
        let mut program: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(solve(&mut program, 8), 1000);
    }

    #[test]
    fn example7_greater_than_8_is_1001() {
        let mut program: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(solve(&mut program, 9), 1001);
        program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(solve(&mut program, 10), 1001);
    }
}
