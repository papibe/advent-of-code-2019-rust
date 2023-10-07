
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

const WALL: char = '#';
const SPACE: char = '.';

fn parse(
    filename: &str
) ->  ((usize, usize), (usize, usize), Vec<Vec<char>>, HashMap<(usize, usize), (usize, usize)>) {
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

    for (row, a_row)in vec_data.iter().enumerate() {
        for (col, cell) in a_row.iter().enumerate() {
            match *cell {
                WALL => maze[row - 2][col - 2] = *cell,
                SPACE => {
                    let cell_row: usize = row - 2;
                    let cell_col: usize = col - 2;
                    maze[cell_row][cell_col] = *cell;

                    // look dor portals
                    let steps: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for (dr, dc) in steps {
                        let portal_row: usize = (row as i32 + dr) as usize;
                        let portal_col: usize = (col as i32 + dc) as usize;

                        let mut node_name: String = String::new();
                        if 'A' <= vec_data[portal_row][portal_col] && vec_data[portal_row][portal_col] <= 'Z' {
                            let first_portal_row: usize = (portal_row as i32 + dr) as usize;
                            let first_portal_col: usize = (portal_col as i32 + dc) as usize;
                            if dr == -1 || dc == -1  {
                                node_name.push(vec_data[first_portal_row][first_portal_col]);
                                node_name.push(vec_data[portal_row][portal_col]);
                            } else {
                                node_name.push(vec_data[portal_row][portal_col]);
                                node_name.push(vec_data[first_portal_row][first_portal_col]);
                            }
                            if portals.contains_key(&node_name) {
                                portals.get_mut(&node_name).unwrap().push((cell_row, cell_col));
                            } else {
                                portals.insert(node_name.clone(), vec![(cell_row, cell_col)]);
                            }

                            positions.insert((cell_row, cell_col), node_name.clone());
                            break;
                        }
                    }
                },
                _ => continue,
            }
        }
    }

    let mut portal_to: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for (_portal_name, pos) in &portals {
        if pos.len() > 1 {
            portal_to.insert(pos[0], pos[1]);
            portal_to.insert(pos[1], pos[0]);
        }
    }

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
    portal_to: &HashMap<(usize, usize), (usize, usize)>,
) -> i32 {
    // Flood fill init
    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::from([(0, start)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([start]);
    let mut min_distance: usize = 0;

    while queue.len() > 0 {
        let (distance, position) = queue.pop_front().unwrap();
        if position == end {
            min_distance = distance;
            break;
        }
        for (new_row, new_col) in get_neighbors(position.0, position.1, &maze) {
            if visited.contains(&(new_row, new_col)) {
                continue;
            }
            if portal_to.contains_key(&(new_row, new_col)) {
                let jump_position = portal_to[&(new_row, new_col)];
                queue.push_back((distance + 2, jump_position)); 
                visited.insert(jump_position);
            } else {
                queue.push_back((distance + 1, (new_row, new_col))); 
                visited.insert((new_row, new_col));
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
    println!("{:?}", solution("./input.txt"));  // 664 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_23() {
        assert_eq!(solution("./example1.txt"), 23);
    }

    #[test]
    fn example2_should_be_58() {
        assert_eq!(solution("./example2.txt"), 58);
    }
}
