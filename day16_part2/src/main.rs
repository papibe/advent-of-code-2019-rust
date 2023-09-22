use std::fs;

fn solve(original_input: Vec<i32>, phases: i32) -> String {
    let length: i32 = original_input.len() as i32;
    let message_len: i32 = length * 10_000;
    
    let message_index = &original_input[0..7]
    .iter()
    .map(|digit| digit.to_string())
    .collect::<Vec<String>>()
    .join("")
    .parse::<i32>()
    .unwrap();

    let internal_index: i32 = message_index % length;

    let work_len: usize = (message_len - message_index) as usize;
    let mut input: Vec<i32> = vec![0; work_len];
    let mut index = internal_index as usize;

    for big_index in 0..work_len {
        input[big_index] = original_input[index];
        index = (index + 1) % length as usize;
    }
    let mut output = input.clone();

    let mut current = &mut input;
    let mut next = &mut output;

    for _phase in 1..=phases {

        let total_sum: i32 = current.iter().sum();
        next[0] = total_sum;

        for position in 1..work_len {
            next[position] = next[position - 1] - current[position - 1];
        }
        for position in 0..work_len {
            next[position] = next[position].abs() % 10;
        }
        (current, next) = (next, current);

    }
    current[..8]
        .into_iter()
        .map(|number| number.to_string())
        .collect::<String>()
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
    println!("{}", solution("./input.txt", 100));  // 36265589
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_should_be_84462026() {
        assert_eq!(solution("./example1.txt", 100), "84462026");
    }

    #[test]
    fn example2_should_be_78725270() {
        assert_eq!(solution("./example2.txt", 100), "78725270");
    }

    #[test]
    fn example3_should_be_53553731() {
        assert_eq!(solution("./example3.txt", 100), "53553731");
    }
}

