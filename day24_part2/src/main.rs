use std::collections::HashMap;
use std::fs;

const ORIGINAL_BUG: char = '#';
// const ORIGINAL_SPACE: char = '.';

const BUG: u8 = 1;
const SPACE: u8 = 0;

#[derive(Debug, Clone)]
struct PlutonianSettlement {
    levels: HashMap<i32, [u8; 25]>,
    nlevels: u32,
}


impl PlutonianSettlement {
    fn new(tiles: [u8; 25]) -> Self {
        PlutonianSettlement {
            levels: HashMap::from([(0, tiles)]),
            nlevels: 0,
        }
    }

    fn add_levels(&mut self) {
        self.nlevels += 1;
        let next_level: i32 = self.nlevels as i32;
        self.levels.insert(next_level, [0; 25]);
        self.levels.insert(-next_level, [0; 25]);
    }

    fn get(&self, level: i32, position: usize) -> u8 {
        if !self.levels.contains_key(&level) {
            // self.levels.insert(level, [0; 25]);
            return 0;
        }
        self.levels[&level][position]
    }

    fn set(&mut self, level: i32, position: usize, value: u8) {
        if !self.levels.contains_key(&level) {
            self.levels.insert(level, [0; 25]);
        }
        let grid = self.levels.get_mut(&level).unwrap();
        grid[position] = value;
    }


    fn get_neighbors(&self, level: i32, position: usize) -> u32 {
        let local_neighbors = match position {
            0 => self.get(level, 1) + self.get(level, 5),
            1 => self.get(level, 0) + self.get(level, 2) + self.get(level, 6),
            2 => self.get(level, 1) + self.get(level, 3) + self.get(level, 7),
            3 => self.get(level, 2) + self.get(level, 4) + self.get(level, 8),
            4 => self.get(level, 3) + self.get(level, 9),
            5 => self.get(level, 0) + self.get(level, 6) + self.get(level, 10),
            6 => self.get(level, 1) + self.get(level, 5) + self.get(level, 7) + self.get(level, 11),
            7 => self.get(level, 2) + self.get(level, 6) + self.get(level, 8),
            8 => self.get(level, 3) + self.get(level, 7) + self.get(level, 9) + self.get(level, 13),
            9 => self.get(level, 4) + self.get(level, 8) + self.get(level, 14),
            10 => self.get(level, 5) + self.get(level, 11) + self.get(level, 15),
            11 => self.get(level, 6) + self.get(level, 10) + self.get(level, 16),
            13 => self.get(level, 8) + self.get(level, 14) + self.get(level, 18),
            14 => self.get(level, 9) + self.get(level, 13) + self.get(level, 19),
            15 => self.get(level, 10) + self.get(level, 16) + self.get(level, 20),
            16 => self.get(level, 11) + self.get(level, 15) + self.get(level, 17) + self.get(level, 21),
            17 => self.get(level, 16) + self.get(level, 18) + self.get(level, 22),
            18 => {
                self.get(level, 13)
                    + self.get(level, 17)
                    + self.get(level, 19)
                    + self.get(level, 23)
            },
            19 => self.get(level, 14) + self.get(level, 18) + self.get(level, 24),
            20 => self.get(level, 15) + self.get(level, 21),
            21 => self.get(level, 16) + self.get(level, 20) + self.get(level, 22),
            22 => self.get(level, 17) + self.get(level, 21) + self.get(level, 23),
            23 => self.get(level, 18) + self.get(level, 22) + self.get(level, 24),
            24 => self.get(level, 19) + self.get(level, 23),
            _ => 0,
        };

        let outer_neighbors = match position {
            0 => self.get(level - 1, 7) + self.get(level - 1, 11),
            1 | 2 | 3 => self.get(level - 1, 7),
            4 => self.get(level - 1, 7) + self.get(level - 1, 13),
            5 | 10 | 15 => self.get(level - 1, 11),
            9 | 14 | 19 => self.get(level - 1, 13),
            20 => self.get(level - 1, 11) + self.get(level - 1, 17),
            21 | 22 | 23 => self.get(level - 1, 17),
            24 => self.get(level - 1, 13) + self.get(level - 1, 17),
            _ => 0,
        };

        let inner_neighbors = match position {
            7 => {
                self.get(level + 1, 0)
                    + self.get(level + 1, 1)
                    + self.get(level + 1, 2)
                    + self.get(level + 1, 3)
                    + self.get(level + 1, 4)
            },
            11 => {
                self.get(level + 1, 0)
                    + self.get(level + 1, 5)
                    + self.get(level + 1, 10)
                    + self.get(level + 1, 15)
                    + self.get(level + 1, 20)
            },
            13 => {
                self.get(level + 1, 4)
                    + self.get(level + 1, 9)
                    + self.get(level + 1, 14)
                    + self.get(level + 1, 19)
                    + self.get(level + 1, 24)
            },
            17 => {
                self.get(level + 1, 20)
                    + self.get(level + 1, 21)
                    + self.get(level + 1, 22)
                    + self.get(level + 1, 23)
                    + self.get(level + 1, 24)
            },
            _ => 0,
        };
        local_neighbors as u32 + outer_neighbors as u32+ inner_neighbors as u32
    }

    fn count_bugs(&self) -> u32 {
        let mut counter: u32 = 0;
        for (level, _grid) in &self.levels {
            for position in 0..25 {
                counter += self.levels[&level][position] as u32;
            }
        }
        counter
    }
}

fn parse(filename: &str) -> [u8; 25] {
    let data = fs::read_to_string(filename).expect("No file found!");

    let vec_data: Vec<Vec<u8>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == ORIGINAL_BUG { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut level: [u8; 25] = [0; 25];
    for (index, cell) in vec_data.iter().flat_map(|row| row.iter()).enumerate() {
        level[index] = *cell;
    }

    level
}


fn solve(grids: &mut PlutonianSettlement, minutes: u32) -> u32 {

    let mut current = grids.clone();

    for _min in 1..=minutes {

        current.add_levels();
        let mut next = current.clone();

        for (level, grid) in &current.levels {

            for position in 0..25 {
                if position == 12 {
                    continue;
                }
                let cell = grid[position];
                let neighbors = current.get_neighbors(*level, position);

                if cell == BUG && neighbors != 1 {
                    next.set(*level, position, SPACE);
                } else if cell == SPACE && (neighbors == 1 || neighbors == 2) {
                    next.set(*level, position, BUG);

                } else {
                    next.set(*level, position, cell);
                }
            }
        }
        current = next;
    }
    current.count_bugs()
}

fn solution(filename: &str, minutes: u32) -> u32 {
    let grid: [u8; 25] = parse(filename);
    let mut grids: PlutonianSettlement = PlutonianSettlement::new(grid);
    solve(&mut grids, minutes)
}

fn main() {
    println!("{}", solution("./input.txt", 200)); // 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_99() {
        assert_eq!(solution("./example.txt", 10), 99);
    }
}
