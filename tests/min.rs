use bitwrap::{
    BitWrap,
    BitWrapError,
};

#[test]
fn test_min() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits(8, name = _v, value = 0x55, min = 0x55)]
        _keep: u8,
    }

    const DATA: &[u8] = &[0x50];

    let mut packet = Packet::default();
    assert_eq!(packet.unpack(DATA), Err(BitWrapError));

    let mut buffer: [u8; 1] = [0; 1];
    assert_eq!(packet.pack(&mut buffer), Ok(1));
    assert_eq!(buffer, [0x55]);

    assert_eq!(packet.unpack(&buffer), Ok(1));
}
