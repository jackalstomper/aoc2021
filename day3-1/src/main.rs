use std::fs::File;
use std::io::{self, BufRead};

const NUM_SIZE: usize = 12;

fn main() {
    let file = File::open("./input").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect();

    let mut counts = [0; NUM_SIZE];
    for line in &lines {
        for i in 0..NUM_SIZE {
            if line.chars().nth(i).unwrap() == '1' {
                counts[i] += 1;
            }
        }
    }

    let line_count = lines.len();
    let mut g = 0u32;
    let mut e = 0u32;
    for i in 0..NUM_SIZE {
        if counts[i] > line_count / 2 {
            g |= 1 << (NUM_SIZE - 1 - i);
        } else {
            e |= 1 << (NUM_SIZE - 1 - i);
        }
    }

    println!("Power consumption is {}", g * e);
}
