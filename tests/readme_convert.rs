use bitwrap::*;


#[test]
fn test_readme_convert() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits_skip(6, 0)]
        #[bits(1, convert(u8, from_bool, to_bool))] flag_1: bool,
        #[bits(1, convert(u8, from_bool, to_bool))] flag_2: bool,
    }

    impl Packet {
        #[inline]
        fn from_bool(value: bool) -> u8 { if value { 1 } else { 0 } }

        #[inline]
        fn to_bool(value: u8) -> bool { value != 0 }
    }

    const DATA: &[u8] = &[0x02];

    let mut packet = Packet::default();
    packet.unpack(DATA);

    assert_eq!(packet.flag_1, true);
    assert_eq!(packet.flag_2, false);

    let mut buffer: [u8; 1] = [0; 1];
    let result = packet.pack(&mut buffer);

    assert_eq!(result, DATA.len());
    assert_eq!(buffer, DATA);
}
