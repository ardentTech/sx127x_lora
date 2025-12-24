extern crate core;

use embedded_hal::spi::ErrorKind;
use sx127x_lora::*;
use sx127x_lora::Sx127xError::VersionMismatch;
use sx127x_lora::Register;

use embedded_hal_mock::eh1::digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction};
use embedded_hal_mock::eh1::MockError;
use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
use crate::RadioMode::{LongRangeMode, Sleep, Stdby};

// TODO use a macro?
fn setup() -> [embedded_hal_mock::eh1::spi::Transaction<u8>; 51] {
    [
        // reset
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        // version check
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegVersion.addr(), 0].to_vec(), [0, 0x12].to_vec()),
        SpiTransaction::transaction_end(),
        // set mode sleep
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegModemConfig1.addr(), 0].to_vec(), [0, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegModemConfig1.addr() | 0x80, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegOpMode.addr() | 0x80, LongRangeMode.addr() | Sleep.addr()].to_vec()),
        SpiTransaction::transaction_end(),
        // set frequency
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFrfMsb.addr() | 0x80, 0xe4].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFrfMid.addr() | 0x80, 0xc0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFrfLsb.addr() | 0x80, 0x0].to_vec()),
        SpiTransaction::transaction_end(),
        // clear fifo tx
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFifoTxBaseAddr.addr() | 0x80, 0x0].to_vec()),
        SpiTransaction::transaction_end(),
        // clear fifo rx
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFifoRxBaseAddr.addr() | 0x80, 0x0].to_vec()),
        SpiTransaction::transaction_end(),
        // lna
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegLna.addr(), 0].to_vec(), [0, 0x20].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegLna.addr() | 0x80, 0x23].to_vec()),
        SpiTransaction::transaction_end(),
        // modem config
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegModemConfig3.addr() | 0x80, 0x04].to_vec()),
        SpiTransaction::transaction_end(),
        // set mode standby
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegModemConfig1.addr(), 0].to_vec(), [0, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegModemConfig1.addr() | 0x80, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegOpMode.addr() | 0x80, LongRangeMode.addr() | Stdby.addr()].to_vec()),
        SpiTransaction::transaction_end(),
    ]
}

#[test]
fn new_ok() {
    let mut spi = SpiMock::new(&setup());
    let mut reset_pin = PinMock::new(&[
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ]);
    match LoRa::new(&mut spi, &mut reset_pin, 915) {
        Ok(_) => {},
        Err(e) => core::panic!("Error: {:?}", e),
    }

    reset_pin.done();
    spi.done();
}

#[test]
fn new_err_version_mismatch() {
    let invalid_version = 0x11;
    let mut spi = SpiMock::new(&[
        // reset
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        // version check
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([0x42, 0].to_vec(), [0, invalid_version].to_vec()),
        SpiTransaction::transaction_end(),
    ]);
    let mut reset_pin = PinMock::new(&[
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ]);
    match LoRa::new(&mut spi, &mut reset_pin, 915) {
        Ok(_) => core::panic!(),
        Err(e) => assert_eq!(e, VersionMismatch(invalid_version)),
    }
    reset_pin.done();
    spi.done();
}

#[test]
fn reset_ok() {
    let mut spi = SpiMock::new(
    &[
        // reset
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        // version check
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegVersion.addr(), 0].to_vec(), [0, 0x12].to_vec()),
        SpiTransaction::transaction_end(),
        // set mode sleep
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegModemConfig1.addr(), 0].to_vec(), [0, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegModemConfig1.addr() | 0x80, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegOpMode.addr() | 0x80, LongRangeMode.addr() | Sleep.addr()].to_vec()),
        SpiTransaction::transaction_end(),
        // set frequency
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFrfMsb.addr() | 0x80, 0xe4].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFrfMid.addr() | 0x80, 0xc0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFrfLsb.addr() | 0x80, 0x0].to_vec()),
        SpiTransaction::transaction_end(),
        // clear fifo tx
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFifoTxBaseAddr.addr() | 0x80, 0x0].to_vec()),
        SpiTransaction::transaction_end(),
        // clear fifo rx
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegFifoRxBaseAddr.addr() | 0x80, 0x0].to_vec()),
        SpiTransaction::transaction_end(),
        // lna
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegLna.addr(), 0].to_vec(), [0, 0x20].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegLna.addr() | 0x80, 0x23].to_vec()),
        SpiTransaction::transaction_end(),
        // modem config
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegModemConfig3.addr() | 0x80, 0x04].to_vec()),
        SpiTransaction::transaction_end(),
        // set mode standby
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer([Register::RegModemConfig1.addr(), 0].to_vec(), [0, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegModemConfig1.addr() | 0x80, 0].to_vec()),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec([Register::RegOpMode.addr() | 0x80, LongRangeMode.addr() | Stdby.addr()].to_vec()),
        SpiTransaction::transaction_end(),
        // reset
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::delay(10_000_000),
        SpiTransaction::transaction_end(),
    ]);
    let mut reset_pin = PinMock::new(&[
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ]);
    let mut lora = LoRa::new(&mut spi, &mut reset_pin, 915).unwrap();
    match lora.reset() {
        Ok(_) => {},
        Err(e) => core::panic!("Error: {:?}", e),
    }
    reset_pin.done();
    spi.done();
}