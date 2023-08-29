use std::fs;
use std::collections::HashSet;

const ASTEROID: char = '#';

#[derive(Debug)]
struct Meteorite {
    row: i32,
    col: i32,
    previosly_seen: i32,
    slopes_ahead: HashSet<(i32, i32)>,
}

impl Meteorite {
    fn slope(&self, meteorite: &Meteorite) -> (i32, i32) {
       let row_diff: i32 = meteorite.row - self.row;
       let col_diff: i32 = meteorite.col - self.col;
       let gcd_diff:  i32 = gcd(row_diff.abs(), col_diff.abs());
       (row_diff / gcd_diff, col_diff / gcd_diff)
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;
    let mut temp: i32;
    while b > 0 {
        temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn parse(filename: &str) -> Vec<Meteorite> {
    // read file
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    let matrix = data.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut asteroids: Vec<Meteorite> = Vec::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == ASTEROID {
                asteroids.push(Meteorite {row: row as i32, col: col as i32, previosly_seen: 0, slopes_ahead: HashSet::new()})
            }
        }
    }

    asteroids
}

fn solution(filename: &str) -> i32 {
    let mut asteroids: Vec<Meteorite> = parse(filename);
    let mut max_detected: i32 = 0;

    for i in 0..asteroids.len() {
        for j in i + 1.. asteroids.len() {
            let slope: (i32, i32) = asteroids[i].slope(&asteroids[j]);
            if !asteroids[i].slopes_ahead.contains(&slope) {
                asteroids[j].previosly_seen += 1;
            }
            asteroids[i].slopes_ahead.insert(slope);
        }
        let detected: i32 = asteroids[i].previosly_seen + asteroids[i].slopes_ahead.len() as i32;
        if detected > max_detected {
            max_detected = detected;
        }
    }

    max_detected
}

fn main() {
    println!("{}", solution("./input.txt"));    // 260
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example_should_be_8() {
        assert_eq!(solution("./example1.txt"), 8);
    }

    #[test]
    fn second_example_should_be_33() {
        assert_eq!(solution("./example2.txt"), 33);
    }

    #[test]
    fn third_example_should_be_35() {
        assert_eq!(solution("./example3.txt"), 35);
    }

    #[test]
    fn fourth_example_should_be_41() {
        assert_eq!(solution("./example4.txt"), 41);
    }

    #[test]
    fn big_fifh_example_should_be_210() {
        assert_eq!(solution("./example4.txt"), 41);
    }

}
