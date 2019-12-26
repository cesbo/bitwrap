use std::net::Ipv4Addr;
use bitwrap::BitWrap;

#[test]
fn test_readme() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Enum {
        V1,
        V2,
    }

    impl Default for Enum {
        fn default() -> Self { Enum::V1 }
    }

    impl From<u8> for Enum {
        fn from(value: u8) -> Self {
            match value {
                0x55 => Enum::V1,
                0xAA => Enum::V2,
                _ => unreachable!(),
            }
        }
    }

    impl Into<u8> for Enum {
        fn into(self) -> u8 {
            match self {
                Enum::V1 => 0x55,
                Enum::V2 => 0xAA,
            }
        }
    }

    #[derive(BitWrap)]
    struct Packet {
        // Get bit
        #[bits(1)]
        flag_1: u8,

        // Get next bit and convert into bool (0 - false, 1 - true)
        #[bits(1)]
        flag_2: bool,

        // Fixed 6 bits
        // on pack set skip value
        // on unpack just ignore them
        #[bits(6, skip = 0b111111)]

        // Get 8 bits and convert them to Enum
        // Should implemented From<u8> for Enum and Into<u8> for Enum
        #[bits(8, from = Enum::from, into = Enum::into)]
        variant: Enum,

        // call pack/unpack implemented for Ipv4Addr
        #[bits]
        ip: Ipv4Addr,

        // use from to get value from function
        #[bits(8, into = self.set_len)]
        len: u8,

        // pack/unpack
        #[bytes(self.len)]
        data: Vec<u8>,
    }

    impl Packet {
        #[inline]
        fn set_len(&self, _value: u8) -> u8 { self.data.len() as u8 }
    }

    impl Default for Packet {
        fn default() -> Self {
            Self {
                flag_1: 0,
                flag_2: false,
                variant: Enum::default(),
                ip: std::net::Ipv4Addr::new(0, 0, 0, 0),
                len: 0,
                data: Vec::default(),
            }
        }
    }

    const DATA: &[u8] = &[
        0xFF, 0xAA, 0xC0, 0xA8, 0xC8, 0xB0, 0x04, 0xF0,
        0x9F, 0xA6, 0x80,
    ];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.flag_1, 1);
    assert_eq!(packet.flag_2, true);
    assert_eq!(packet.variant, Enum::V2);
    assert_eq!(packet.ip, Ipv4Addr::new(192, 168, 200, 176));
    assert_eq!(packet.len, 4);
    assert_eq!(packet.data.as_slice(), "🦀".as_bytes());

    let mut buffer: [u8; 11] = [0; 11];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
