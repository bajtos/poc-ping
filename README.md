# poc-ping

```
❯ cargo run
Running `target/debug/poc-ping`
ICMP Echo Request: IcmpEchoHeader { id: 56604, seq: 1 }
ICMP Ping Request packet:
08 00 70 E8 DD 1C 00 01 23 C5 17 50 57 C2 01 15 8B 97 CE 7E 7C A4 D0 F3 37 5C 6D F1 00 78 C8 98
ICMP Ping Reply from Some(127.0.0.1:0) (52 (0x34) bytes):
45 00 20 00 1B 66 00 00 40 01 00 00 7F 00 00 01 7F 00 00 01 00 00 78 E8 DD 1C 00 01 23 C5 17 50 57 C2 01 15 8B 97 CE 7E 7C A4 D0 F3 37 5C 6D F1 00 78 C8 98
IPv4 response header: payload_len=Ok(8172)
ICMP Ping Response: type=0 code=0 type=EchoReply(IcmpEchoHeader { id: 56604, seq: 1 })
echo id match? true, seq match? true
```

## Resources

- https://github.com/JulianSchmid/etherparse
- https://github.com/xphoniex/icmp-rust
- https://github.com/knsd/tokio-ping
- https://github.com/ruxo/ping-rs
- https://github.com/bparli/fastping-rs/
- [macOS ping](https://github.com/apple-oss-distributions/network_cmds/blob/network_cmds-698.60.4/ping.tproj/ping.c)
- https://github.com/rust-lang/socket2/pull/532

  Note: it seems that WASIp2 does not support ICMP :(
  https://github.com/WebAssembly/wasi-sockets/blob/2e96a2ff547ac2955ff7e16e9964462d9d483c84/wit/udp-create-socket.wit
