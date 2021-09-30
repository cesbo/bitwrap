# BitWrap

[![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)

BitWrap is a derive macro and trait to declare a struct data member
with explicit size, in bits.

---

## BitWrapExt Trait

Trait declares 2 methods:

```rust
fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError>
```

`pack` method serialize struct fields into `dst` array

```rust
fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError>
```

`unpack` method deserialize struct fields from `src` array

## BitWrap Macro

```rust
use bitwrap::{BitWrap, BitWrapExt};

#[derive(BitWrap)]
struct Packet {
    // single bit field
    #[bitfield(1)]
    field_1: u8,

    // single bit field as boolean:
    // - 0 - false
    // - 1 - true
    #[bitfield]
    field_2: bool,

    // virtual field with option `name`
    // unpack reads bits to the internall variable `_reserved`
    // pack sets 6 bits from defined `value`
    #[bitfield(6, name = _reserved, value = 0b111111)]

    // call BitWrapExt methods for Ipv4Addr
    #[bitfield]
    ip: std::net::Ipv4Addr

    // byte array
    #[bitfield]
    mac: [u8; 6],

    // virtual field with optn `name` to define buffer length
    #[bitfield(8, name = data_len, value = self.data.len())]

    // get slice of `data_len` bytes and call BitWrapExt method for Vec<T>
    // where T is u8 or with implemented BitWrapExt + Default traits
    #[bitfield(data_len)]
    data: Vec<u8>,
}
```
