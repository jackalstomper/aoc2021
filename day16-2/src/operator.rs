use super::packet::Packet;
use bitstream_io::{BitRead, BitReader, Endianness};
use std::io::Read;

#[derive(Copy, Clone)]
enum LengthType {
    Bits = 15,
    SubPackets = 11,
}

enum OperationType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

trait Operation {
    fn operate(&self, operator: &Operator) -> u64;
}

pub struct Operator {
    bit_size: u32,
    packets: Vec<Packet>,
    operation: OperationType,
}

impl Operator {
    pub fn bit_size(&self) -> u32 {
        self.bit_size
    }

    pub fn packets(&self) -> &Vec<Packet> {
        &self.packets
    }

    pub fn version_sum(&self) -> u32 {
        self.packets
            .iter()
            .fold(0u32, |memo, e| memo + e.version_sum())
    }

    pub fn from<R: Read, E: Endianness>(reader: &mut BitReader<R, E>, packet_type: u8) -> Self {
        let length_type = match reader.read_bit().unwrap() {
            true => LengthType::SubPackets,
            false => LengthType::Bits,
        };

        let mut size = 1 + length_type as u32;
        let length_count: u32 = reader.read(length_type as u32).unwrap();

        let sub_packets = match length_type {
            LengthType::Bits => Self::read_by_bits(reader, length_count),
            LengthType::SubPackets => Self::read_by_subpackets(reader, length_count),
        };

        size += sub_packets.iter().fold(0u32, |memo, e| memo + e.bit_size());

        let operation = match packet_type {
            0 => OperationType::Sum,
            1 => OperationType::Product,
            2 => OperationType::Min,
            3 => OperationType::Max,
            5 => OperationType::GreaterThan,
            6 => OperationType::LessThan,
            7 => OperationType::Equal,
            _ => panic!("Unsupported operation type"),
        };

        Self {
            bit_size: size,
            packets: sub_packets,
            operation: operation,
        }
    }

    pub fn operate(&self) -> u64 {
        match &self.operation {
            OperationType::Sum => self.sum(),
            OperationType::Product => self.product(),
            OperationType::Min => self.min(),
            OperationType::Max => self.max(),
            OperationType::GreaterThan => self.greater_than(),
            OperationType::LessThan => self.less_than(),
            OperationType::Equal => self.equal(),
        }
    }

    fn read_by_bits<R, E>(reader: &mut BitReader<R, E>, bit_size: u32) -> Vec<Packet>
    where
        R: Read,
        E: Endianness,
    {
        let mut bits_read = 0;
        let mut packets = Vec::new();

        while bits_read < bit_size {
            let p = Packet::from(reader);
            bits_read += p.bit_size();
            packets.push(p);
        }

        packets
    }

    fn read_by_subpackets<R, E>(reader: &mut BitReader<R, E>, sub_count: u32) -> Vec<Packet>
    where
        R: Read,
        E: Endianness,
    {
        let mut packets = Vec::new();
        for _ in 0..sub_count {
            let p = Packet::from(reader);
            packets.push(p);
        }

        packets
    }

    pub fn sum(&self) -> u64 {
        self.packets.iter().fold(0, |memo, e| memo + e.operate())
    }

    pub fn product(&self) -> u64 {
        self.packets.iter().fold(1, |memo, e| memo * e.operate())
    }

    pub fn min(&self) -> u64 {
        match self.packets.iter().map(|e| e.operate()).min() {
            None => 0,
            Some(e) => e,
        }
    }

    pub fn max(&self) -> u64 {
        match self.packets.iter().map(|e| e.operate()).max() {
            None => 0,
            Some(e) => e,
        }
    }

    pub fn greater_than(&self) -> u64 {
        let left = self.packets[0].operate();
        let right = self.packets[1].operate();
        (left > right) as u64
    }

    pub fn less_than(&self) -> u64 {
        let left = self.packets[0].operate();
        let right = self.packets[1].operate();
        (left < right) as u64
    }

    pub fn equal(&self) -> u64 {
        let left = self.packets[0].operate();
        let right = self.packets[1].operate();
        (left == right) as u64
    }
}
