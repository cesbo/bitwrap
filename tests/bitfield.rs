use bitwrap::*;


#[test]
fn test_bitfield() {
    const DATA: &[u8] = &[0xA5, 0x5B, 0xF5, 0x67, 0x89, 0xAF, 0xAA];

    #[derive(Default, Debug, BitWrap)]
    struct F1 {
        #[bits(4)] skip_2: u8,
        #[bits(12)] or_mask_test: u16,
    }

    #[derive(Default, Debug, BitWrap)]
    struct F2 {
        #[bits(13)] or_rshift_test: u16,
        #[bits(3)] skip_3: u8,
    }

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(6)] rshift_test: u8,
        #[bits(4)] lshift_rshift_test: u8,
        #[bits(6)] skip_1: u8,
        #[bitfield] f1: F1,
        #[bitfield] f2: F2,
        #[bits(8)] tail: u8,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA);

    assert_eq!(result, DATA.len());
    assert_eq!(packet.rshift_test, 0x29);
    assert_eq!(packet.lshift_rshift_test, 0x05);
    assert_eq!(packet.skip_1, 0x1B);
    assert_eq!(packet.f1.skip_2, 0x0F);
    assert_eq!(packet.f1.or_mask_test, 0x0567);
    assert_eq!(packet.f2.or_rshift_test, 0x1135);
    assert_eq!(packet.f2.skip_3, 0x07);
    assert_eq!(packet.tail, 0xAA);

    let mut buffer: Vec<u8> = Vec::with_capacity(256);
    let result = packet.pack(&mut buffer);

    assert_eq!(result, DATA.len());
    assert_eq!(buffer.as_slice(), DATA);
}
