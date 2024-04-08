use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

use tuple_space::tuple::consts::*;
use tuple_space::tuple::tuple::Tuple;
use tuple_space::tuple_packet::consts::*;
use tuple_space::tuple_packet::tuple_packet::{TuplePacket, TuplePacketBuilder};
use tuple_space::util::{Serializable, SliceU8};

#[allow(unused)]
const MAX_PACKET_SIZE: usize = TS_REQ_TYPE_AND_FLAGS_SIZE
    + TS_NUM_SIZE
    + TS_CHECKSUM_SIZE
    + (TUPLE_NAME_MAX_SIZE + 1)
    + (TUPLE_FIELD_MAX_SIZE * TUPLE_MAX_FIELDS);

#[derive(Clone, Copy, Debug)]
struct WorkerHandle {
    state: WorkerState,
}

#[derive(Copy, Clone, Debug)]
enum WorkerState {
    Idle,
    Active,
}

#[derive(Clone, Debug)]
pub(crate) struct Server<const N: usize> {
    addr: SocketAddrV4,
    ts_list: Vec<Tuple>,
    workers: [WorkerHandle; N],
}

impl<const N: usize> Server<N> {
    fn new(addr: SocketAddrV4) -> Self {
        Self {
            addr,
            ts_list: Vec::new(),
            workers: [WorkerHandle {
                state: WorkerState::Idle,
            }; N],
        }
    }

    fn run(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind(self.addr)?;

        println!("Server running on {:?}", self.addr);
        println!("Max packet size is: {MAX_PACKET_SIZE}");

        loop {
            let mut packet_buf: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];
            let (size, client_addr) = socket.recv_from(&mut packet_buf)?;
            println!("Received {size} bytes from client {client_addr:?}");
            println!("Packet bytes: {:b}", SliceU8(&packet_buf[..size]));
            println!(
                "String-decoded: {}",
                String::from_utf8_lossy(&packet_buf[..size])
            );

            let packet = TuplePacket::deserialize(&packet_buf[..size]);
            println!("Packet: {:?}", packet);

            let resp = match packet {
                Ok(p) => match (p.req_type, p.flags) {
                    (TS_REQ_EMPTY, TS_FLAG_HELLO) => TuplePacketBuilder::new()
                        .tuple(
                            Tuple::from_str(&format!(
                                "('{:?}')",
                                match p.tuple.clone() {
                                    Some(t) => t.name,
                                    None => "".to_owned(),
                                },
                            ))
                            .unwrap(),
                        )
                        .req_type(TS_REQ_EMPTY)
                        .flags(TS_FLAG_HELLO | TS_FLAG_ACK)
                        .num(p.increment_num())
                        .build(),

                    _ => todo!("Implement handling other requiests"),
                },
                Err(e) => TuplePacket::new(
                    Tuple::new(&format!("{e:?}"), 0),
                    TS_REQ_EMPTY,
                    Some(TS_FLAG_ERR),
                ),
            };

            println!("Sending packet: {:?}", resp);
            let _res = socket.send_to(&resp.serialize(), client_addr)?;
        }
    }
}

const SERVER_IP: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const SERVER_PORT: u16 = 2137;
const WORKERS_AMOUNT: usize = 32;
fn main() -> std::io::Result<()> {
    ctrlc::set_handler(move || {
        println!("[Ctrl+C] Closing...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let server = Server::<WORKERS_AMOUNT>::new(SocketAddrV4::new(SERVER_IP, SERVER_PORT));

    server.run()
}
