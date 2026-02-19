use crate::register::Register;

#[derive(Clone, Copy)]
pub enum Interrupt {
    RxDone,
    TxDone,
    RxTimeout,
}

impl Interrupt {
    // maps an interrupt to a bit flag in RegIrqFlags
    pub fn flag(self) -> u8 {
        match self {
            Interrupt::RxDone => 0x40,
            Interrupt::RxTimeout => 0x80,
            Interrupt::TxDone => 0x08
        }
    }

    pub fn mask(self) -> u8 {
        match self {
            // maps an interrupt to a pair of bits on RegDioMappingX
            Interrupt::RxDone | Interrupt::TxDone => 0xc0, // Dio0Mapping
            Interrupt::RxTimeout => 0x30,                  // Dio1Mapping
        }
    }


    pub fn dio_mapping_addr(self) -> u8 {
        Register::RegDioMapping1.addr() // DIO0..=DIO3
        // Register::RegDioMapping2.addr() // DIO4..=DIO5
    }

    pub fn value(self) -> u8 {
        match self {
            // already shifted into position for corresponding mask
            Interrupt::RxDone | Interrupt::RxTimeout => 0x0, // 0000_0000
            Interrupt::TxDone => 0x40                        // 0100_0000
        }
    }
}