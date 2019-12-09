use bitwrap::*;


#[test]
fn test_bits_skip() {
    const DATA: &[u8] = &[0xAC, 0x55, 0xF5];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(6)] v1: u8,
        #[bits_skip(2)]
        #[bits(8)] v2: u8,
        #[bits_skip(4, 0b1111)]
        #[bits(4)] v3: u8,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA);

    assert_eq!(result, DATA.len());
    assert_eq!(packet.v1, 0x2B);
    assert_eq!(packet.v2, 0x55);
    assert_eq!(packet.v3, 0x05);

    let mut buffer: [u8; 3] = [0; 3];
    let result = packet.pack(&mut buffer);

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
