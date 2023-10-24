use std::collections::HashSet;
use std::fs;

const BUG: char = '#';
const SPACE: char = '.';

fn parse(filename: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(filename).expect("No file found!");

    data.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_biodiversity_rating(grid: &mut Vec<Vec<char>>) -> i64 {
    let rows: usize = grid.len();
    // let cols: usize = grid[0].len();

    let mut rating: usize = 0;
    for (row, a_row) in grid.iter().enumerate() {
        for (col, cell) in a_row.iter().enumerate() {
            if *cell == BUG {
                let exponent: u32 = col as u32 + ((row as u32) * (rows as u32));
                rating += 2_usize.pow(exponent);
            }
        }
    }
    rating as i64
}

fn get_bug_neighbors(row: usize, col: usize, grid: &Vec<Vec<char>>) -> usize {
    let rows: i64 = grid.len() as i64;
    let cols: i64 = grid[0].len() as i64;

    let mut neighbors: usize = 0;
    let steps = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (row_step, col_step) in steps {
        let n_row: i64 = row as i64 + row_step as i64;
        let n_col: i64 = col as i64 + col_step as i64;
        if 0 <= n_row && n_row < rows && 0 <= n_col && n_col < cols {
            if grid[n_row as usize][n_col as usize] == BUG {
                neighbors += 1;
            }
        }
    }
    neighbors
}

fn solve(grid: &Vec<Vec<char>>) -> i64 {
    let mut states: HashSet<i64> = HashSet::new();

    let mut current_state = grid.clone();
    let mut next_state = grid.clone();

    let mut current = &mut current_state;
    let mut next = &mut next_state;

    loop {
        let rating = get_biodiversity_rating(current);
        if states.contains(&rating) {
            return rating;
        }

        states.insert(rating);

        for (row, a_row) in current.iter().enumerate() {
            for (col, cell) in a_row.iter().enumerate() {
                let neighbors = get_bug_neighbors(row, col, &current);
                if *cell == BUG && neighbors != 1 {
                    next[row][col] = SPACE;
                } else if *cell == SPACE && (neighbors == 1 || neighbors == 2) {
                    next[row][col] = BUG;
                } else {
                    next[row][col] = current[row][col];
                }
            }
        }

        (current, next) = (next, current);
    }
}

fn solution(filename: &str) -> i64 {
    let grid: Vec<Vec<char>> = parse(filename);
    solve(&grid)
}

fn main() {
    println!("{}", solution("./input.txt")); // 20751345
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_2129920() {
        assert_eq!(solution("./example1.txt"), 2129920);
    }

    #[test]
    fn example2_diversity_should_be_2129920() {
        let mut grid = parse("./example2.txt");
        assert_eq!(get_biodiversity_rating(&mut grid), 2129920);
    }
}
