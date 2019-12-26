use bitwrap::*;


#[test]
fn test_vec() {
    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(8)]
        len: usize,

        #[bytes(self.len)]
        data: Vec<u8>,
    }

    const DATA: &[u8] = &[0x04, 0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    packet.unpack(DATA).unwrap();
    dbg!(&packet);
}
