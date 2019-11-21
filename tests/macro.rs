use bitwrap::*;

#[test]
fn test_unpack() {
    const DATA: &[u8] = &[0x47, 0x05, 0x8d, 0x19];

    #[derive(Default, Debug, BitWrap)]
    struct Packet {
        #[bits(8)] sync_byte: u8,
        #[bits(1)] transport_error_indicator: u8,
        #[bits(1)] payload_unit_start_indicator: u8,
        #[bits(1)] transport_priority: u8,
        #[bits(13)] pid: u16,
        #[bits(2)] transport_scrambling_control: u8,
        #[bits(2)] adaptation_field_control: u8,
        #[bits(4)] continuity_counter: u8,
    }

    let mut packet = Packet::default();
    packet.unpack(DATA);
    dbg!(&packet);

    // assert_eq!(packet.sync_byte, 0x47);
    // assert_eq!(packet.transport_error_indicator, 0);
    // assert_eq!(packet.payload_unit_start_indicator, 0);
    // assert_eq!(packet.transport_priority, 0);
    // assert_eq!(packet.pid, 1421);
    // assert_eq!(packet.transport_scrambling_control, 0);
    // assert_eq!(packet.adaptation_field_control, 1);
    // assert_eq!(packet.continuity_counter, 9);
}
