use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;
    io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .for_each(|line| {
            let mut words = line.split(' ');
            let dir = words.next().unwrap();
            let amount = words.next().unwrap().parse::<u32>().unwrap();

            match dir {
                "down" => a += amount,
                "up" => a -= amount,
                "forward" => {
                    x += amount;
                    y += a * amount;
                }
                _ => panic!("Unsupported direction"),
            }
        });

    println!("Final pos is {}", x * y);
}
