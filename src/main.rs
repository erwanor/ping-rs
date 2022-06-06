use std::net::{IpAddr, Ipv4Addr};
use etherparse::{Ipv4HeaderSlice};
use icmp::{self, IcmpSocket};
use packet::builder::Builder;
use packet::icmp as icmp2;

struct IcmpHeader {
    msg: u8,
    code: u8,
    checksum: u16,
    data: u32,
}

impl IcmpHeader {
    pub fn ping() -> Self {
        IcmpHeader {
            msg: 8,
            code: 0,
            checksum: 0,
            data: 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let localhost = IpAddr::V4(Ipv4Addr::new(139, 178, 84, 217 ));
    let mut socket = icmp::IcmpSocket::connect(localhost).expect("failed to bind local socket");
    let request = IcmpHeader::ping();
    let mut b: [u8; 8] = [0; 8];
    b[0] = 8;
    b[2] = 8;

    let builder = icmp2::Builder::default();
    let echo_builder = builder.echo().expect("Failed to build echo");
    let packet = echo_builder.with(b).expect("Failed to build packet");

    socket.send(&b);

    let mut b2: [u8; 32] = [0; 32];

    loop {
        socket.recv(b2.as_mut());
        let header = Ipv4HeaderSlice::from_slice(&b2).expect("Couldn't read slice");
        let source = header.source_addr();

        println!("{header:?}, {source:?}");
    }
}
