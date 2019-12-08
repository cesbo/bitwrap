# bitwrap

[![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)

## Intro

bitwrap is a derive macro and interface to declare a struct data member
with explicit size, in bits.

## Example

`bits` attribute accept only one argument - field size in bits.
For example packet has next format:

| Field | Bits |
|---|---|
| f1 | 1 |
| f2 | 1 |
| f3 | 2 |
| f4 | 4 |
| f5 | 16 |

```rust
use bitwrap::*;

#[derive(Default, BitWrap)]
struct Packet {
    #[bits(1)] f1: u8,
    #[bits(1)] f2: u8,
    #[bits(2)] f3: u8,
    #[bits(4)] f4: u8,
    #[bits(16)] f5: u16,
}

const DATA: &[u8] = &[0xAA, 0x12, 0x34];

let mut packet = Packet::default();
packet.unpack(DATA);

assert_eq!(packet.f1, 1);
assert_eq!(packet.f2, 0);
assert_eq!(packet.f3, 2);
assert_eq!(packet.f4, 0x0A);
assert_eq!(packet.f5, 0x1234);

let mut buffer: Vec<u8> = Vec::new();
packet.pack(&mut buffer);

assert_eq!(buffer.as_slice(), DATA);
```

## Nested Fields

```rust
use std::net::Ipv4Addr;
use bitwrap::*;

#[derive(BitWrap)]
struct IpAddr {
    #[bitwrap] inner: Ipv4Addr,
}

impl Default for IpAddr {
    fn default() -> Self {
        IpAddr {
            inner: Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}

#[derive(Default, BitWrap)]
struct IP4 {
    #[bits(8)] ttl: u8,
    #[bits(8)] protocol: u8,
    #[bits(16)] checksum: u16,
    #[bitwrap] src: IpAddr,
    #[bitwrap] dst: IpAddr,
}

const DATA: &[u8] = &[
    0x40, 0x88, 0x37, 0x5D, 0x8B, 0x85, 0xCC, 0xB0,
    0x8B, 0x85, 0xCC, 0xB7,
];

let mut packet = IP4::default();
packet.unpack(DATA);

assert_eq!(packet.ttl, 64);
assert_eq!(packet.protocol, 136);
assert_eq!(packet.checksum, 0x375D);
assert_eq!(packet.src.inner, Ipv4Addr::new(139, 133, 204, 176));
assert_eq!(packet.dst.inner, Ipv4Addr::new(139, 133, 204, 183));

let mut buffer: Vec<u8> = Vec::new();
packet.pack(&mut buffer);

assert_eq!(buffer.as_slice(), DATA);
```

## Skip bits

Some packets contains reserved or fixed bits.
This bits could be skiped with `bits_skip` attribute.
For example packet has next format:

| Field | Bits |
|---|---|
| Data | 6 |
| 0b00 | 2 |
| 0b1111 | 4 |
| Data | 4 |

bits_skip attribute accept next arguments:

- size in bits
- value. optional argument. by the default: 0

```rust
use bitwrap::*;

#[derive(Default, BitWrap)]
struct Packet {
    #[bits(6)] f1: u8,
    #[bits_skip(2)]
    #[bits_skip(4, 0b1111)]
    #[bits(4)] f2: u8,
}

const DATA: &[u8] = &[0xAC, 0xF5];

let mut packet = Packet::default();
packet.unpack(DATA);

assert_eq!(packet.f1, 0x2B);
assert_eq!(packet.f2, 0x05);

let mut buffer: Vec<u8> = Vec::new();
packet.pack(&mut buffer);

assert_eq!(buffer.as_slice(), DATA);
```

## TODO

- list of nested fields
