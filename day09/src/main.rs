use std::fs;
use std::collections::HashMap;

enum OperationType {
    SUM = 1,
    MUL = 2,
    CPY = 3,
    OUT = 4,
    JIT = 5,
    JIF = 6,
    LTH = 7,
    EQL = 8,
    ARB = 9,
    END = 99,
}

impl OperationType {
    fn from_i64(number: i64) -> OperationType {
        match number {
            1 => OperationType::SUM,
            2 => OperationType::MUL,
            3 => OperationType::CPY,
            4 => OperationType::OUT,
            5 => OperationType::JIT,
            6 => OperationType::JIF,
            7 => OperationType::LTH,
            8 => OperationType::EQL,
            9 => OperationType::ARB,
            99 => OperationType::END,
            _ => panic!("Unknown operation: {}", number),
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode = 0,
    ImmediateMode = 1,
    RelativeMode = 2,
}

impl ParameterMode {
    fn from_i64(number: i64) -> ParameterMode {
        match number {
            0 => ParameterMode::PositionMode,
            1 => ParameterMode::ImmediateMode,
            2 => ParameterMode::RelativeMode,
            _ => panic!("Unknown parameter mode: {}", number),
        }
    }
}

struct Operation {
    operation : OperationType,
    first_parameter_mode: ParameterMode,
    second_parameter_mode: ParameterMode,
    third_parameter_mode: ParameterMode,
}


// IntcodeComputer 'class'
struct IntcodeComputer{
    _name: char,
    program: HashMap<i64, i64>,
    pointer: i64,
    halted: bool,
}

impl IntcodeComputer {

    fn run(&mut self, input: i64) -> i64 {
        let mut output: Vec<i64> = Vec::new();
        let mut relative_base: i64 = 0;
    
        loop {
            
            let operation: Operation = self.parse_instruction();

            match operation.operation {
                OperationType::SUM => self.sum(operation, relative_base),
                OperationType::MUL => self.mul(operation, relative_base),
                OperationType::CPY => self.cpy(input, operation, relative_base),
                OperationType::OUT => self.out(operation, &mut output, relative_base),
                OperationType::JIT => self.jit(operation, relative_base),
                OperationType::JIF => self.jif(operation, relative_base),
                OperationType::LTH => self.lth(operation, relative_base),
                OperationType::EQL => self.eql(operation, relative_base),
                OperationType::ARB => self.arb(operation, &mut relative_base),
                OperationType::END => break,
            }
        }
        self.halted = true;
        output[output.len() - 1]
    }

    fn parse_instruction(& self) -> Operation {
        let instruction = self.program[&self.pointer];
        let operation: i64 = instruction % 100;
        let parameters: i64 = instruction / 100;
    
        let first_parameter_mode: i64 = parameters % 10;
        let parameters: i64 = parameters / 10;
        let second_parameter_mode: i64 = parameters % 10;
        let parameters: i64 = parameters / 10;
        let third_parameter_mode: i64 = parameters % 10;
    
        Operation {
            operation: OperationType::from_i64(operation),
            first_parameter_mode: ParameterMode::from_i64(first_parameter_mode),
            second_parameter_mode: ParameterMode::from_i64(second_parameter_mode),
            third_parameter_mode: ParameterMode::from_i64(third_parameter_mode),
        }
    }

    fn sum(
        &mut self,
        operation: Operation,
        relative_base: i64,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode, relative_base);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
        };
    
        self.program.insert(result_index, parameter1 + parameter2);
        self.pointer += 4;
    }

