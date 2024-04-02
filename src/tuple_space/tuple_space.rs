use crate::{tuple::tuple::Tuple, util::Serializable};

// TUPLE SPACE REQUEST TYPES
const TS_REQ_EMPTY: u8 = 0b000;
const TS_REQ_EMPTY_STR: &str = "EMPTY";
const TS_REQ_OUT: u8 = 0b001;
const TS_REQ_OUT_STR: &str = "OUT";
const TS_REQ_IN: u8 = 0b010;
const TS_REQ_IN_STR: &str = "IN";
const TS_REQ_INP: u8 = 0b011;
const TS_REQ_INP_STR: &str = "INP";
const TS_REQ_RD: u8 = 0b100;
const TS_REQ_RD_STR: &str = "RD";
const TS_REQ_RDP: u8 = 0b101;
const TS_REQ_RDP_STR: &str = "RDP";

const TS_FLAG_ACK: u8 = 0b00001;
const TS_FLAG_ACK_STR: &str = "ACK";
const TS_FLAG_RETRANSMIT: u8 = 0b00010;
const TS_FLAG_RETRANSMIT_STR: &str = "RETRANSMIT";
const TS_FLAG_KEEPALIVE: u8 = 0b00100;
const TS_FLAG_KEEPALIVE_STR: &str = "KEEPALIVE";
const TS_FLAG_HELLO: u8 = 0b01000;
const TS_FLAG_HELLO_STR: &str = "HELLO";
const TS_FLAG_ERR: u8 = 0b10000;
const TS_FLAG_ERR_STR: &str = "ERROR";

type Uuid = u32;

// req_type: 3 bits
// flags:    5 bits
// num:     24 bits
// tuple:   variable number of bytes
// parity:   8 bits
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TuplePacket {
    req_type: u8,
    flags: u8,
    num: Uuid,
    tuple: Option<Tuple>,
    checksum: Option<u8>,
}

impl TuplePacket {
    fn packet_uuid() -> Uuid {
        rand::random::<u32>() % (2u32.pow(24) - 1)
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

    pub fn empty() -> Self {
        Self {
            req_type: TS_REQ_EMPTY,
            flags: 0,
            num: Self::packet_uuid(),
            tuple: None,
            checksum: None,
        }
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

// req_type: 3 bits
// flags:    5 bits
// num:     24 bits
// tuple:   variable number of bytes
// parity:   8 bits
impl Serializable for TuplePacket {
    type Error = ();

    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];

        // req_type & flags
        res.push(self.req_type << 5 & self.flags);

        // num
        res.extend(&self.num.to_be_bytes()[1..]);

        // tuple (if it exists)
        if let Some(t) = &self.tuple {
            res.extend(t.serialize());
        }

        // parity
        res.push(self.calculate_checksum());

        return res;
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut res = Self::empty();

        // req_type & flags
        res.req_type = (bytes[0] >> 5) & 0b0000_0111;
        res.flags = bytes[0] & 0b0001_1111;

        // num
        res.num = u32::from_be_bytes([0, bytes[1], bytes[2], bytes[3]]);

        // parity
        let &par = bytes.last().ok_or(())?;
        res.checksum = Some(par);

        // tuple
        let tup = Tuple::deserialize(&bytes[4..bytes.len() - 1]);
        match tup {
            Ok(t) => res.tuple = Some(t),
            Err(_) => return Err(()),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tuple::tuple::Tuple,
        util::{DisplayBinary, Serializable},
    };

    use super::TuplePacket;

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
}
