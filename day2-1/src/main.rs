use std::fs::File;
use std::io::{self, BufRead};

struct Order {
    direction: Direction,
    amount: u32,
}

enum Direction {
    Forward,
    Down,
    Up,
}

fn main() {
    let file = File::open("./input").unwrap();
    let orders: Vec<Order> = io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            let mut words = line.split(" ");
            let dir = match words.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unsupported direction"),
            };

            Order {
                direction: dir,
                amount: words.next().unwrap().parse::<u32>().unwrap(),
            }
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    for order in orders.iter() {
        match &order.direction {
            Direction::Forward => x += order.amount,
            Direction::Down => y += order.amount,
            Direction::Up => y -= order.amount,
        }
    }

    println!("Final pos is {}", x * y);
}
