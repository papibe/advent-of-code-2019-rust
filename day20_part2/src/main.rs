use std::collections::HashMap;
use std::fs;

use priority_queue::DoublePriorityQueue;

const WALL: char = '#';
const SPACE: char = '.';

#[derive(Debug)]
struct Portal {
    name: String,
    jump_to: (usize, usize),
    is_inner: bool,
    is_outer: bool,
}

impl Portal {
    fn is_open(&self, level: usize) -> bool {
        if self.name == "AA" {
            false
        } else if level == 0 && self.name == "ZZ" {
            true
        } else if level != 0 && self.name == "ZZ" {
            false
        } else if level == 0 && self.is_outer {
            false
        } else if self.is_inner {
            true
        } else {
            true
        }
    }
    fn next_level(&self, level: usize) -> usize {
        if self.is_inner {
            level + 1
        } else if level == 0 && self.name == "ZZ" {
            0
        } else if self.is_outer {
            level - 1
        } else {
            panic!("level is crazy!")
        }
    }
}

fn is_portal_inner(pos: (usize, usize), rows: usize, cols: usize) -> bool {
    if pos.0 == 0 || pos.1 == 0 {
        return false;
    }
    if pos.0 == rows - 1 || pos.1 == cols - 1 {
        return false;
    }
    true
}

