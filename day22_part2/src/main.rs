use std::fs;

use modinverse::modinverse;
use num_bigint::BigInt;
use regex::Regex;

#[derive(Debug)]
enum ShuffleTechnique {
    NewStack,
    Cut,
    Deal,
}

fn parse(filename: &str) -> Vec<(ShuffleTechnique, i32)> {
    let data = fs::read_to_string(filename).expect(&format!("File not found: {filename}"));

    let new_stack_re = Regex::new(r"^deal into new stack$").unwrap();
    let stack_incr_re = Regex::new(r"^deal with increment (?<value>\d+)$").unwrap();
    let cut_re = Regex::new(r"^cut (?<value>-{0,1}\w+)$").unwrap();

    let mut instructions: Vec<(ShuffleTechnique, i32)> = vec![];

    for line in data.lines() {
        let (instruction, value) = if let Some(_ins) = new_stack_re.captures(line) {
            (ShuffleTechnique::NewStack, 0)
        } else if let Some(ins) = stack_incr_re.captures(line) {
            (ShuffleTechnique::Deal, ins["value"].parse::<i32>().unwrap())
        } else if let Some(ins) = cut_re.captures(line) {
            (ShuffleTechnique::Cut, ins["value"].parse::<i32>().unwrap())
        } else {
            panic!("what the what!");
        };

        instructions.push((instruction, value));
    }

    instructions
}

fn shuffle(instructions: &Vec<(ShuffleTechnique, i32)>, size: usize, card: usize) -> usize {
    let mut card_position: usize = card;

    for (instruction, value_i32) in instructions.iter().rev() {
        let value: i64 = *value_i32 as i64;
        match instruction {
            ShuffleTechnique::NewStack => card_position = size - 1 - card_position,
            ShuffleTechnique::Deal => {
                let value_mod_inverse = modinverse(value, size as i64);
                match value_mod_inverse {
                    Some(inverse) => {
                        let card_position_big: u128 = card_position as u128;
                        let inverse_big: u128 = inverse as u128;
                        let size_big: u128 = size as u128;
                        let new_card_position_big: u128 =
                            (card_position_big * inverse_big) % size_big;

                        card_position = new_card_position_big as usize;
                    }
                    None => panic!("modinverse() didn't work as expected"),
                }
            }
            ShuffleTechnique::Cut => {
                let increment: usize = if value < 0 {
                    size - value.abs() as usize
                } else {
                    value as usize
                };
                if card_position >= size - increment as usize {
                    card_position = card_position - (size - increment);
                } else {
                    card_position = increment + card_position;
                }
            }
        }
    }
    card_position
}

fn solve(
    instructions: &Vec<(ShuffleTechnique, i32)>,
    size: usize,
    card: usize,
    times: usize,
) -> usize {
    //
    // new_position = m * position + c
    //

    let position1 = shuffle(&instructions, size, 2020);
    let position2 = shuffle(&instructions, size, 2019);

    // calculate value of m and c
    let m = position1 as i64 - position2 as i64;
    let c1 = (position1 as i64) - m * 2020;
    let c2 = (position2 as i64) - m * 2019;

    // double check c is correct
    assert_eq!(c1, c2);
    let c = c1;

    // conversion to big ints for big numbers math
    let card_bi = BigInt::from(card);
    let m_bi = BigInt::from(m.abs());
    let times_bi = BigInt::from(times);
    let size_bi = BigInt::from(size);
    let c_bi = BigInt::from(c);
    let one_bi = BigInt::from(1);

    // size is big prime inverse exists -> unwrap()
    let m_inv = modinverse(1 - m, size as i64).unwrap();

    let m_to_size_bi: BigInt = m_bi.modpow(&times_bi, &size_bi);
    let inverse_bi: BigInt = BigInt::from(m_inv);

    let mut position: BigInt =
        -m_to_size_bi.clone() * card_bi + c_bi * (one_bi + m_to_size_bi.clone()) * inverse_bi;
    position = position % size_bi;

    // convert back to u64, and then to usize
    let (_, digits) = position.to_u64_digits();

    digits[0] as usize
}

fn solution(filename: &str, size: usize, card: usize, times: usize) -> usize {
    let instructions = parse(filename);
    solve(&instructions, size, card, times)
}

fn main() {
    println!(
        "{}",
        solution("./input.txt", 119315717514047, 2020, 101741582076661) // 104073967000066
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_shuffle_part1() {
        let instructions = parse("./input.txt");

        assert_eq!(shuffle(&instructions, 10007, 3074), 2019);
    }
}
