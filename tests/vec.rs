use bitwrap::*;


#[test]
fn test_vec() {
    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(8, into = self.set_len)]
        len: usize,

        #[bytes(self.len)]
        data: Vec<u8>,
    }

    impl Packet {
        fn set_len(&self, _value: usize) -> u8 { self.data.len() as u8 }
    }

    const DATA: &[u8] = &[0x04, 0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.len, 4);
    assert_eq!(packet.data.as_slice(), &DATA[1 ..]);

    let mut buffer: [u8; 5] = [0; 5];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
