use std::fs;
// use std::collections::HashMap;

fn parse(filename: &str) -> Vec<usize> {
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    data
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

fn solution(filename: &str, wide: usize, tall: usize) -> usize {
    let pixels: Vec<usize> = parse(filename);
    let level_size: usize = wide * tall;
    // let levels = pixels.len() / level_size;

    // let mut counter: Vec<Vec<usize>> = vec![Vec::from([0, 0, 0]); levels];
    let mut min_zeros: usize = level_size + 1;  // a big number beyond possible
    let mut min_ones: usize = 0;
    let mut min_twos: usize = 0;
    for level in (0..pixels.len()).step_by(level_size) {
        let mut zeros: usize = 0;
        let mut ones: usize = 0;
        let mut twos: usize = 0;
    
        for index in level..level + level_size {
            if pixels[index] == 0 {
                zeros += 1;
            } else if pixels[index] == 1 {
                ones += 1;
            } else if pixels[index] == 2 {
                twos += 1;
            }
        }
        if zeros <= min_zeros {
            min_zeros = zeros;
            min_ones = ones;
            min_twos = twos;
        }
    }
    min_ones * min_twos
}

fn main() {
    // println!("{}", solution("./example.txt", 3, 2));
    println!("{}", solution("./input.txt", 25, 6));
}
