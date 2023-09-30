use std::collections::{HashSet, VecDeque};
use std::fs;

const ENTRANCE: char = '@';
const WALL: char = '#';
// const SPACE: char = '.';

fn parse(filename: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    data.lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_entrance_and_keys(maze: &Vec<Vec<char>>) -> (usize, usize, usize) {
    let mut entrance_row: usize = 1;
    let mut entrance_col: usize = 1;
    let mut number_of_keys: usize = 0;

    // find entrance (@), and all keys (lower case letters)
    for (row, the_row) in maze.iter().enumerate() {
        for (col, cell) in the_row.iter().enumerate() {
            match *cell {
                ENTRANCE => {
                    entrance_row = row;
                    entrance_col = col;
                }
                'a'..='z' => number_of_keys += 1,
                _ => continue,
            }
        }
    }
    (entrance_row, entrance_col, number_of_keys)
}

fn get_neighbors(row: usize, col: usize, maze: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let steps: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut neighbors: Vec<(usize, usize)> = vec![];

    for (step_row, step_col) in steps {
        let is_row_inside: bool =
            row as i32 + step_row >= 0 && row as i32 + step_row < maze.len() as i32;
        let is_col_inside: bool =
            col as i32 + step_col >= 0 && col as i32 + step_col < maze[0].len() as i32;
        if !is_row_inside || !is_col_inside {
            continue;
        }

        let new_row: usize = (row as i32 + step_row) as usize;
        let new_col: usize = (col as i32 + step_col) as usize;
        if maze[new_row][new_col] != WALL {
            neighbors.push((new_row, new_col));
        }
    }
    neighbors
}

fn solve(maze: Vec<Vec<char>>) -> i32 {
    let (entrance_row, entrance_col, number_of_keys) = get_entrance_and_keys(&maze);

    // BFS init
    let mut queue: VecDeque<(usize, usize, usize, usize)> =
        VecDeque::from([(0, 0, entrance_row, entrance_col)]);
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::from([(0, 0, 0)]);

    let goal: usize = (2 as usize).pow(number_of_keys as u32) - 1;

    let mut distance: usize = 0;
    let mut keys: usize;
    let mut row: usize;
    let mut col: usize;

    // BFS
    let mut counter = 0;
    while queue.len() > 0 {
        // println!("{:?}", queue);

        if counter > 10_000_000 {
            println!("max depth reached!");
            break;
        }
        counter += 1;

        (distance, keys, row, col) = queue.pop_front().unwrap();
        // println!("{:?}", keys);
        if keys == goal {
            // println!("we arrived!: {}", distance);
            break;
        }
        for (new_row, new_col) in get_neighbors(row, col, &maze) {
            let mut new_keys = keys;
            // is it a door?
            if maze[new_row][new_col].is_uppercase() {
                let key_bit =
                    1 << ((maze[new_row][new_col].to_ascii_lowercase() as usize) - 'a' as usize);
                if (keys & key_bit) == 0 {  // no key to pass through door
                    continue;
                }
            }
            // is it a key?
            if maze[new_row][new_col].is_lowercase() {
                let key_bit =
                    1 << ((maze[new_row][new_col].to_ascii_lowercase() as usize) - 'a' as usize);
                new_keys = keys | key_bit;
            }
            // check if we've been here before
            let state_key = (new_row, new_col, new_keys);
            if !visited.contains(&state_key) {
                visited.insert(state_key);
                queue.push_back((distance + 1, new_keys, new_row, new_col));
            }
        }
    }
    distance as i32
}

fn solution(filename: &str) -> i32 {
    let maze: Vec<Vec<char>> = parse(filename);
    solve(maze)
}

fn main() {
    println!("{}", solution("./input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_8() {
        assert_eq!(solution("./example1.txt"), 8);
    }

    #[test]
    fn example2_should_be_86() {
        assert_eq!(solution("./example2.txt"), 86);
    }

    #[test]
    fn example3_should_be_132() {
        assert_eq!(solution("./example3.txt"), 132);
    }
    #[test]
    fn example4_should_be_136() {
        assert_eq!(solution("./example4.txt"), 136);
    }
    #[test]
    fn example5_should_be_81() {
        assert_eq!(solution("./example5.txt"), 81);
    }
}