    fn get_parameter(&mut self, parameter_mode: ParameterMode, relative_base: i64, offset: i64) -> i64 {
        match parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            },
            ParameterMode::ImmediateMode => return *self.program.entry(self.pointer + offset).or_insert(0),
            ParameterMode::RelativeMode => {
                let index: i64 = relative_base + *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            },
        }
    }
    
    fn get_first_parameter(&mut self, first_parameter_mode: ParameterMode, relative_base: i64) -> i64 {
        self.get_parameter(first_parameter_mode, relative_base, 1)
    }
    
    fn get_second_parameter(&mut self, second_parameter_mode: ParameterMode, relative_base: i64) -> i64 {
        self.get_parameter(second_parameter_mode, relative_base, 2)
    }

    fn mul(
        &mut self,
        operation: Operation,
        relative_base: i64,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode, relative_base);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
        };
    
        self.program.insert(result_index, parameter1 * parameter2);
        self.pointer += 4;
    }


    fn cpy(&mut self, input: i64, operation: Operation, relative_base: i64) {
        match operation.first_parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            },
            ParameterMode::RelativeMode => {
                let index: i64 = relative_base + *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            },
            _ => panic!("Incorrect first parameter mode: {:?}", operation.first_parameter_mode),
        }
        self.pointer += 2;
    }

    fn out(
        &mut self,
        operation: Operation,
        output: &mut Vec<i64>,
        relative_base: i64,
    ) {
        let operand: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        output.push(operand);
        self.pointer += 2;
    }

    fn jit(
        &mut self,
        operation: Operation,
        relative_base: i64,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode, relative_base);
    
        if parameter1 != 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn jif(
        &mut self,
        operation: Operation,
        relative_base: i64,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode, relative_base);
    
        if parameter1 == 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn lth(
        &mut self,
        operation: Operation,
        relative_base: i64,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode, relative_base);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
        };
    
        if parameter1 < parameter2 {
            self.program.insert(result_index, 1);
        } else {
            self.program.insert(result_index, 0);
        }
        self.pointer += 4;
    }

    fn eql(
        &mut self,
        operation: Operation,
        relative_base: i64,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode, relative_base);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
        };
    
        if parameter1 == parameter2 {
            self.program.insert(result_index, 1);
        } else {
            self.program.insert(result_index, 0);
        }
        self.pointer += 4;
    }

    fn arb(&mut self, operation: Operation, relative_base: &mut i64) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode, *relative_base);
        *relative_base += parameter1;
    
        self.pointer += 2;
    }
}


fn parse(filename: &str) -> HashMap<i64, i64> {
    // read file
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    // convert content into a vector of integers
    let vec_data: Vec<i64>  = data
        .split(",")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut program: HashMap<i64, i64> = HashMap::new();

    for (index, value) in vec_data.iter().enumerate() {
        program.insert(index as i64, *value);
    }

    program
}


fn solution(filename: &str, input: i64) -> i64 {
    let mut computer = IntcodeComputer {
        _name: ' ',
        program: parse(filename),
        pointer: 0,
        halted: false,

    };
    computer.run(input)
}

fn main() {
    // println!("{:?}", solution("./example1.txt", 1)); // 99
    // full output ^ output: [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    // println!("{:?}", solution("./example2.txt", 1)); // 1219070632396864
    // println!("{:?}", solution("./example3.txt", 1)); // 1125899906842624
    println!("part1: {:?}", solution("./input.txt", 1)); // 4234906522
    println!("part2: {:?}", solution("./input.txt", 2)); // 60962
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(vec_data: Vec<i64>, input: i64) -> i64 {
        let mut computer = IntcodeComputer {
            _name: ' ',
            program: HashMap::new(),
            pointer: 0,
            halted: false,
    
        };
        for (index, value) in vec_data.iter().enumerate() {
            computer.program.insert(index as i64, *value);
        }
    
        computer.run(input)
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
        assert_eq!(solve(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 0), 0);
    }

    #[test]
    fn example5_positon_mode_not_equal_to_0_is_1() {
        assert_eq!(solve(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 1), 1);
        assert_eq!(solve(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 5), 1);
    }

    #[test]
    fn example6_immediate_mode_equal_to_0_is_0() {
        assert_eq!(solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0), 0);
    }

    #[test]
    fn example6_immediate_mode_not_equal_to_0_is_1() {
        assert_eq!(solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 1), 1);
        assert_eq!(solve(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 5), 1);
    }

    #[test]
    fn example7_less_than_8_is_999() {
        assert_eq!(solve(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ], 1), 999);
        assert_eq!(solve(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ], 7), 999);
    }

    #[test]
    fn example7_equal_to_8_is_1000() {
        assert_eq!(solve(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ], 8), 1000);
    }

    #[test]
    fn example7_greater_than_8_is_1001() {
        assert_eq!(solve(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ], 9), 1001);
        assert_eq!(solve(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ], 10), 1001);
    }
}
