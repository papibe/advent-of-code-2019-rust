use std::fs;

fn solution(filename: &str) -> i32 {
    // read file
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // cycle over all lines (masses)
    let mut total_fuel = 0;
    for module in data.lines() {
        let mass = module.parse::<i32>().unwrap();

        // determine how much more fuel the previous fuel requires
        let mut module_fuel = 0;
        let mut fuel = mass / 3 - 2;
        while fuel > 0 {
            module_fuel += fuel;
            fuel = fuel / 3 - 2;
        }
        total_fuel += module_fuel
    }
    total_fuel
}

fn main() {
        println!("{}", solution("./src/day01/input.txt"));    // 4900568
}
