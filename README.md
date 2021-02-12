# BitWrap

[![docs](https://docs.rs/bitwrap/badge.svg)](https://docs.rs/bitwrap)

BitWrap is a derive macro and trait to declare a struct data member
with explicit size, in bits.

---

## BitWrap Trait

BitWrap trait declares 2 methods:

```rust
fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError>
```

`pack` method serialize struct fields into dst array

```rust
fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError>
```

`unpack` method deserialize struct fields from src array

## BitWrap Macro

```rust
use bitwrap::BitWrap;

#[derive(BitWrap)]
struct Packet {
    // Get/Set bit
    #[bitfield(1)]
    flag_1: u8,

    // Get/Set bit and convert into bool:
    // - 0 - false
    // - 1 - true
    #[bitfield(1)]
    flag_2: bool,

    // virtual field with option `name` to skip reserved bits
    // on 'pack()' set 6 bits with defined value
    #[bitfield(6, name = _reserved, value = 0b111111)]

    // Get 8 bits and convert them to Enum
    // on 'pack()' call 'into(Enum) -> T'
    // on 'unpack()' call 'from(T) -> Enum'
    // T is a unsigned depends of the bit field size
    #[bitfield(8, from = Enum::from, into = Enum::into)]
    variant: Enum,

    // call BitWrap methods for Ipv4Addr
    #[bitfield]
    ip: std::net::Ipv4Addr

    // byte array
    #[bitfield]
    mac: [u8; 6],

    // virtual field with optn `name` to define buffer length
    #[bitfield(8, name = data_len, value = self.data.len())]

    // get slice of `data_len` bytes and call BitWrap method for Vec<T>
    // where T is u8 or with implemented BitWrap + Default traits
    #[bitfield(data_len)]
    data: Vec<u8>,
}
```
