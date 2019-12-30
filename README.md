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
    #[bits(1)]
    flag_1: u8,

    // Get/Set bit and convert into bool:
    // - 0 - false
    // - 1 - true
    #[bits(1)]
    flag_2: bool,

    // Fixed 6 bits
    // on 'pack()' set 6 bits with defined value
    // on 'unpack()' skip 6 bits
    #[bits(6, skip = 0b111111)]

    // Get 8 bits and convert them to Enum
    // on 'pack()' call 'into(Enum) -> T'
    // on 'unpack()' call 'from(T) -> Enum'
    // T is a unsigned depends of the bit field size
    #[bits(8, from = Enum::from, into = Enum::into)]
    variant: Enum,

    // call BitWrap methods for Ipv4Addr
    #[bits]
    ip: std::net::Ipv4Addr

    // byte array
    #[bytes]
    mac: [u8; 6],

    // virtual field for the bytes option
    // unpacked value should be greater or equal to 1
    // and less or equal to 10
    #[bits(8,
        name = data_len,
        value = self.data.len(),
        min = 1,
        max = 10)]

    // call BitWrap method for Vec<T> with defined
    // buffer length where T is u8 or with implemented
    // BitWrap + Default traits
    #[bytes(data_len)]
    data: Vec<u8>,
}
```
