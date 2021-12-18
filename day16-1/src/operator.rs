use super::packet::Packet;
use bitstream_io::{BitRead, BitReader, Endianness};
use std::io::Read;

#[derive(Copy, Clone)]
enum LengthType {
    Bits = 15,
    SubPackets = 11,
}

pub struct Operator {
    bit_size: u32,
    packets: Vec<Packet>,
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

    pub fn from<R: Read, E: Endianness>(reader: &mut BitReader<R, E>) -> Self {
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

        Self {
            bit_size: size,
            packets: sub_packets,
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
}
