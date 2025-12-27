pub(crate) fn crc_validation_needed(reg_hop_channel: u8) -> bool {
    (reg_hop_channel >> 6 & 0x1) == 1
}

pub(crate) fn rx_packet_termination_ok(reg_irq_flags: u8) -> bool {
    (reg_irq_flags >> 4) & 0xf == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_validation_needed_false() {
        assert!(!crc_validation_needed(0b1011_1111));
    }

    #[test]
    fn crc_validation_needed_true() {
        assert!(crc_validation_needed(0b1111_1111));
    }

    #[test]
    fn rx_packet_termination_ok_false() {
        assert!(!rx_packet_termination_ok(0b1000_1111));
    }

    #[test]
    fn rx_packet_termination_ok_true() {
        assert!(rx_packet_termination_ok(0b0000_1111));
    }
}