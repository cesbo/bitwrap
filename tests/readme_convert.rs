#![no_std]

use bitwrap::*;


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

    impl From<u8> for Coffee {
        fn from(value: u8) -> Self {
            match value {
                0 => Coffee::Water,
                1 => Coffee::Latte,
                2 => Coffee::Cappuccino,
                3 => Coffee::Espresso,
                4 => Coffee::Americano,
                _ => unreachable!(),
            }
        }
    }

    impl Into<u8> for Coffee {
        fn into(self) -> u8 {
            match self {
                Coffee::Water => 0,
                Coffee::Latte => 1,
                Coffee::Cappuccino => 2,
                Coffee::Espresso => 3,
                Coffee::Americano => 4,
            }
        }
    }

    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits_skip(4)]
        #[bits_convert(4)] coffee: Coffee,
    }

    const DATA: &[u8] = &[0x01];

    let mut packet = Packet::default();
    packet.unpack(DATA).unwrap();

    assert_eq!(packet.coffee, Coffee::Latte);

    let mut buffer: [u8; 1] = [0; 1];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(&buffer[.. result], DATA);
}
