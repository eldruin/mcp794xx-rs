//! This is a platform agnostic Rust driver for the MCP794xx real-time clock
//! / calendar family, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
extern crate rtcc;
pub use rtcc::{DateTime, Hours, Rtcc};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C/SPI bus error
    Comm(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// MCP794xx RTCC driver
#[derive(Debug, Default)]
pub struct Mcp794xx<DI> {
    iface: DI,
    is_enabled: bool,
}

const DEVICE_ADDRESS: u8 = 0b1101111;

struct Register;
impl Register {
    const SECONDS: u8 = 0x00;
    const MINUTES: u8 = 0x01;
    const HOURS: u8 = 0x02;
}

struct BitFlags;
impl BitFlags {
    const ST: u8 = 0b1000_0000;
    const H24_H12: u8 = 0b0100_0000;
    const AM_PM: u8 = 0b0010_0000;
}

pub mod interface;
use interface::I2cInterface;
mod common;

impl<I2C, E> Mcp794xx<I2cInterface<I2C>>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    /// Create a new instance of the MCP7940N device.
    pub fn new_mcp7940n(i2c: I2C) -> Self {
        Mcp794xx {
            iface: I2cInterface { i2c },
            is_enabled: false,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy_mcp7940n(self) -> I2C {
        self.iface.i2c
    }
}
impl<DI, E> Mcp794xx<DI>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
{
    /// Enable the oscillator (set the clock running).
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let seconds = self.iface.read_register(Register::SECONDS)?;
        self.iface
            .write_register(Register::SECONDS, seconds | BitFlags::ST)?;
        self.is_enabled = true;
        Ok(())
    }

    /// Disable the oscillator (stops the clock) (default).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let seconds = self.iface.read_register(Register::SECONDS)?;
        self.iface
            .write_register(Register::SECONDS, seconds & !BitFlags::ST)?;
        self.is_enabled = false;
        Ok(())
    }

    fn check_lt<T: PartialOrd>(value: T, reference: T) -> Result<(), Error<E>> {
        if !(value < reference) {
            Err(Error::InvalidInputData)
        } else {
            Ok(())
        }
    }
}

mod private {
    use super::interface;
    pub trait Sealed {}

    impl<E> Sealed for interface::I2cInterface<E> {}
    impl<E> Sealed for interface::ReadData<Error = E> {}
    impl<E> Sealed for interface::WriteData<Error = E> {}
}
