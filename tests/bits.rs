use bitwrap::*;


#[test]
fn test_bits() {
    const DATA: &[u8] = &[0xA5, 0x5B, 0x12, 0x34, 0xF5, 0x67, 0x89, 0xAF];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(6)] rshift_test: u8,
        #[bits(4)] lshift_rshift_test: u8,
        #[bits(6)] skip_1: u8,
        #[bits(16)] or_test: u16,
        #[bits(4)] skip_2: u8,
        #[bits(12)] or_mask_test: u16,
        #[bits(13)] or_rshift_test: u16,
        #[bits(3)] skip_3: u8,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA);

    assert_eq!(result, DATA.len());
    assert_eq!(packet.rshift_test, 0x29);
    assert_eq!(packet.lshift_rshift_test, 0x05);
    assert_eq!(packet.skip_1, 0x1B);
    assert_eq!(packet.or_test, 0x1234);
    assert_eq!(packet.skip_2, 0x0F);
    assert_eq!(packet.or_mask_test, 0x0567);
    assert_eq!(packet.or_rshift_test, 0x1135);
    assert_eq!(packet.skip_3, 0x07);

    let mut buffer: [u8; 8] = [0; 8];
    let result = packet.pack(&mut buffer);

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}


#[test]
fn test_overflow() {
    const DATA: &[u8] = &[0xCE, 0x5B, 0x00];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(32)] value: u32,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA);
    assert_eq!(result, 4);
}
