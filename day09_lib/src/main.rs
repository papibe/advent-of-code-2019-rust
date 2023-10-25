// use std::fs;
use std::collections::VecDeque;

use intcode::{parse, IntcodeComputer};

fn solution(filename: &str, input_value: i64) -> i64 {
    let mut computer = IntcodeComputer::new(parse(filename));

    let mut input = VecDeque::from([input_value]);
    let output = computer.run(&mut input);

    *output.last().unwrap()
}

fn main() {
    println!("part1: {:?}", solution("./input.txt", 1)); // 4234906522
    println!("part2: {:?}", solution("./input.txt", 2)); // 60962
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn solve(vec_data: Vec<i64>, input_value: i64) -> i64 {
        let mut computer = IntcodeComputer::new(HashMap::new());

        for (index, value) in vec_data.iter().enumerate() {
            computer.program.insert(index as i64, *value);
        }
        let mut input = VecDeque::from([input_value]);
        let output = computer.run(&mut input);

        *output.last().unwrap()
    }

    //
    // Old tests
    // No opcode 9, and fix memory, but they should pass.
    //

    #[test]
    fn example1_position_mode_equal_to_8_is_1() {
        assert_eq!(solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8), 1);
    }

    #[test]
    fn example1_position_mode_not_equal_to_8_is_0() {
        assert_eq!(solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 0), 0);
        assert_eq!(solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7), 0);
        assert_eq!(solve(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 9), 0);
    }

    #[test]
    fn example2_position_mode_less_than_8_is_1() {
        assert_eq!(solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 2), 1);
        assert_eq!(solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7), 1);
    }

    #[test]
    fn example2_position_mode_not_less_than_8_is_0() {
        assert_eq!(solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8), 0);
        assert_eq!(solve(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9), 0);
    }

    #[test]
    fn example3_immediate_mode_equal_to_8_is_1() {
        assert_eq!(solve(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8), 1);
    }

    #[test]
    fn example3_immediate_mode_not_equal_to_8_is_0() {
        assert_eq!(solve(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 7), 0);
        assert_eq!(solve(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9), 0);
    }

    #[test]
    fn example4_immediate_mode_less_than_to_8_is_1() {
        assert_eq!(solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 2), 1);
        assert_eq!(solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 5), 1);
    }

    #[test]
    fn example4_immediate_mode_not_less_than_to_8_is_0() {
        assert_eq!(solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8), 0);
        assert_eq!(solve(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 9), 0);
    }

    #[test]
    fn example5_positon_mode_equal_to_0_is_0() {
        assert_eq!(
            solve(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                0
            ),
            0
        );
    }

    #[test]
    fn example5_positon_mode_not_equal_to_0_is_1() {
        assert_eq!(
            solve(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                1
            ),
            1
        );
        assert_eq!(
            solve(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                5
            ),
            1
        );
    }

    #[test]
    fn example6_immediate_mode_equal_to_0_is_0() {
        assert_eq!(
            solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0),
            0
        );
    }

    #[test]
    fn example6_immediate_mode_not_equal_to_0_is_1() {
        assert_eq!(
            solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 1),
            1
        );
        assert_eq!(
            solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 5),
            1
        );
    }

    #[test]
    fn example7_less_than_8_is_999() {
        assert_eq!(
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                1
            ),
            999
        );
        assert_eq!(
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                7
            ),
            999
        );
    }

    #[test]
    fn example7_equal_to_8_is_1000() {
        assert_eq!(
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                8
            ),
            1000
        );
    }

    #[test]
    fn example7_greater_than_8_is_1001() {
        assert_eq!(
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                9
            ),
            1001
        );
        assert_eq!(
            solve(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                10
            ),
            1001
        );
    }
}
