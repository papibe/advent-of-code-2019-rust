use std::fs;

const TRANSPARENT: usize = 2;
const WHITE: usize = 0;

fn parse(filename: &str) -> Vec<usize> {
    let data = fs::read_to_string(filename)
        .expect(&format!("File not found: {filename}"));

    data
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

fn solution(filename: &str, wide: usize, tall: usize) -> String {
    let pixels: Vec<usize> = parse(filename);
    let level_size: usize = wide * tall;

    let mut image: Vec<usize> = vec![TRANSPARENT; level_size];
    for level in (0..pixels.len()).step_by(level_size) {
    
        let mut pixel_index: usize = 0;
        for index in level..level + level_size {
            if image[pixel_index] == TRANSPARENT {
                image[pixel_index] = pixels[index];
            } 
            pixel_index += 1;
        }
    }
    
    let mut string_list: Vec<String> = Vec::new();

    for i in 0..image.len() {
        if i % wide == 0 {
            string_list.push("\n".to_string());
        }
        if image[i] == WHITE {
            string_list.push(" ".to_string());
        } else {
            string_list.push("#".to_string());
        }
    }
    string_list.push("\n".to_string());

    string_list.join("")
}

fn main() {
    // println!("{}", solution("./example.txt", 2, 2)); // a slash? (/)
    println!("{}", solution("./input.txt", 25, 6)); // GJYEA
}
