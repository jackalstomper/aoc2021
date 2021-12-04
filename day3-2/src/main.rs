use std::fs::File;
use std::io::{self, BufRead};

const NUM_SIZE: usize = 12;

fn main() {
    let file = File::open("./input").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect();

    let oxy = get_value(&lines, ValueType::Oxygen);
    let co2 = get_value(&lines, ValueType::CO2);
    println!("Oxy: {}, CO2: {}, Rating: {}", oxy, co2, oxy * co2);
}

enum ValueType {
    Oxygen,
    CO2,
}

fn get_value(lines: &Vec<String>, value_type: ValueType) -> u32 {
    let mut line_ref: Vec<&String> = lines.iter().collect();
    let mut pos = 0;
    while line_ref.len() != 1 {
        let counts = get_counts(&line_ref);
        line_ref = line_ref
            .iter()
            .filter(|line| {
                let c = line.chars().nth(pos).unwrap();
                match value_type {
                    ValueType::Oxygen => {
                        if counts[pos].0 >= counts[pos].1 {
                            return c == '1';
                        } else {
                            return c == '0';
                        }
                    }
                    ValueType::CO2 => {
                        if counts[pos].0 < counts[pos].1 {
                            return c == '1';
                        } else {
                            return c == '0';
                        }
                    }
                }
            })
            .map(|e| *e)
            .collect();

        pos += 1;
    }

    u32::from_str_radix(line_ref.get(0).unwrap(), 2).unwrap()
}

fn get_counts(lines: &Vec<&String>) -> [(usize, usize); NUM_SIZE] {
    let mut counts = [(0, 0); NUM_SIZE];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if i >= NUM_SIZE {
                break;
            }

            if c == '1' {
                counts[i].0 += 1;
            } else {
                counts[i].1 += 1;
            }
        }
    }

    counts
}
