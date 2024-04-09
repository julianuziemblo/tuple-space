use crate::tuple::tuple::TupleParseError;
use crate::util::{take_first_n_const, take_range};
use crate::{tuple::tuple::Tuple, util::Serializable};

use crate::tuple_packet::consts::*;

type Uuid = u32;

// req_type: 3 bits
// flags:    5 bits
// num:     24 bits
// tuple:   variable number of bytes (min. 0)
// checksum: 8 bits
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TuplePacket {
    pub req_type: u8,
    pub flags: u8,
    pub num: Uuid,
    pub tuple: Option<Tuple>,
    pub checksum: Option<u8>,
}

impl TuplePacket {
    fn packet_uuid() -> Uuid {
        rand::random::<u32>() % 2u32.pow(24)
    }

    pub fn increment_num(&self) -> u32 {
        (self.num + 1) % 2u32.pow(24)
    }

    pub fn calculate_checksum(&self) -> u8 {
        self.req_type.count_ones() as u8
            + self.flags.count_ones() as u8
            + self.num.count_ones() as u8
            + match &self.tuple {
                Some(t) => t.serialize().iter().map(|e| e.count_ones()).sum(),
                None => 0,
            } as u8
    }

    pub fn new(tuple: Tuple, req_type: u8, flags: Option<u8>) -> Self {
        Self {
            req_type,
            flags: flags.unwrap_or(0),
            num: Self::packet_uuid(),
            tuple: Some(tuple),
            checksum: None,
        }
    }
}

impl Default for TuplePacket {
    fn default() -> Self {
        Self {
            req_type: TS_REQ_EMPTY,
            flags: 0,
            num: Self::packet_uuid(),
            tuple: None,
            checksum: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TuplePacketBuilder {
    tuple_packet: TuplePacket,
}

impl TuplePacketBuilder {
    pub fn new() -> Self {
        Self {
            tuple_packet: Default::default(),
        }
    }

    pub fn req_type(mut self, req_type: u8) -> Self {
        self.tuple_packet.req_type = req_type;
        self
    }

    pub fn flags(mut self, flags: u8) -> Self {
        self.tuple_packet.flags = flags;
        self
    }

    pub fn num(mut self, num: u32) -> Self {
        self.tuple_packet.num = num;
        self
    }

    pub fn tuple(mut self, tuple: Tuple) -> Self {
        self.tuple_packet.tuple = Some(tuple);
        self
    }

    pub fn build(mut self) -> TuplePacket {
        self.tuple_packet.checksum = Some(self.tuple_packet.calculate_checksum());
        self.tuple_packet
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TuplePacketError {
    InvalidLength(usize),
    TupleParseError(TupleParseError),
}

// req_type: 3 bits
// flags:    5 bits
// num:     24 bits
// tuple:   variable number of bytes
// parity:   8 bits
impl Serializable for TuplePacket {
    type Error = TuplePacketError;

    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];

        // req_type & flags
        res.push(self.req_type << 5 | self.flags);

        // num
        res.extend(&self.num.to_be_bytes()[1..]);

        // tuple (if it exists)
        if let Some(t) = &self.tuple {
            res.extend(t.serialize());
        }

        // parity
        res.push(self.calculate_checksum());

        res
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(TuplePacket {
            req_type: (bytes
                .first()
                .ok_or(TuplePacketError::InvalidLength(bytes.len()))?
                >> 5)
                & 0b0000_0111,

            flags: bytes
                .first()
                .ok_or(TuplePacketError::InvalidLength(bytes.len()))?
                & 0b0001_1111,

            num: u32::from_be_bytes(
                take_first_n_const(bytes).map_err(|e| TuplePacketError::InvalidLength(e.0))?,
            ),

            checksum: Some(*bytes.last().ok_or(TuplePacketError::InvalidLength(0))?),

            tuple: Some(
                Tuple::deserialize(
                    take_range(bytes, 4..bytes.len() - 1)
                        .map_err(|_| TuplePacketError::InvalidLength(bytes.len()))?,
                )
                .map_err(TuplePacketError::TupleParseError)?,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{tuple::tuple::Tuple, util::Serializable};
    use std::str::FromStr;

    use super::{TuplePacket, TS_REQ_EMPTY};
    use super::{TuplePacketBuilder, TS_FLAG_HELLO};

    #[inline(always)]
    fn test_serialize(tuple: Tuple) {
        let mut packet = TuplePacket::new(tuple, 0, None);
        // println!("packet: {:?}", packet);
        packet.checksum = Some(packet.calculate_checksum());
        // println!("checksum: {}", packet.calculate_checksum());

        let packet_ser = packet.serialize();
        // println!("serialized packet: {:?}", packet_ser.display_bin());

        let packet_des = TuplePacket::deserialize(&packet_ser).unwrap();
        // println!("deserialized packet: {:?}", packet_des);
        assert_eq!(packet, packet_des);
    }

    #[test]
    fn serialize_test1() {
        let tuple = Tuple::from_str("('tuple1', int 123, float 32, int ?)").unwrap();
        // println!("Tuple: {:?}", tuple);

        test_serialize(tuple);
    }

    #[test]
    fn serialize_test2() {
        let tuple = Tuple::from_str("('tuple2')").unwrap();
        // println!("Tuple: {:?}", tuple);

        test_serialize(tuple);
    }

    #[test]
    fn serialize_test3() {
        let tuple = Tuple::from_str("('tuple3', int ?, float ?, float 2137)").unwrap();
        // println!("Tuple: {:?}", tuple);

        test_serialize(tuple);
    }

    #[test]
    fn tuple_packet_builder_test() {
        let tuple_packet1 = TuplePacketBuilder::new()
            .req_type(TS_REQ_EMPTY)
            .flags(TS_FLAG_HELLO)
            .build();
        let mut tuple_packet2 = TuplePacket::default();
        tuple_packet2.req_type = TS_REQ_EMPTY;
        tuple_packet2.flags = TS_FLAG_HELLO;

        println!("tuple_packet1: {tuple_packet1:?}");
        println!("tuple_packet2: {tuple_packet2:?}");
    }
}
