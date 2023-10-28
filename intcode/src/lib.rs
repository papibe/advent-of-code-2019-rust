use std::collections::{HashMap, VecDeque};
use std::fs;

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
pub struct IntcodeComputer {
    pub address: usize,
    pub program: HashMap<i64, i64>,
    pub pointer: i64,
    pub halted: bool,
    pub relative_base: i64,
    pub get_input: fn(&mut VecDeque<i64>) -> Option<i64>,
    pub is_output_ready: fn(& Vec<i64>) -> bool,
}


fn get_input(input: &mut VecDeque<i64>) -> Option<i64> {
    input.pop_front()
}

fn is_output_ready(_output: &Vec<i64>) -> bool {
    false
}

impl IntcodeComputer {
    
    pub fn new(program: HashMap<i64, i64>) -> Self {
        IntcodeComputer {
            address: 0,
            program: program,
            pointer: 0,
            halted: false,
            relative_base: 0,
            get_input: get_input,
            is_output_ready: is_output_ready,
        }
    }

    pub fn run(&mut self, input: &mut VecDeque<i64>) -> Vec<i64> {
        let mut output: Vec<i64> = vec![];
        loop {
            let operation: Operation = self.parse_instruction();

            match operation.operation {
                OperationType::SUM => self.sum(operation),
                OperationType::MUL => self.mul(operation),
                OperationType::CPY =>
                    match (self.get_input)(input) {
                        Some(input_value) => self.cpy(input_value, operation),
                        None => return output,
                    },
                OperationType::OUT => self.out(operation, &mut output),
                OperationType::JIT => self.jit(operation),
                OperationType::JIF => self.jif(operation),
                OperationType::LTH => self.lth(operation),
                OperationType::EQL => self.eql(operation),
                OperationType::ARB => self.arb(operation),
                OperationType::END => break,
            }

            if (is_output_ready)(&output) {
                return output;
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

    fn sum(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
            }
            _ => panic!(
                "Incorrect third parameter mode: {:?}",
                operation.third_parameter_mode
            ),
        };

        self.program.insert(result_index, parameter1 + parameter2);
        self.pointer += 4;
    }

    fn get_parameter(&mut self, parameter_mode: ParameterMode, offset: i64) -> i64 {
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
                    self.relative_base + *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            }
        }
    }

    fn get_first_parameter(&mut self, first_parameter_mode: ParameterMode) -> i64 {
        self.get_parameter(first_parameter_mode, 1)
    }

    fn get_second_parameter(&mut self, second_parameter_mode: ParameterMode) -> i64 {
        self.get_parameter(second_parameter_mode, 2)
    }

    fn mul(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
            }
            _ => panic!(
                "Incorrect third parameter mode: {:?}",
                operation.third_parameter_mode
            ),
        };

        self.program.insert(result_index, parameter1 * parameter2);
        self.pointer += 4;
    }

    fn cpy(&mut self, input: i64, operation: Operation) {
        // let input: i64 = inputs.pop_front().unwrap();
        match operation.first_parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            }
            ParameterMode::RelativeMode => {
                let index: i64 =
                    self.relative_base + *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            }
            _ => panic!(
                "Incorrect first parameter mode: {:?}",
                operation.first_parameter_mode
            ),
        }
        self.pointer += 2;
    }

    fn out(&mut self, operation: Operation, output: &mut Vec<i64>) {
        let operand: i64 = self.get_first_parameter(operation.first_parameter_mode);
        output.push(operand);

        self.pointer += 2;
    }

    fn jit(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);

        if parameter1 != 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn jif(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);

        if parameter1 == 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn lth(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
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

    fn eql(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);

        let result_index: i64 = match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => {
                self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0)
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

    fn arb(&mut self, operation: Operation) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        self.relative_base += parameter1;

        self.pointer += 2;
    }
}

pub fn parse(filename: &str) -> HashMap<i64, i64> {
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
