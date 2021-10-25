use bitwrap::*;


#[test]
fn test_string() {
    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bitfield(8, name = data_len, value = self.data.len())]
        #[bitfield(data_len)]
        data: String,
    }

    const DATA: &[u8] = &[0x0B, b'H', b'e', b'l', b'l', b'o', b',', b' ', 0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.data.as_str(), "Hello, 🦀");

    let mut buffer: [u8; 12] = [0; 12];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(&buffer[.. result], DATA);
}
