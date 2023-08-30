use std::fs;
use std::collections::{HashSet, HashMap, VecDeque};
use libm;

const ASTEROID: char = '#';
const PI_F64: f64 = 3.1415926535897931160e+00;


#[derive(Debug)]
struct Meteorite {
    row: i32,
    col: i32,
    previosly_seen: i32,
    slopes_ahead: HashSet<(i32, i32)>,
}

#[derive(Debug)]
struct Meteor {
    row: i32,
    col: i32,
    distant_to_base: i32,
}

#[derive(Debug)]
struct Target {
    radian: i32,
    meteorites: VecDeque<Meteor>,
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

fn get_base(asteroids: &mut Vec<Meteorite>) -> (i32, i32) {
    let mut max_detected: i32 = 0;
    let mut base: (i32, i32) = (-1, -1);

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
            base = (asteroids[i].row, asteroids[i].col)
        }
    }
    base
}

fn slope_to_radian(y: f64, x: f64) -> f64 {
    let at: f64 = libm::atan2(y, x);
    if y < 0.0 && x < 0.0 {
        return at + PI_F64 + PI_F64;
    }
    at
}

fn get_radian_ish(base: (i32, i32), meteorite: (i32, i32)) -> f64 {
    let row_diff: i32 = meteorite.0 - base.0;
    let col_diff: i32 = meteorite.1 - base.1;
    let gcd_diff:  i32 = gcd(row_diff.abs(), col_diff.abs());
    let x: f64 = (row_diff / gcd_diff).into();
    let y: f64 = (col_diff / gcd_diff).into();

    slope_to_radian(x, y)
}

fn get_targets(base: (i32, i32), asteroids: &Vec<Meteorite>) -> Vec<Target> {
    // let base_row: i32 = base.0;
    // let base_col: i32 = base.1;

    let mut targets: HashMap<i32, Vec<Meteor>> = HashMap::new();

    for i in 0..asteroids.len() {
        if base == (asteroids[i].row, asteroids[i].col) {
            continue;
        }
        let radian: i32 = (
            get_radian_ish(base, (asteroids[i].row, asteroids[i].col)) * 1_000.0
        ) as i32;

        if targets.contains_key(&radian) {
                let distance: i32 = (asteroids[i].row - base.0).pow(2) + (asteroids[i].col - base.1).pow(2);
                targets.get_mut(&radian).unwrap().push(
                Meteor {
                    row: asteroids[i].row,
                    col: asteroids[i].col,
                    distant_to_base: distance,
                }
            );
        } else {
            targets.insert(radian,
                Vec::from([
                    Meteor {
                        row: asteroids[i].row,
                        col: asteroids[i].col,
                        distant_to_base: asteroids[i].row * asteroids[i].row + asteroids[i].col * asteroids[i].col,
                    }
                ])
            );
        }
    }
    for (_k, v) in &mut targets {
        v.sort_by_key(|x| x.distant_to_base);
    }

    let mut vec_targets: Vec<Target> = targets
        .into_iter()
        .map(|(k, v)| Target {radian: k, meteorites: VecDeque::from(v)})
        .collect();
    vec_targets.sort_by_key(|t| t.radian);

    vec_targets
}

fn solution(filename: &str) -> i32 {
    let mut asteroids: Vec<Meteorite> = parse(filename);
    let base: (i32, i32) = get_base(&mut asteroids);
    let mut vaporized_index: i32 = 0;

    let mut targets: Vec<Target> = get_targets(base, &asteroids);
    while vaporized_index < 200 {
        for target in &mut targets {
            if target.meteorites.len() > 0 {
                let meteorite: Meteor = target.meteorites.pop_front().unwrap();
                vaporized_index += 1;
                if vaporized_index == 200 {
                    return meteorite.row + meteorite.col * 100;
                }
            }
        }
    }
    -1
}

fn main() {
    println!("{}", solution("./example1.txt")); // 802
    println!("{}", solution("./input.txt"));    // 608
}
