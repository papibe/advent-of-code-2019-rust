use std::fs;
use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}


impl Moon {
    fn gravity(&mut self, m1: &Moon, m2: &Moon) {
        for index in 0..3 {
            if m2.position[index] > m1.position[index] {
                self.position[index] += 1;
            } else if m2.position[index] < m1.position[index] {
                self.position[index] -= 1;
            }
        }
    }

    fn apply_velocity(&mut self, moon: &Moon) {
        for index in 0..3 {
            self.position[index] += moon.velocity[index];
        }    
    }

    fn set_velocity(&mut self, past_moon: &Moon) {
        for index in 0..3 {
            self.velocity[index] = self.position[index] - past_moon.position[index];
        }
    }
}


fn parse(filename: &str) -> Vec<Moon> {
    // read file
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    let re = Regex::new(r"(?m)^<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)>$").unwrap();

    let mut moons: Vec<Moon> = Vec::new();
    for (_, [x, y, z]) in re.captures_iter(data.as_str()).map(|c| c.extract()) {

        moons.push(
            Moon {
                position: [
                    x.parse::<i32>().unwrap(),
                    y.parse::<i32>().unwrap(),
                    z.parse::<i32>().unwrap(),
                ],
                velocity: [0, 0, 0],
            }
        )
    }
    moons
}

fn set_next_state(moons: &mut Vec<Moon>, next_moons: &Vec<Moon>) {
    for (index, moon) in moons.iter_mut().enumerate() {
        for i in 0..3 {
            moon.position[i] = next_moons[index].position[i];
            moon.velocity[i] = next_moons[index].velocity[i];
        }
    }
}


fn is_same_state(original_moons: &Vec<Moon>, current_moons: &Vec<Moon>, step: i32) -> bool {
    if step == 0 {
        return false;
    }
    for i in 0..original_moons.len() {
        if original_moons[i].position != current_moons[i].position {
            return false;
        }
        if original_moons[i].velocity != current_moons[i].velocity {
            return false;
        }
    }
    true
}

fn is_the_column_the_same(original_moons: &Vec<Moon>, current_moons: &Vec<Moon>, column: i32) -> bool {
    let col: usize = column as usize;

    for i in 0..original_moons.len() {
        if original_moons[i].position[col] != current_moons[i].position[col] {
            return false;
        }
        if original_moons[i].velocity[col] != current_moons[i].velocity[col] {
            return false;
        }
    }
    true
}

fn is_xs_the_same(original_moons: &Vec<Moon>, current_moons: &Vec<Moon>, step: i32) -> bool {
    if step == 0 {
        return false;
    }
    is_the_column_the_same(original_moons, current_moons, 0)
}

fn is_ys_the_same(original_moons: &Vec<Moon>, current_moons: &Vec<Moon>, step: i32) -> bool {
    if step == 0 {
        return false;
    }
    is_the_column_the_same(original_moons, current_moons, 1)
}

fn is_zs_the_same(original_moons: &Vec<Moon>, current_moons: &Vec<Moon>, step: i32) -> bool {
    if step == 0 {
        return false;
    }
    is_the_column_the_same(original_moons, current_moons, 2)
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut a = x;
    let mut b = y;
    let mut temp: i64;
    while b > 0 {
        temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(x: i64, y: i64) -> i64 {
    (x * y) / gcd(x, y)
}

fn energy(moons: &mut Vec<Moon>) -> i64 {
    let original_moons = moons.to_vec();
    let mut next_moons = moons.clone();
    let mut step: i32 = 0;

    let mut x_cycle: i64 = 0;
    let mut y_cycle: i64 = 0;
    let mut z_cycle: i64 = 0;

    while !is_same_state(&original_moons, &moons, step) {
        if x_cycle == 0 && is_xs_the_same(&original_moons, moons, step) {
            x_cycle = step as i64;
        }
        if y_cycle == 0 && is_ys_the_same(&original_moons, moons, step) {
            y_cycle = step as i64;
        }
        if z_cycle == 0 && is_zs_the_same(&original_moons, moons, step) {
            z_cycle = step as i64;
        }
        if x_cycle != 0 && y_cycle != 0 && z_cycle != 0 {
            break;
        }

        // apply gravity
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let (split1, split2) = moons.split_at_mut(i + 1);
                let moon1 = &mut split1[i];
                let moon2 = &mut split2[j - i - 1]; 
                next_moons[i].gravity(moon1, moon2);
                next_moons[j].gravity(moon2, moon1);
            }
        }
        // println!("-------------------------------------");
        for i in 0..moons.len() {
            next_moons[i].apply_velocity(&moons[i]);
        }        
        // calculate velocity
        for i in 0..moons.len() {
            next_moons[i].set_velocity(&moons[i]);
        }

        set_next_state(moons, &next_moons);
        step += 1;

    }
    lcm(lcm(x_cycle, y_cycle),z_cycle)
}

fn solution(filename: &str) -> i64 {
    let mut moons: Vec<Moon> = parse(filename);
    energy(&mut moons)
}
fn main() {
    // println!("{}", solution("./example1.txt")); // 2772
    // println!("{}", solution("./example2.txt"));    // 4686774924
    println!("{}", solution("./input.txt"));   //   467081194429464
}
