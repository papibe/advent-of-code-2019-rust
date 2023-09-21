use std::fs;

struct Pattern {
    base_pattern: [i32; 4],
    pointer: i32,
    rep_pointer: i32,
    phase: i32,
}

impl Pattern {
    fn new(phase: i32) -> Pattern {
        Pattern {
            base_pattern: [0, 1, 0, -1],
            pointer: -1,
            rep_pointer: -1,
            phase: phase,
        }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {

        self.rep_pointer = (self.rep_pointer + 1) % self.phase;
        if self.rep_pointer == 0 {
            self.pointer = (self.pointer + 1) % self.base_pattern.len() as i32;
        }
        Some(self.base_pattern[self.pointer as usize])
    }
}

fn solve(original_input: Vec<i32>, phases: i32) -> String {
    let mut input = original_input.clone();
    let mut output = original_input.clone();
    let length: usize = input.len();

    let mut current = &mut input;
    let mut next = &mut output;

    // generate matrix of patterns
    let mut pattern: Vec<Vec<i32>> = vec![vec![0; length]; length];

    for position in 0..length {
        let mut pattern_iter: Pattern = Pattern::new((position + 1) as i32);
        let _dummy = pattern_iter.next().unwrap();  // "skip the very first value exactly once"

        for col in 0..length {
            pattern[position][col] = pattern_iter.next().unwrap();
        }
    }

    for _phase in 1..=phases {

        for position in 0..length {
            
            let mut output_value: i32 = 0;
            for (index, number) in current.iter().enumerate() {
                output_value += number * pattern[position][index];
            }
            output_value = output_value.abs() % 10;
            next[position] = output_value;
        }
        (current, next) = (next, current);
    }
    current
        .into_iter()
        .map(|number| number.to_string())
        .collect::<String>()[..8].to_string()
}

fn parse(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename).expect("file error");

    data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}

fn solution(filename: &str, phases: i32) -> String {
    let input = parse(filename);
    solve(input, phases)
}

fn main() {
    println!("{}", solution("./input.txt", 100));  // 27831665
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_01029498() {
        assert_eq!(solution("./example1.txt", 4), "01029498");
    }

    #[test]
    fn example2_should_be_24176176() {
        assert_eq!(solution("./example2.txt", 100), "24176176");
    }

    #[test]
    fn example3_should_be_73745418() {
        assert_eq!(solution("./example3.txt", 100), "73745418");
    }
    #[test]
    fn example4_should_be_52432133() {
        assert_eq!(solution("./example4.txt", 100), "52432133");
    }
}

