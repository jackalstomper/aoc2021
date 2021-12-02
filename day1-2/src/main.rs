use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut increases = 0u32;
    let mut last_depth = 0u32;
    let file = File::open("./input").unwrap();
    let lines: Vec<u32> = io::BufReader::new(file).lines()
        .filter_map(io::Result::ok)
        .map(|e| e.parse::<u32>().unwrap())
        .collect();
    for (i, one) in lines.iter().enumerate() {
        let two = if i + 1 >= lines.len() { 0 } else { lines[i + 1] };
        let three = if i + 2 >= lines.len() { 0 } else { lines[i + 2] };
        let sum = one + two + three;
        if sum > last_depth && i > 0 {
            increases += 1;
        }

        last_depth = sum;
    }

    println!("Depth increased {} times", increases);
}
