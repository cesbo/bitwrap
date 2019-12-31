use bitwrap::{
    BitWrap,
    BitWrapError,
};

#[test]
fn test_eq() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits(8, name = _v, value = 0x55, eq = 0x55)]
        _keep: u8,
    }

    const DATA: &[u8] = &[0xAA];

    let mut packet = Packet::default();
    match packet.unpack(DATA) {
        Err(BitWrapError) => {}
        _ => unreachable!(),
    }

    let mut buffer: [u8; 1] = [0; 1];
    assert_eq!(packet.pack(&mut buffer), Ok(1));
    assert_eq!(buffer, [0x55]);

    assert_eq!(packet.unpack(&buffer), Ok(1));
}
