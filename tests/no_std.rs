#![no_std]

use {
    bitwrap::{
        BitWrap,
        BitWrapExt,
    },
};


#[test]
fn test_readme_no_std() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bitfield(1)] flag_1: u8,
        #[bitfield(1)] flag_2: u8,
        #[bitfield(2)] data_3: u8,
        #[bitfield(12)] data_4: u16,
    }

    const DATA: &[u8] = &[0xA2, 0x34];

    let mut packet = Packet::default();
    packet.unpack(DATA).unwrap();

    assert_eq!(packet.flag_1, 1);
    assert_eq!(packet.flag_2, 0);
    assert_eq!(packet.data_3, 2);
    assert_eq!(packet.data_4, 0x0234);

    let mut buffer: [u8; 2] = [0; 2];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(buffer, DATA);
}
