use anyhow::{Context, Result};
use etherparse::{IcmpEchoHeader, Icmpv4Header, Icmpv4Slice, Icmpv4Type, Ipv4HeaderSlice};
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

    let echo_id = random();
    let echo_seq = 1;

    // --- SEND ICMP ECHO REQUEST PACKET --//

    let request = build_ping_request(echo_id, echo_seq).unwrap();
    println!("ICMP Ping Request packet:");
    hexdump(&request);

    // Note: Type::RAW requires root permissions.
    // An IMCP ping packet is a standard DGRAM packet and does not require raw sockets.
    // See https://apple.stackexchange.com/a/312861/51077
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))
        .context("Failed to create socket")?;

    socket
        .bind(&SocketAddrV4::new(src, 0).into())
        .context("Failed to bind socket")?;

    socket
        .send_to(&request, &SocketAddrV4::new(dst, 0).into())
        .context("Failed to send ICMP Ping Request packet")?;

    // --- RECEIVE ICMP ECHO RESPONSE PACKET --//

    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .context("Failed to set read timeout")?;

    let mut buffer = Vec::<u8>::with_capacity(64_000);
    let (size, addr) = socket
        .recv_from(&mut buffer.spare_capacity_mut())
        .context("Failed to receive ICMP Ping Reply packet")?;
    // SAFETY: recv_from wrote "size" number of bytes into the buffer
    unsafe {
        buffer.set_len(size);
    }

    println!(
        "ICMP Ping Reply packet from {:?} ({size} (0x{size:02x}) bytes):",
        addr.as_socket()
    );
    hexdump(&buffer[..size]);

    // decode the header
    let header =
        Ipv4HeaderSlice::from_slice(&buffer).context("Failed to parse response IPv4 header")?;
    println!(
        "IPv4 response header: payload_len={:?}",
        header.payload_len()
    );

    // For some reasons, the packet bytes returned by `recv_from` on macOS
    // have the payload length incorrectly set to 8172 bytes
    // As a result, we cannot use Ipv4Slice::from_slice(&buffer) to parse the packet
    // let packet = Ipv4Slice::from_slice(&buffer).context("Failed to parse response packet")?;
    // println!("ICMP response packet: {:?}", packet.payload());

    let header_length = header.slice().len();
    let response_slice = Icmpv4Slice::from_slice(&buffer[header_length..])
        .context("Failed to parse ICMPv4 Response header")?;

    println!(
        "ICMP Ping Response: type={} code={} type={:?}",
        response_slice.type_u8(),
        response_slice.code_u8(),
        response_slice.icmp_type()
    );

    match response_slice.icmp_type() {
        Icmpv4Type::EchoReply(echo_reply) => {
            println!(
                "echo id match? {}, seq match? {}",
                echo_reply.id == echo_id,
                echo_reply.seq == echo_seq
            );
        }
        _ => {
            println!("Unexpected ICMP type: {:?}", response_slice.icmp_type());
        }
    }

    Ok(())
}

fn build_ping_request(id: u16, seq: u16) -> Result<Vec<u8>> {
    let payload = random::<Token>();

    let echo_header = IcmpEchoHeader { id, seq };
    println!("ICMP Echo Request: {echo_header:?}");
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
