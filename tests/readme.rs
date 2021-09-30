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


#[test]
fn test_readme() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Variant {
        Value55,
        ValueAA,
    }

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
        #[bitfield(1)]
        field_1: u8,

        #[bitfield(1)]
        field_2: bool,

        #[bitfield(6, name = _reserved, value = 0b111111)]

        #[bitfield(8)]
        variant: Variant,

        #[bitfield(32)]
        ip: Ipv4Addr,

        #[bitfield]
        mac: [u8; 6],

        #[bitfield(8, name = data_len, value = self.data.len())]

        #[bitfield(data_len)]
        data: Vec<u8>,
    }

    impl Default for Packet {
        fn default() -> Self {
            Self {
                field_1: 0,
                field_2: false,
                variant: Variant::Value55,
                ip: std::net::Ipv4Addr::new(0, 0, 0, 0),
                mac: [0; 6],
                data: Vec::default(),
            }
        }
    }

    const DATA: &[u8] = &[
        0xFF,
        0xAA,
        0xC0, 0xA8, 0xC8, 0xB0,
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x04,
        0xF0, 0x9F, 0xA6, 0x80,
    ];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.field_1, 1);
    assert!(packet.field_2);
    assert_eq!(packet.variant, Variant::ValueAA);
    assert_eq!(packet.ip, Ipv4Addr::new(192, 168, 200, 176));
    assert_eq!(packet.mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    assert_eq!(packet.data.as_slice(), "🦀".as_bytes());

    let mut buffer: [u8; 256] = [0; 256];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
