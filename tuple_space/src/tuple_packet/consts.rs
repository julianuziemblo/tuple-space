#[allow(unused)]
pub const TS_REQ_EMPTY: u8 = 0b000;
#[allow(unused)]
pub const TS_REQ_EMPTY_STR: &str = "EMPTY";
#[allow(unused)]
pub const TS_REQ_OUT: u8 = 0b001;
#[allow(unused)]
pub const TS_REQ_OUT_STR: &str = "OUT";
#[allow(unused)]
pub const TS_REQ_IN: u8 = 0b010;
#[allow(unused)]
pub const TS_REQ_IN_STR: &str = "IN";
#[allow(unused)]
pub const TS_REQ_INP: u8 = 0b011;
#[allow(unused)]
pub const TS_REQ_INP_STR: &str = "INP";
#[allow(unused)]
pub const TS_REQ_RD: u8 = 0b100;
#[allow(unused)]
pub const TS_REQ_RD_STR: &str = "RD";
#[allow(unused)]
pub const TS_REQ_RDP: u8 = 0b101;
#[allow(unused)]
pub const TS_REQ_RDP_STR: &str = "RDP";

// TUPLE SPACE PACKET FLAGS
#[allow(unused)]
pub const TS_FLAG_ACK: u8 = 0b00001;
#[allow(unused)]
pub const TS_FLAG_ACK_STR: &str = "ACK";
#[allow(unused)]
pub const TS_FLAG_RETRANSMIT: u8 = 0b00010;
#[allow(unused)]
pub const TS_FLAG_RETRANSMIT_STR: &str = "RETRANSMIT";
#[allow(unused)]
pub const TS_FLAG_KEEPALIVE: u8 = 0b00100;
#[allow(unused)]
pub const TS_FLAG_KEEPALIVE_STR: &str = "KEEPALIVE";
#[allow(unused)]
pub const TS_FLAG_HELLO: u8 = 0b01000;
#[allow(unused)]
pub const TS_FLAG_HELLO_STR: &str = "HELLO";
#[allow(unused)]
pub const TS_FLAG_ERR: u8 = 0b10000;
#[allow(unused)]
pub const TS_FLAG_ERR_STR: &str = "ERROR";

#[allow(unused)]
pub const TS_REQ_TYPE_AND_FLAGS_SIZE: usize = 1;
#[allow(unused)]
pub const TS_NUM_SIZE: usize = 3;
#[allow(unused)]
pub const TS_CHECKSUM_SIZE: usize = 1;
