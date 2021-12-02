use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut increases = 0;
    let mut last_depth = 0;

    let file = File::open("./input").unwrap();
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        let depth = line.unwrap().parse::<u32>().unwrap();
        if depth > last_depth && i > 0 {
            increases += 1;
        }

        last_depth = depth;
    }

    println!("Depth increased {} times", increases);
}
