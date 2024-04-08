use std::net::UdpSocket;
// use std::str::FromStr;

use tuple_space::{
    tuple::tuple::{Tuple, TupleBuilder, TupleField},
    tuple_packet::{
        consts::{TS_FLAG_HELLO, TS_REQ_EMPTY},
        tuple_packet::{TuplePacket, TuplePacketBuilder},
    },
    util::{Serializable, SliceU8},
};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2138")?;

    let tuple = TupleBuilder::new()
        .name("chujchuj!")
        .field(TupleField::Int(None))
        .field(TupleField::Float(Some(std::f32::consts::PI)))
        .build();
    // let tuple = Tuple::new("t1", 0);

    let packet = TuplePacketBuilder::new()
        .req_type(TS_REQ_EMPTY)
        .flags(TS_FLAG_HELLO)
        .tuple(tuple)
        .build();
    println!("Packet: {packet:?}");
    let serialized: &[u8] = &packet.serialize();

    socket.send_to(serialized, "127.0.0.1:2137")?;

    println!("Sent packet: {:20b}", SliceU8(serialized));

    println!("Deserialized: {:?}", TuplePacket::deserialize(serialized));

    let mut buf: [u8; 64] = [0; 64];
    let size = socket.recv(&mut buf)?;
    let buf = &buf[..size];
    let packet = TuplePacket::deserialize(buf);
    println!("Got packet from server: {packet:?}");
    println!("bytes: {:?}", buf);

    Ok(())
}
