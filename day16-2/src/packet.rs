use super::literal::Literal;
use super::operator::Operator;
use bitstream_io::{BitRead, BitReader, Endianness};
use std::io::Read;

enum PacketType {
    Literal(Literal),
    Operator(Operator),
}

pub struct Packet {
    version: u8,
    packet_type: PacketType,
    bit_size: u32,
}

impl Packet {
    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn packet_type(&self) -> &PacketType {
        &self.packet_type
    }

    pub fn bit_size(&self) -> u32 {
        self.bit_size
    }

    pub fn operate(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(l) => l.value(),
            PacketType::Operator(o) => o.operate(),
        }
    }

    pub fn version_sum(&self) -> u32 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as u32,
            PacketType::Operator(o) => self.version as u32 + o.version_sum(),
        }
    }

    pub fn from<R: Read, E: Endianness>(reader: &mut BitReader<R, E>) -> Self {
        let version = reader.read(3).unwrap();
        let pt: u8 = reader.read(3).unwrap();
        let mut size = 6;

        let packet_type = match pt {
            4 => {
                let lit = Literal::from(reader);
                size += lit.bit_size();
                PacketType::Literal(lit)
            }
            _ => {
                let op = Operator::from(reader, pt);
                size += op.bit_size();
                PacketType::Operator(op)
            }
        };

        Self {
            version: version,
            packet_type: packet_type,
            bit_size: size,
        }
    }
}
