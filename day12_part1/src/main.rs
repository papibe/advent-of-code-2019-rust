use std::fs;
use regex::Regex;
// use std::rc::Rc;

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

    fn potential_energy(&self) -> i32 {
        self.position[0].abs() + self.position[1].abs() + self.position[2].abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity[0].abs() + self.velocity[1].abs() + self.velocity[2].abs()
    }

    fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
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


fn energy(moons: &mut Vec<Moon>, steps: i32) -> i32 {
    let mut next_moons = moons.clone();
    for _step in 0..steps {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let (split1, split2) = moons.split_at_mut(i + 1);
                let moon1 = &mut split1[i];
                let moon2 = &mut split2[j - i - 1]; 
                next_moons[i].gravity(moon1, moon2);
                next_moons[j].gravity(moon2, moon1);
            }
        }
        for i in 0..moons.len() {
            next_moons[i].apply_velocity(&moons[i]);
        }        
        for i in 0..moons.len() {
            next_moons[i].set_velocity(&moons[i]);
        }

        *moons = next_moons.clone();
    }

    // calculate energy of the system
    let mut total_energy: i32 = 0;
    for i in 0..moons.len() {
        total_energy += moons[i].energy();
    }
    total_energy
}

fn solution(filename: &str, steps: i32) -> i32 {
    let mut moons: Vec<Moon> = parse(filename);
    energy(&mut moons, steps)
}
fn main() {
    // println!("{}", solution("./example1.txt", 10));     // 179
    // println!("{}", solution("./example2.txt", 100));    // 1940
    println!("{}", solution("./input.txt", 1_000));   // 14907
}
