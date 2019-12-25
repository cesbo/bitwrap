use bitwrap::*;


#[test]
fn test_set_value() {
    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits(8, into = self.set_value)]
        value: u8,

        data: Vec<u8>
    }

    impl Packet {
        #[inline]
        fn set_value(&self, _value: u8) -> u8 { self.data.len() as u8 }
    }

    const DATA: &[u8] = &[0];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, 1);
    assert_eq!(packet.value, 0);

    packet.data.push(0x01);
    packet.data.push(0x02);
    packet.data.push(0x03);

    let mut buffer: [u8; 1] = [0; 1];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, 1);
    assert_eq!(&buffer, &[0x03]);
}