fn parse(
    filename: &str,
) -> (
    (usize, usize),
    (usize, usize),
    Vec<Vec<char>>,
    HashMap<(usize, usize), Portal>,
) {
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    // convert content into a matrix of chars
    let vec_data: Vec<Vec<char>> = data
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let maze_rows: usize = vec_data.len() - 4;
    let maze_cols: usize = vec_data[0].len() - 4;
    let mut maze: Vec<Vec<char>> = vec![vec![' '; maze_cols]; maze_rows];

    // filter only maze
    let mut portals: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut positions: HashMap<(usize, usize), String> = HashMap::new();

    for (row, a_row) in vec_data.iter().enumerate() {
        for (col, cell) in a_row.iter().enumerate() {
            match *cell {
                WALL => maze[row - 2][col - 2] = *cell,
                SPACE => {
                    let cell_row: usize = row - 2;
                    let cell_col: usize = col - 2;
                    maze[cell_row][cell_col] = *cell;
                    // println!("cell_row: {}, cell_col: {}", cell_row, cell_col);
                    let steps: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for (dr, dc) in steps {
                        let portal_row: usize = (row as i32 + dr) as usize;
                        let portal_col: usize = (col as i32 + dc) as usize;
                        // println!("{}, {}", portal_row, portal_col);

                        let mut node_name: String = String::new();
                        if 'A' <= vec_data[portal_row][portal_col]
                            && vec_data[portal_row][portal_col] <= 'Z'
                        {
                            let first_portal_row: usize = (portal_row as i32 + dr) as usize;
                            let first_portal_col: usize = (portal_col as i32 + dc) as usize;
                            if dr == -1 || dc == -1 {
                                node_name.push(vec_data[first_portal_row][first_portal_col]);
                                node_name.push(vec_data[portal_row][portal_col]);
                            } else {
                                node_name.push(vec_data[portal_row][portal_col]);
                                node_name.push(vec_data[first_portal_row][first_portal_col]);
                            }
                            // println!("node_name: {}", node_name);
                            if portals.contains_key(&node_name) {
                                portals
                                    .get_mut(&node_name)
                                    .unwrap()
                                    .push((cell_row, cell_col));
                            } else {
                                portals.insert(node_name.clone(), vec![(cell_row, cell_col)]);
                            }

                            positions.insert((cell_row, cell_col), node_name.clone());
                            break;
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    let maze_rows: usize = maze.len();
    let maze_cols: usize = maze[0].len();

    let mut portal_to: HashMap<(usize, usize), Portal> = HashMap::new();
    for (portal_name, pos) in &portals {
        if pos.len() > 1 {
            let mut is_inner: bool = is_portal_inner(pos[0], maze_rows, maze_cols);
            portal_to.insert(
                pos[0],
                Portal {
                    name: portal_name.clone(),
                    jump_to: pos[1],
                    is_inner: is_inner,
                    is_outer: !is_inner,
                },
            );

            is_inner = is_portal_inner(pos[1], maze_rows, maze_cols);
            portal_to.insert(
                pos[1],
                Portal {
                    name: portal_name.clone(),
                    jump_to: pos[0],
                    is_inner: is_inner,
                    is_outer: !is_inner,
                },
            );
        }
    }
    portal_to.insert(
        portals["AA"][0],
        Portal {
            name: "AA".to_string(),
            jump_to: portals["AA"][0],
            is_inner: false,
            is_outer: true,
        },
    );
    portal_to.insert(
        portals["ZZ"][0],
        Portal {
            name: "ZZ".to_string(),
            jump_to: portals["ZZ"][0],
            is_inner: false,
            is_outer: true,
        },
    );

    // println!("{:?}", portals);
    // println!("{:?}", positions);
    // println!("{:?}", portal_to);

    (portals["AA"][0], portals["ZZ"][0], maze, portal_to)
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
        if maze[new_row][new_col] == SPACE {
            neighbors.push((new_row, new_col));
        }
    }
    neighbors
}

fn solve(
    start: (usize, usize),
    end: (usize, usize),
    maze: &Vec<Vec<char>>,
    portal_to: &HashMap<(usize, usize), Portal>,
) -> i32 {
    // Flood fill init
    // let mut queue: VecDeque<(usize, usize, (usize, usize))> = VecDeque::from([(0, 0, start)]);
    let mut queue: DoublePriorityQueue<(usize, (usize, usize)), usize> = DoublePriorityQueue::new();
    queue.push((0, start), 0);

    // let mut visited: HashSet<(usize, (usize, usize))> = HashSet::from([(0, start)]);
    let mut visited: HashMap<(usize, (usize, usize)), usize> = HashMap::new();
    visited.insert((0, start), 0);

    let mut min_distance: usize = 0;

    while queue.len() > 0 {
        let ((level, position), distance) = queue.pop_min().unwrap();
        // if portal_to.contains_key(&position) {
        //     println!("{}, {:?}, {:?}, {}", level, position, portal_to[&position],distance);
        // }
        // println!("{}, {}, {:?}", distance, level, position);
        if level == 0 && position == end {
            // println!("magnificent!");
            min_distance = distance;
            break;
        }
        for (new_row, new_col) in get_neighbors(position.0, position.1, &maze) {
            if portal_to.contains_key(&(new_row, new_col)) {
                let portal: &Portal = &portal_to[&(new_row, new_col)];
                if !portal.is_open(level) {
                    // println!("CLOSED:  {:?} -> {:?}, {:?}", position, (new_row, new_col), portal);
                    continue;
                }
                // println!("  {:?} -> {:?}, {:?}", position, (new_row, new_col), portal);
                let jump_position = portal.jump_to;
                let next_level = portal.next_level(level);

                let mut new_distance: usize = distance;
                if portal.name != "ZZ" {
                    new_distance += 2;
                } else {
                    new_distance += 1;
                }

                let state_key = (next_level, jump_position);

                if !visited.contains_key(&state_key) {
                    visited.insert(state_key, new_distance);
                    queue.push((next_level, jump_position), new_distance);
                } else {
                    let old_distance = visited[&state_key];
                    let current_distance = distance + 2;
                    if current_distance < old_distance {
                        visited.insert(state_key, current_distance);
                        queue.push((level, (new_row, new_col)), current_distance);
                    }
                }
            } else {
                let state_key = (level, (new_row, new_col));

                if !visited.contains_key(&state_key) {
                    visited.insert((level, (new_row, new_col)), distance + 1);
                    queue.push((level, (new_row, new_col)), distance + 1);
                } else {
                    let old_distance = visited[&state_key];
                    let current_distance = distance + 1;
                    if current_distance < old_distance {
                        visited.insert(state_key, current_distance);
                        queue.push((level, (new_row, new_col)), current_distance);
                    }
                }
            }
        }
    }
    min_distance as i32
}

fn solution(filename: &str) -> i32 {
    let (start, end, maze, portal_to) = parse(filename);
    solve(start, end, &maze, &portal_to)
}

fn main() {
    println!("{:?}", solution("./input.txt")); // 7334
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_23() {
        assert_eq!(solution("./example1.txt"), 26);
    }

    #[test]
    fn example3_should_be_396() {
        assert_eq!(solution("./example3.txt"), 396);
    }
}
