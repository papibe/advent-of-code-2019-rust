use std::fs;
use std::collections::HashMap;

fn parse(filename: &str) -> HashMap<String, String> {
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    // let mut orbit_map = HashMap::new();

    let orbit_map: HashMap<String, String> = data.lines()
        .map(
            |line| {
                let mut splited_line = line.split(")");
                let center: String = splited_line.next().unwrap().to_string();
                let orbiter: String = splited_line.next().unwrap().to_string();
                (orbiter, center)
            }
        ).collect();
    
    orbit_map
}

fn len_to_com(orbits: &HashMap<String, String>, orbiter: String) -> i32 {
    if orbiter == "COM" {
        return 0
    }
    1 + len_to_com(orbits, orbits[&orbiter].to_string())
}

fn solve(orbits: HashMap<String, String>) -> i32 {
    let mut counter: i32 = 0;
    for (orbiter, _center) in &orbits {
        counter += len_to_com(&orbits, orbiter.to_string());
    }

    counter
}

fn solution(filename: &str) -> i32 {
    let orbit_map: HashMap<String, String> = parse(filename);
    solve(orbit_map)
}

fn main() {
    println!("{}", solution("./example.txt"));  // 42
    println!("{}", solution("./input.txt"));    // 162439
}
