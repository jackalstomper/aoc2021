mod literal;
mod operator;
mod packet;

use bitstream_io::{BigEndian, BitReader};
use packet::Packet;
use std::fs::File;
use std::io::{self, BufRead, Cursor};

fn main() {
    let file = File::open("./input").unwrap();
    let buff = file_to_buff(file);
    let mut reader = BitReader::endian(Cursor::new(&buff), BigEndian);
    let p = Packet::from(&mut reader);
    println!("Value is {}", p.operate());
}

fn file_to_buff(file: File) -> Vec<u8> {
    let mut buff = Vec::new();
    for line_result in io::BufReader::new(file).lines() {
        let line = line_result.unwrap();
        let mut c_iter = line.chars();
        loop {
            // Smash two 4 bit values into a byte
            let (b1, b2) = (c_iter.next(), c_iter.next());
            if let Some(e1) = b1 {
                let byte1 = u8::from_str_radix(&e1.to_string(), 16).unwrap();
                let byte2 = match b2 {
                    Some(e2) => u8::from_str_radix(&e2.to_string(), 16).unwrap(),
                    None => 0,
                };

                buff.push((byte1 << 4) | byte2);
            } else {
                break; // Input EOF
            }
        }
    }

    buff
}
