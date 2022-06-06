use std::net::{IpAddr, Ipv4Addr};

use icmp::{self, IcmpSocket};

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
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut socket = icmp::IcmpSocket::connect(localhost).expect("failed to bind local socket");
    let request = IcmpHeader::ping();
    let mut b: [u8; 8] = [0; 8];
    b[0] = 8;

    socket.send(&b);

    let mut b2: [u8; 32] = [0; 32];

    socket.recv(b2.as_mut());
    println!("{b2:?}");
}
