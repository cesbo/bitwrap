use bitwrap::*;


#[test]
fn test_vec() {
    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(8, name = data_len, value = self.data.len())]
        #[bytes(data_len)]
        data: Vec<u8>,
    }

    const DATA: &[u8] = &[0x04, 0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.data.as_slice(), &DATA[1 ..]);

    let mut buffer: [u8; 5] = [0; 5];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}


#[test]
fn test_vec_overflow() {
    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(8, name = data_len, value = self.data.len())]
        #[bytes(data_len)]
        data: Vec<u8>,
    }

    const DATA: &[u8] = &[0xFF, 0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    assert_eq!(packet.unpack(DATA), Err(BitWrapError));

    packet.data.extend_from_slice(&[0xF0, 0x9F, 0xA6, 0x80]);

    let mut buffer: [u8; 4] = [0; 4];
    assert_eq!(packet.pack(&mut buffer), Err(BitWrapError));
}
