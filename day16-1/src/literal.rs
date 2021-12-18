use bitstream_io::{BitRead, BitReader, Endianness};
use std::io::Read;

pub struct Literal {
    bit_size: u32,
    value: u64,
}

impl Literal {
    pub fn bit_size(&self) -> u32 {
        self.bit_size
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn from<R: Read, E: Endianness>(reader: &mut BitReader<R, E>) -> Self {
        let mut size = 0;
        let mut flag = true;
        let mut ret_val = 0;
        let mut block_pos = 64 / 4;

        while flag {
            block_pos -= 1;
            flag = reader.read_bit().unwrap();
            let val: u64 = reader.read(4).unwrap();
            ret_val |= val << (block_pos * 4);
            size += 5;
        }

        let value = ret_val >> (block_pos * 4); // Clear leading zeros from unused blocks
        Literal {
            value: value,
            bit_size: size,
        }
    }
}
