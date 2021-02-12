use bitwrap::*;


#[test]
fn test_readme_skip() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bitfield(6)] f1: u8,
        #[bitfield(2, value = 0)] _reserved_1: u8,
        #[bitfield(4, value = 0b1111)] _reserved_2: u8,
        #[bitfield(4)] f2: u8,
    }

    const DATA: &[u8] = &[0xAC, 0xF5];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.f1, 0x2B);
    assert_eq!(packet.f2, 0x05);

    let mut buffer: [u8; 2] = [0; 2];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
