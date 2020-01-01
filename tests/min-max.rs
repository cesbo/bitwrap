use bitwrap::{
    BitWrap,
    BitWrapError,
};

#[test]
fn test_min() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits(8,
            name = _v,
            value = 0x55,
            min = 0x55,
            max = 0xAA)]
        _keep: u8,
    }

    const DATA_1: &[u8] = &[0x50];

    let mut packet = Packet::default();
    assert_eq!(packet.unpack(DATA_1), Err(BitWrapError));

    const DATA_2: &[u8] = &[0xFF];

    let mut packet = Packet::default();
    assert_eq!(packet.unpack(DATA_2), Err(BitWrapError));

    let mut buffer: [u8; 1] = [0; 1];
    assert_eq!(packet.pack(&mut buffer), Ok(1));
    assert_eq!(buffer, [0x55]);

    assert_eq!(packet.unpack(&buffer), Ok(1));
}
