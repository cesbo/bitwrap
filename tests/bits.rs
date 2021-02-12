use bitwrap::*;


#[test]
fn test_bits() {
    const DATA: &[u8] = &[0xA5, 0x5B, 0x12, 0x34, 0xF5, 0x67, 0x89, 0xAF];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bitfield(6)] rshift_test: u8,
        #[bitfield(4)] lshift_rshift_test: u8,
        #[bitfield(6)] skip_1: u8,
        #[bitfield(16)] or_test: u16,
        #[bitfield(4)] skip_2: u8,
        #[bitfield(12)] or_mask_test: u16,
        #[bitfield(13)] or_rshift_test: u16,
        #[bitfield(3)] skip_3: u8,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.rshift_test, 0x29);
    assert_eq!(packet.lshift_rshift_test, 0x05);
    assert_eq!(packet.skip_1, 0x1B);
    assert_eq!(packet.or_test, 0x1234);
    assert_eq!(packet.skip_2, 0x0F);
    assert_eq!(packet.or_mask_test, 0x0567);
    assert_eq!(packet.or_rshift_test, 0x1135);
    assert_eq!(packet.skip_3, 0x07);

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(64, 0);
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}


#[test]
fn test_bits_overflow() {
    const DATA: &[u8] = &[0x00, 0xAA];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bitfield(16)] value: u8,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();
    assert_eq!(result, DATA.len());
    assert_eq!(packet.value, 0xAA);

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(64, 0);
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}


#[test]
fn test_overflow() {
    const DATA: &[u8] = &[0xCE, 0x5B, 0x00];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bitfield(32)] value: u32,
    }

    let mut packet = Packet::default();
    match packet.unpack(DATA) {
        Err(BitWrapError) => {}
        _ => unreachable!(),
    };
}
