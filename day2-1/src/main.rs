use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input").unwrap();
    let mut x = 0;
    let mut y = 0;
    io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .for_each(|line| {
            let mut words = line.split(' ');
            let dir = words.next().unwrap();
            let amount = words.next().unwrap().parse::<u32>().unwrap();

            match dir {
                "down" => y += amount,
                "up" => y -= amount,
                "forward" => x += amount,
                _ => panic!("Unsupported direction"),
            }
        });

    println!("Final pos is {}", x * y);
}
