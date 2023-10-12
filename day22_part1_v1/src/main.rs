use regex::Regex;
use std::fs;
use std::mem::swap;

#[derive(Debug)]
struct List {
    head: usize,
    array: Vec<usize>,
    next: Vec<usize>,
    prev: Vec<usize>,
}

impl List {
    fn new(length: usize) -> Self {
        let mut array: Vec<usize> = vec![0; length];
        let mut next: Vec<usize> = vec![0; length];
        let mut prev: Vec<usize> = vec![0; length];
        for index in 0..length {
            array[index] = index;
            next[index] = (index + 1) % length;
            prev[(index + 1) % length] = index;
        }
        List {
            head: 0,
            array: array,
            next: next,
            prev: prev,
        }
    }

    fn _to_string(&self) -> String {
        let mut output: Vec<String> = vec!["".to_string(); self.array.len()];
        let mut pointer: usize = self.head;
        let mut index: usize = 0;
        let length: usize = self.array.len();

        for _ in 0..length {
            output[index] = self.array[pointer].to_string();
            pointer = self.next[pointer];
            index += 1;
        }

        output.join(" ")
    }

    fn new_stack(&mut self) {
        self.head = self.prev[self.head];

        swap(&mut self.next, &mut self.prev);
    }

    fn cut(&mut self, n: i32) {
        let mut increment: i32 = n;
        if n < 0 {
            increment = self.array.len() as i32 - n.abs();
        }
        let last: usize = self.prev[self.head];

        // get new head
        let mut new_head: usize = self.head;
        let mut previous: usize = self.head;

        for _ in 0..increment {
            previous = new_head;
            new_head = self.next[new_head];
        }

        self.next[last] = self.head;
        self.prev[self.head] = last;

        self.next[previous] = new_head;
        self.prev[new_head] = previous;

        self.head = new_head;
    }

    fn deal(&mut self, n: usize) {
        let o_array: Vec<usize> = self.array.clone();
        let mut head: usize = self.head;
        let mut table_pointer: usize = 0;
        let length: usize = self.array.len();

        for _index in 0..length {
            self.array[table_pointer] = o_array[head];
            head = self.next[head];
            table_pointer = (table_pointer + n) % length;
        }

        for index in 0..length {
            self.next[index] = (index + 1) % length;
            self.prev[(index + 1) % length] = index;
        }

        self.head = 0;
    }

    fn get_card(&self, card: usize) -> usize {
        let mut pointer: usize = self.head;
        let mut index: usize = 0;
        let length: usize = self.array.len();

        for _ in 0..length {
            if self.array[pointer] == card {
                return index;
            }
            pointer = self.next[pointer];
            index += 1;
        }
        0
    }
}

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

fn solve(instructions: &Vec<(ShuffleTechnique, i32)>, size: usize, card: usize) -> i32 {
    let mut deck: List = List::new(size);

    for (instruction, value) in instructions {
        match instruction {
            ShuffleTechnique::NewStack => deck.new_stack(),
            ShuffleTechnique::Deal => deck.deal(*value as usize),
            ShuffleTechnique::Cut => deck.cut(*value),
        }
    }
    deck.get_card(card) as i32
}

fn solution(filename: &str, size: usize, card: usize) -> i32 {
    let instructions = parse(filename);
    solve(&instructions, size, card)
}

fn main() {
    println!("{}", solution("./input.txt", 10007, 2019)); // 3074
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut list: List = List::new(10);
        list.deal(7);
        list.new_stack();
        list.new_stack();
        assert_eq!(list._to_string(), "0 3 6 9 2 5 8 1 4 7");
    }

    #[test]
    fn example2() {
        let mut list: List = List::new(10);
        list.cut(6);
        list.deal(7);
        list.new_stack();
        assert_eq!(list._to_string(), "3 0 7 4 1 8 5 2 9 6");
    }

    #[test]
    fn example3() {
        let mut list: List = List::new(10);
        list.deal(7);
        list.deal(9);
        list.cut(-2);
        assert_eq!(list._to_string(), "6 3 0 7 4 1 8 5 2 9");
    }

    #[test]
    fn example4() {
        let mut list: List = List::new(10);
        list.new_stack();
        list.cut(-2);
        list.deal(7);
        list.cut(8);
        list.cut(-4);
        list.deal(7);
        list.cut(3);
        list.deal(9);
        list.deal(3);
        list.cut(-1);
        assert_eq!(list._to_string(), "9 2 5 8 1 4 7 0 3 6");
    }
}
