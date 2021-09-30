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
use {
    core::convert::{
        TryFrom,
        Infallible,
    },
    std::net::Ipv4Addr,
    bitwrap::{
        BitWrap,
        BitWrapExt,
        BitWrapError,
    },
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Variant { Value55, ValueAA }

impl Default for Variant {
    fn default() -> Self { Variant::Value55 }
}

impl TryFrom<u8> for Variant {
    type Error = BitWrapError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x55 => Ok(Variant::Value55),
            0xAA => Ok(Variant::ValueAA),
            _ => Err(BitWrapError),
        }
    }
}

impl TryFrom<Variant> for u8 {
    type Error = Infallible;
    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
            Variant::Value55 => Ok(0x55),
            Variant::ValueAA => Ok(0xAA),
        }
    }
}

#[derive(BitWrap)]
struct Packet {
    // single bit field
    #[bitfield(1)]
    field_1: u8,

    // bit field as boolean. 0 is false, otherwise is true
    #[bitfield(1)]
    field_2: bool,

    // virtual field with option `name`
    // unpack reads bits to the internall variable `_reserved`
    // pack sets 6 bits from defined `value`
    #[bitfield(6, name = _reserved, value = 0b111111)]

    // use TryFrom<u8> for Variant
    #[bitfield(8)]
    variant: Variant,

    // use TryFrom<u32> for Ipv4Addr
    #[bitfield(32)]
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
