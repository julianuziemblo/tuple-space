use crate::tuple::tuple::Tuple;

type Uuid = u32;

// TUPLE SPACE REQUEST TYPES
const TS_REQ_EMPTY: u32 = 0b000;
const TS_REQ_EMPTY_STR: &str = "EMPTY";
const TS_REQ_OUT: u32 = 0b001;
const TS_REQ_OUT_STR: &str = "OUT";
const TS_REQ_IN: u32 = 0b010;
const TS_REQ_IN_STR: &str = "IN";
const TS_REQ_INP: u32 = 0b011;
const TS_REQ_INP_STR: &str = "INP";
const TS_REQ_RD: u32 = 0b100;
const TS_REQ_RD_STR: &str = "RD";
const TS_REQ_RDP: u32 = 0b101;
const TS_REQ_RDP_STR: &str = "RDP";

const TS_FLAG_ACK: u32 = 0b00001;
const TS_FLAG_ACK_STR: &str = "ACK";
const TS_FLAG_RETRANSMIT: u32 = 0b00010;
const TS_FLAG_RETRANSMIT_STR: &str = "RETRANSMIT";
const TS_FLAG_KEEPALIVE: u32 = 0b00100;
const TS_FLAG_KEEPALIVE_STR: &str = "KEEPALIVE";
const TS_FLAG_HELLO: u32 = 0b01000;
const TS_FLAG_HELLO_STR: &str = "HELLO";
const TS_FLAG_ERR: u32 = 0b10000;
const TS_FLAG_ERR_STR: &str = "ERROR";

pub struct TuplePacket<'a> {
    req_type: u32,
    flags: u32,
    num: Uuid,
    tuple: Option<Tuple<'a>>,
    parity: u8,
}

impl<'a> TuplePacket<'a> {
    fn packet_uuid() -> Uuid {
        todo!("Generate random number")
    }

    pub fn calculate_parity(tuple: &Tuple) -> u8 {
        todo!("Calculate parity based on self.as_bytes()");
    }

    pub fn empty(flags: Option<u32>) -> Self {
        Self {
            req_type: TS_REQ_EMPTY,
            flags: flags.unwrap_or(0),
            num: Self::packet_uuid(),
            tuple: None,
            parity: 0,
        }
    }

    pub fn new(tuple: Tuple<'a>, req_type: u32, flags: Option<u32>) -> Self {
        Self {
            req_type,
            flags: flags.unwrap_or(0),
            num: Self::packet_uuid(),
            parity: Self::calculate_parity(&tuple),
            tuple: Some(tuple),
        }
    }
}
