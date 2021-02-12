#![no_std]

use bitwrap::BitWrap;


#[test]
fn test_array() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bitfield(8, name = _reserved, value = 0)]
        #[bitfield]
        data: [u8; 4],
    }

    const DATA: &[u8] = &[0x00, 0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.data, &DATA[1 ..]);

    let mut buffer: [u8; 5] = [0; 5];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(buffer, DATA);
}
