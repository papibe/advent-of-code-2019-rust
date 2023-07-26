use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

// struct Point(i32, i32);

fn parse(filename: &str) -> Vec<Vec<String>> {
    // read file
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    // each line will be parsed into a vector of instructions (string)
    // all lines will be return in another vector
    data.lines()
        .map(|line| line.split(",").map(|s| s.to_string()).collect())
        .collect()
}

fn create_wire(instructions: &Vec<String>) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    // movements rules
    let movements = HashMap::from([
        ("R", (0, 1)),
        ("L", (0, -1)),
        ("U", (1, 0)),
        ("D", (-1, 0)),
        ]);
    let mut wire_points = HashSet::new();

    let mut current_row = 0;
    let mut current_col = 0;
    let mut signal_delay: HashMap<(i32, i32), i32> = HashMap::new();
    let mut current_signal_delay = 0;

    // obtain points for the wire by following instructions
    for instruction in instructions {
        let (row, col) = movements.get(&instruction[0..1]).unwrap();
        let steps: i32 = instruction[1..].parse::<i32>().unwrap();
        for _ in 0..steps {
            current_row += row;
            current_col += col;
            wire_points.insert((current_row, current_col));
            current_signal_delay += 1;
            if ! signal_delay.contains_key(&(current_row, current_col)) {
                signal_delay.insert((current_row, current_col), current_signal_delay);
            }
        }
    }
    (wire_points, signal_delay)
}

fn solution(filename: &str) -> i32 {
    // parse file
    let wire_instructions = parse(filename);

    // obtain poits for each wire
    let (wire0, signal0) = create_wire(&wire_instructions[0]);
    let (wire1, signal1) = create_wire(&wire_instructions[1]);

    // intersert, get Manhattan distance and get in min value
    wire0.intersection(&wire1).into_iter()
        .map(
            |(row, col)|
            signal0.get(&(*row, *col)).unwrap() + signal1.get(&(*row, *col)).unwrap()
        )
        .min()
        .unwrap()
}

fn main() {
    println!("{}", solution("./example1.txt")); // 30
    println!("{}", solution("./example2.txt")); // 610
    println!("{}", solution("./example3.txt")); // 410
    println!("{}", solution("./input.txt"));
}
