# bitwrap

[![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)

## Intro

bitwrap is a derive macro and interface to declare a struct data member
with explicit size, in bits.

## Example

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
use bitwrap::*;

#[derive(Default, BitWrap)]
struct Field {
    #[bits(1)] f1: u8,
    #[bits(1)] f2: u8,
    #[bits(2)] f3: u8,
    #[bits(4)] f4: u8,
}

#[derive(Default, BitWrap)]
struct Packet {
    #[bitfield] nested: Field,
    #[bits(16)] f1: u16,
}

const DATA: &[u8] = &[0xAA, 0x12, 0x34];

let mut packet = Packet::default();
packet.unpack(DATA);

assert_eq!(packet.nested.f1, 1);
assert_eq!(packet.nested.f2, 0);
assert_eq!(packet.nested.f3, 2);
assert_eq!(packet.nested.f4, 0x0A);
assert_eq!(packet.f1, 0x1234);

let mut buffer: Vec<u8> = Vec::new();
packet.pack(&mut buffer);

assert_eq!(buffer.as_slice(), DATA);
```

## TODO

- little-endian
- list of nested fields
