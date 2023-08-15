use std::collections::HashMap;
use std::fs;

fn parse(filename: &str) -> HashMap<String, String> {
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    // let mut orbit_map = HashMap::new();

    let orbit_map: HashMap<String, String> = data
        .lines()
        .map(|line| {
            let mut splited_line = line.split(")");
            let center: String = splited_line.next().unwrap().to_string();
            let orbiter: String = splited_line.next().unwrap().to_string();
            (orbiter, center)
        })
        .collect();

    orbit_map
}

fn get_path_to_root(orbits: &HashMap<String, String>, root: &str, node: &str) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    let mut current = node;

    while current != root {
        path.push(current.to_string());
        current = &orbits[current];
    }

    path
}

fn solve(orbits: HashMap<String, String>, you: &str, san: &str) -> i32 {
    let you_path_to_root: Vec<String> = get_path_to_root(&orbits, "COM", you);
    let san_path_to_root: Vec<String> = get_path_to_root(&orbits, "COM", san);

    let you_len: usize = you_path_to_root.len();
    let san_len: usize = san_path_to_root.len();

    let mut you_index: i32 = (you_len - 1) as i32;
    let mut san_index: i32 = (san_len - 1) as i32;
    while you_index >= 0 && san_index >= 0 {
        if you_path_to_root[you_index as usize] != san_path_to_root[san_index as usize] {
            break;
        }
        you_index -= 1;
        san_index -= 1;
    }

    (you_index + san_index) as i32
}

fn solution(filename: &str) -> i32 {
    let orbit_map: HashMap<String, String> = parse(filename);

    solve(orbit_map, "YOU", "SAN")
}

fn main() {
    println!("{}", solution("./example.txt")); // 4
    println!("{}", solution("./input.txt")); // 367
}
