#![no_std]

use {
    core::convert::{
        TryFrom,
        Infallible,
    },
    bitwrap::*,
};


#[test]
fn test_readme_convert() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Coffee {
        Water,
        Latte,
        Cappuccino,
        Espresso,
        Americano,
    }

    impl Default for Coffee {
        fn default() -> Self { Coffee::Water }
    }

    impl TryFrom<u8> for Coffee {
        type Error = BitWrapError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Coffee::Water),
                1 => Ok(Coffee::Latte),
                2 => Ok(Coffee::Cappuccino),
                3 => Ok(Coffee::Espresso),
                4 => Ok(Coffee::Americano),
                _ => Err(BitWrapError),
            }
        }
    }

    impl TryFrom<Coffee> for u8 {
        type Error = Infallible;

        fn try_from(value: Coffee) -> Result<Self, Self::Error> {
            match value {
                Coffee::Water => Ok(0),
                Coffee::Latte => Ok(1),
                Coffee::Cappuccino => Ok(2),
                Coffee::Espresso => Ok(3),
                Coffee::Americano => Ok(4),
            }
        }
    }

    #[derive(Default, BitWrap)]
    struct Packet {
        #[bitfield(4, name = _reserved, value = 0)]
        #[bitfield(4)]
        coffee: Coffee,
    }

    const DATA: &[u8] = &[0x01];

    let mut packet = Packet::default();
    packet.unpack(DATA).unwrap();

    assert_eq!(packet.coffee, Coffee::Latte);

    let mut buffer: [u8; 1] = [0; 1];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(&buffer[.. result], DATA);
}
