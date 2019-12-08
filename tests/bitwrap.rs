use bitwrap::*;


#[test]
fn test_bitwrap() {
    use std::net::Ipv4Addr;

    // Test Data
    const DATA: &[u8] = &[
        0x00, 0x04, 0x76, 0xDD, 0xBB, 0x3A, 0x00, 0x04,
        0x75, 0xC7, 0x87, 0x49, 0x08, 0x00, 0x45, 0x00,
        0x00, 0x28, 0x52, 0x7E, 0x40, 0x00, 0x40, 0x88,
        0x37, 0x5D, 0x8B, 0x85, 0xCC, 0xB0, 0x8B, 0x85,
        0xCC, 0xB7,
    ];

    // Hardware address
    #[derive(Default, Debug)]
    struct HW {
        inner: [u8; 6],
    }

    impl BitWrap for HW {
        fn pack(&self, dst: &mut Vec<u8>) -> usize {
            dst.extend_from_slice(&self.inner);
            6
        }

        fn unpack<R: AsRef<[u8]>>(&mut self, src: R) -> usize {
            let src = src.as_ref();
            assert!(src.len() >= 6);
            self.inner.clone_from_slice(&src[0 .. 6]);
            6
        }
    }

    // Ethernet Header
    #[derive(Default, Debug, BitWrap)]
    struct Eth {
        #[bitwrap] dst: HW,
        #[bitwrap] src: HW,
        #[bits(16)] etype: u16,
    }

    // IPv4 Address
    #[derive(Debug, BitWrap)]
    struct IpAddr {
        #[bitwrap] inner: Ipv4Addr,
    }

    impl Default for IpAddr {
        fn default() -> Self {
            IpAddr {
                inner: Ipv4Addr::new(0, 0, 0, 0),
            }
        }
    }

    // IPv4 Packet
    #[derive(Default, Debug, BitWrap)]
    struct IP4 {
        #[bits(4)] version: u8,
        #[bits(4)] header_length: u8,
        #[bits(6)] dscp: u8,
        #[bits(2)] ecn: u8,
        #[bits(16)] total_length: u16,
        #[bits(16)] id: u16,
        #[bits_skip(1, 0)]
        #[bits(2)] flags: u8,
        #[bits(13)] offset: u16,
        #[bits(8)] ttl: u8,
        #[bits(8)] protocol: u8,
        #[bits(16)] checksum: u16,
        #[bitwrap] src: IpAddr,
        #[bitwrap] dst: IpAddr,
    }

    // Packet
    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bitwrap] eth: Eth,
        #[bitwrap] ipv4: IP4,
    }

    let mut packet = Packet::default();
    let result = packet.unpack(DATA);

    assert_eq!(result, DATA.len());
    assert_eq!(packet.eth.dst.inner, [0x00, 0x04, 0x76, 0xDD, 0xBB, 0x3A]);
    assert_eq!(packet.eth.src.inner, [0x00, 0x04, 0x75, 0xC7, 0x87, 0x49]);
    assert_eq!(packet.eth.etype, 0x0800);
    assert_eq!(packet.ipv4.version, 4);
    assert_eq!(packet.ipv4.header_length, 5);
    assert_eq!(packet.ipv4.dscp, 0);
    assert_eq!(packet.ipv4.ecn, 0);
    assert_eq!(packet.ipv4.total_length, 40);
    assert_eq!(packet.ipv4.id, 0x527E);
    assert_eq!(packet.ipv4.flags, 0b10);
    assert_eq!(packet.ipv4.offset, 0);
    assert_eq!(packet.ipv4.ttl, 64);
    assert_eq!(packet.ipv4.protocol, 136);
    assert_eq!(packet.ipv4.checksum, 0x375D);
    assert_eq!(packet.ipv4.src.inner, Ipv4Addr::new(139, 133, 204, 176));
    assert_eq!(packet.ipv4.dst.inner, Ipv4Addr::new(139, 133, 204, 183));

    let mut buffer: Vec<u8> = Vec::with_capacity(256);
    let result = packet.pack(&mut buffer);

    assert_eq!(result, DATA.len());
    assert_eq!(buffer.as_slice(), DATA);
}
