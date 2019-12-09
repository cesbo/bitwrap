#![no_std]


use bitwrap::*;


#[test]
fn test_bits_no_std() {
    const DATA: &[u8] = &[0xA5, 0x5B];

    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits(6)] f1: u8,
        #[bits(4)] f2: u8,
        #[bits(6)] f3: u8,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA);

    assert_eq!(result, DATA.len());
    assert_eq!(packet.f1, 0x29);
    assert_eq!(packet.f2, 0x05);
    assert_eq!(packet.f3, 0x1B);

    let mut buffer: [u8; 2] = [0; 2];
    let result = packet.pack(&mut buffer);

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
