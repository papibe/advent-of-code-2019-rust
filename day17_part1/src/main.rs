use std::collections::HashMap;
use std::fs;

const WALL: i64 = 35;
const SPACE: i64 = 46;
const NEWLINE: i64 = 10;

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
    operation: OperationType,
    first_parameter_mode: ParameterMode,
    second_parameter_mode: ParameterMode,
    third_parameter_mode: ParameterMode,
}

// IntcodeComputer 'class'
struct IntcodeComputer {
    _name: char,
    program: HashMap<i64, i64>,
    pointer: i64,
    halted: bool,
}

impl IntcodeComputer {
    fn run(&mut self, input: i64) -> Vec<i64> {
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
        output
    }

    fn parse_instruction(&self) -> Operation {
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

    fn sum(&mut self, operation: Operation, relative_base: i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 =
            self.get_second_parameter(operation.second_parameter_mode, relative_base);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
            }
            _ => panic!(
                "Incorrect third parameter mode: {:?}",
                operation.third_parameter_mode
            ),
        };

        self.program.insert(result_index, parameter1 + parameter2);
        self.pointer += 4;
    }

    fn get_parameter(
        &mut self,
        parameter_mode: ParameterMode,
        relative_base: i64,
        offset: i64,
    ) -> i64 {
        match parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            }
            ParameterMode::ImmediateMode => {
                return *self.program.entry(self.pointer + offset).or_insert(0)
            }
            ParameterMode::RelativeMode => {
                let index: i64 =
                    relative_base + *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            }
        }
    }

    fn get_first_parameter(
        &mut self,
        first_parameter_mode: ParameterMode,
        relative_base: i64,
    ) -> i64 {
        self.get_parameter(first_parameter_mode, relative_base, 1)
    }

    fn get_second_parameter(
        &mut self,
        second_parameter_mode: ParameterMode,
        relative_base: i64,
    ) -> i64 {
        self.get_parameter(second_parameter_mode, relative_base, 2)
    }

    fn mul(&mut self, operation: Operation, relative_base: i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 =
            self.get_second_parameter(operation.second_parameter_mode, relative_base);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
            }
            _ => panic!(
                "Incorrect third parameter mode: {:?}",
                operation.third_parameter_mode
            ),
        };

        self.program.insert(result_index, parameter1 * parameter2);
        self.pointer += 4;
    }

    fn cpy(&mut self, input: i64, operation: Operation, relative_base: i64) {
        match operation.first_parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            }
            ParameterMode::RelativeMode => {
                let index: i64 = relative_base + *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            }
            _ => panic!(
                "Incorrect first parameter mode: {:?}",
                operation.first_parameter_mode
            ),
        }
        self.pointer += 2;
    }

    fn out(&mut self, operation: Operation, output: &mut Vec<i64>, relative_base: i64) {
        let operand: i64 = self.get_first_parameter(operation.first_parameter_mode, relative_base);
        output.push(operand);
        self.pointer += 2;
    }

    fn jit(&mut self, operation: Operation, relative_base: i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 =
            self.get_second_parameter(operation.second_parameter_mode, relative_base);

        if parameter1 != 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn jif(&mut self, operation: Operation, relative_base: i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 =
            self.get_second_parameter(operation.second_parameter_mode, relative_base);

        if parameter1 == 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn lth(&mut self, operation: Operation, relative_base: i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 =
            self.get_second_parameter(operation.second_parameter_mode, relative_base);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
            }
            _ => panic!(
                "Incorrect third parameter mode: {:?}",
                operation.third_parameter_mode
            ),
        };

        if parameter1 < parameter2 {
            self.program.insert(result_index, 1);
        } else {
            self.program.insert(result_index, 0);
        }
        self.pointer += 4;
    }

    fn eql(&mut self, operation: Operation, relative_base: i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, relative_base);
        let parameter2: i64 =
            self.get_second_parameter(operation.second_parameter_mode, relative_base);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
            }
            _ => panic!(
                "Incorrect third parameter mode: {:?}",
                operation.third_parameter_mode
            ),
        };

        if parameter1 == parameter2 {
            self.program.insert(result_index, 1);
        } else {
            self.program.insert(result_index, 0);
        }
        self.pointer += 4;
    }

    fn arb(&mut self, operation: Operation, relative_base: &mut i64) {
        let parameter1: i64 =
            self.get_first_parameter(operation.first_parameter_mode, *relative_base);
        *relative_base += parameter1;

        self.pointer += 2;
    }
}

fn parse(filename: &str) -> HashMap<i64, i64> {
    // read file
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    // convert content into a vector of integers
    let vec_data: Vec<i64> = data
        .split(",")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut program: HashMap<i64, i64> = HashMap::new();

    for (index, value) in vec_data.iter().enumerate() {
        program.insert(index as i64, *value);
    }

    program
}

fn solution(filename: &str) -> i32 {
    let mut computer = IntcodeComputer {
        _name: ' ',
        program: parse(filename),
        pointer: 0,
        halted: false,
    };

    let output = computer.run(0);

    let mut scaffolding: Vec<Vec<i64>> = vec![];
    let mut scaffolding_row: Vec<i64> = vec![];

    for ascii in &output {
        if *ascii == NEWLINE {
            if scaffolding_row.len() > 0 {
                scaffolding.push(scaffolding_row);
                scaffolding_row = vec![];
            }
        } else {
            scaffolding_row.push(*ascii);
        }
    }
    let mut alignment: usize = 0;
    let rows: usize = scaffolding.len();
    let cols: usize = scaffolding[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if scaffolding[row][col] == WALL {
                if 0 <= row as i32 - 1 && row + 1 < rows && 0 <= col as i32 - 1 && col + 1 < cols {
                    let upper_row: usize = row - 1;
                    let lower_row: usize = row + 1;

                    let left_col: usize = col - 1;
                    let right_col: usize = col + 1;

                    if scaffolding[upper_row][col] == WALL
                        && scaffolding[row][right_col] == WALL
                        && scaffolding[lower_row][col] == WALL
                        && scaffolding[row][left_col] == WALL
                    {
                        alignment += row * col;
                        // print!("O");
                    } else {
                        // print!("#");
                    }
                } else {
                    // print!("#");
                }
            } else if scaffolding[row][col] == SPACE {
                // print!(".");
            } else {
                // print!("{}", char::from_u32(scaffolding[row][col] as u32).unwrap());
            }
        }
        // println!();
    }
    alignment as i32
}

fn main() {
    println!("{:?}", solution("./input.txt")); // 4800
}
