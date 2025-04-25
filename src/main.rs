use anyhow::{Context, Result};
use etherparse::{IcmpEchoHeader, Icmpv4Header, Icmpv4Type};
use rand::random;
use socket2::{Domain, Protocol, Socket, Type};
use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

const TOKEN_SIZE: usize = 24;
type Token = [u8; TOKEN_SIZE];

fn main() -> anyhow::Result<()> {
    let src = "127.0.0.1"
        .parse::<Ipv4Addr>()
        .context("Failed to parse source address")?;
    let dst = "127.0.0.1"
        .parse::<Ipv4Addr>()
        .context("Failed to parse destination address")?;

    let request = build_ping_request(random(), 1).unwrap();
    println!("ICMP Ping Request packet:");
    hexdump(&request);

    // Note: Type::RAW requires root permissions.
    // An IMCP ping packet is a standard DGRAM packet and does not require raw sockets.
    // See https://apple.stackexchange.com/a/312861/51077
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))
        .context("Failed to create socket")?;
    socket
        .set_recv_buffer_size(512)
        .context("Failed to set receive buffer size")?;

    socket
        .bind(&SocketAddrV4::new(src, 0).into())
        .context("Failed to bind socket")?;

    socket
        .send_to(&request, &SocketAddrV4::new(dst, 0).into())
        .context("Failed to send ICMP Ping Request packet")?;

    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .context("Failed to set read timeout")?;

    let mut buffer = Vec::<u8>::with_capacity(512);
    let (size, addr) = socket
        .recv_from(&mut buffer.spare_capacity_mut())
        .context("Failed to receive ICMP Ping Reply packet")?;

    println!("ICMP Ping Reply packet from {addr:?}:");
    hexdump(&buffer[..size]);

    Ok(())
}

fn build_ping_request(id: u16, seq: u16) -> Result<Vec<u8>> {
    let payload = random::<Token>();

    let echo_header = IcmpEchoHeader { id, seq };
    let icmpv4_echo = Icmpv4Header::with_checksum(Icmpv4Type::EchoRequest(echo_header), &payload);

    let mut request = icmpv4_echo.to_bytes().to_vec();
    request.write(&payload)?;
    Ok(request)
}

fn hexdump(data: &[u8]) {
    for b in data {
        print!("{b:02X} ");
    }
    println!();
}
