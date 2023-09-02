use std::fs;
use std::cmp;
use std::collections::{HashMap, HashSet};

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

enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    fn from_i64(number: i64) -> Color {
        match number {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unknow color number: {}", number),
        }
    }
}

enum Turn {
    Left = 0,
    Right = 1,
}

impl Turn {
    fn from_i64(number: i64) -> Turn {
        match number {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Unknow turn number: {}", number),
        }
    }
}


// IntcodeComputer 'class'
struct IntcodeComputer{
    _name: char,
    program: HashMap<i64, i64>,
    pointer: i64,
    relative_base: i64,
    halted: bool,
}

impl IntcodeComputer {

    fn run(&mut self, input: i64) -> Vec<i64> {
        let mut output: Vec<i64> = Vec::new();
        // let mut relative_base: i64 = 0;
    
        loop {
            
            let operation: Operation = self.parse_instruction();

            match operation.operation {
                OperationType::SUM => self.sum(operation),
                OperationType::MUL => self.mul(operation),
                OperationType::CPY => self.cpy(input, operation),
                OperationType::OUT => {
                    self.out(operation, &mut output);
                    if output.len() == 2 {  // paint color and direction to turn
                        return output;
                    }
                },
                OperationType::JIT => self.jit(operation),
                OperationType::JIF => self.jif(operation),
                OperationType::LTH => self.lth(operation),
                OperationType::EQL => self.eql(operation),
                OperationType::ARB => self.arb(operation),
                OperationType::END => break,
            }
            // println!("relative_base: {}", self.relative_base);
        }
        self.halted = true;
        // println!("output: {:?}", output);
        output
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
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
        };
    
        self.program.insert(result_index, parameter1 + parameter2);
        self.pointer += 4;
    }

    fn get_parameter(&mut self, parameter_mode: ParameterMode, offset: i64) -> i64 {
        match parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            },
            ParameterMode::ImmediateMode => return *self.program.entry(self.pointer + offset).or_insert(0),
            ParameterMode::RelativeMode => {
                let index: i64 = self.relative_base + *self.program.entry(self.pointer + offset).or_insert(0);
                return *self.program.entry(index).or_insert(0);
            },
        }
    }
    
    fn get_first_parameter(&mut self, first_parameter_mode: ParameterMode) -> i64 {
        self.get_parameter(first_parameter_mode, 1)
    }
    
    fn get_second_parameter(&mut self, second_parameter_mode: ParameterMode) -> i64 {
        self.get_parameter(second_parameter_mode, 2)
    }

    fn mul(
        &mut self,
        operation: Operation,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
        };
    
        self.program.insert(result_index, parameter1 * parameter2);
        self.pointer += 4;
    }


    fn cpy(&mut self, input: i64, operation: Operation) {
        match operation.first_parameter_mode {
            ParameterMode::PositionMode => {
                let index: i64 = *self.program.entry(self.pointer + 1).or_insert(0);
                self.program.insert(index, input);
            },
            ParameterMode::RelativeMode => {
                let index: i64 = self.relative_base + *self.program.entry(self.pointer + 1).or_insert(0);
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
    ) {
        let operand: i64 = self.get_first_parameter(operation.first_parameter_mode);
        output.push(operand);
        self.pointer += 2;
    }

    fn jit(
        &mut self,
        operation: Operation,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);
    
        if parameter1 != 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn jif(
        &mut self,
        operation: Operation,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);
    
        if parameter1 == 0 {
            self.pointer = parameter2;
        } else {
            self.pointer += 3;
        }
    }

    fn lth(
        &mut self,
        operation: Operation,
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
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
    ) {
        let parameter1: i64 = self.get_first_parameter(operation.first_parameter_mode);
        let parameter2: i64 = self.get_second_parameter(operation.second_parameter_mode);
    
        let result_index: i64 =  match operation.third_parameter_mode {
            ParameterMode::PositionMode => *self.program.entry(self.pointer + 3).or_insert(0),
            ParameterMode::RelativeMode => self.relative_base + *self.program.entry(self.pointer + 3).or_insert(0),
            _ => panic!("Incorrect third parameter mode: {:?}", operation.third_parameter_mode),
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


fn solution(filename: &str, input: i64) -> String {
    let mut computer = IntcodeComputer {
        _name: ' ',
        program: parse(filename),
        pointer: 0,
        relative_base: 0,
        halted: false,

    };

    let mut panel_color: i64 = input;

    let mut current_position: (i32, i32) = (0, 0);
    let mut facing_direction: (i32, i32) = (1, 0);
    let mut painted_panels: HashSet<(i32, i32)> = HashSet::new();
    let mut white_panels: HashSet<(i32, i32)> = HashSet::new();
    let mut black_panels: HashSet<(i32, i32)> = HashSet::new();

    // if initial panel is white
    if panel_color == 1 {
        white_panels.insert(current_position);
    }

    while !computer.halted {
        let output: Vec<i64> = computer.run(panel_color);
        if computer.halted {
            break;
        }
        let color: Color = Color::from_i64(output[0]);
        let turn: Turn = Turn::from_i64(output[1]);

        // determine current tile color for input
        painted_panels.insert(current_position);
        match color {
            Color::Black => {
                black_panels.insert(current_position);
                white_panels.remove(&current_position);
            },
            Color::White => {
                white_panels.insert(current_position);
                black_panels.remove(&current_position);
            },
        };

        // set new position and new facing
        facing_direction = match turn {
            Turn::Left => (facing_direction.1, -facing_direction.0),
            Turn::Right => (-facing_direction.1, facing_direction.0),
        };
        current_position = (current_position.0 + facing_direction.0, current_position.1 + facing_direction.1);

        // if current_position in white_panels {
        panel_color = if white_panels.contains(&current_position) { 1 } else { 0 };
    }
    let mut row_min = i32::MAX;
    let mut row_max = i32::MIN;
    let mut col_min = i32::MAX;
    let mut col_max = i32::MIN;
    for (row, col) in &white_panels {
        row_min = cmp::min(row_min, *row);
        row_max = cmp::max(row_max, *row);

        col_min = cmp::min(col_min, *col);
        col_max = cmp::max(col_max, *col);
    }

    let mut string_list: Vec<String> = Vec::new();

    for row in (row_min..=row_max).rev() {
        for col in col_min..=col_max {
            if white_panels.contains(&(row, col)) {
                string_list.push("#".to_string());
            } else {
                string_list.push(".".to_string());
            }
        }
        string_list.push("\n".to_string());
    }

    string_list.join("")
}

fn main() {
    print!("{}", solution("./input.txt", 1)); //
}
