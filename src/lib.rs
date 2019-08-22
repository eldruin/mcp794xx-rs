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
#[derive(Debug)]
pub struct Mcp794xx<DI> {
    iface: DI,
    is_enabled: bool,
    is_battery_power_enabled: bool,
    control: Config,
}

#[derive(Debug, Clone, Copy)]
struct Config {
    bits: u8,
}

const DEVICE_ADDRESS: u8 = 0b1101111;

struct Register;
impl Register {
    const SECONDS: u8 = 0x00;
    const MINUTES: u8 = 0x01;
    const HOURS: u8 = 0x02;
    const WEEKDAY: u8 = 0x03;
    const DAY: u8 = 0x04;
    const MONTH: u8 = 0x05;
    const YEAR: u8 = 0x06;
    const CONTROL: u8 = 0x07;
}

struct BitFlags;
impl BitFlags {
    const ST: u8 = 0b1000_0000;
    const H24_H12: u8 = 0b0100_0000;
    const AM_PM: u8 = 0b0010_0000;
    const VBATEN: u8 = 0b0000_1000;
    const PWRFAIL: u8 = 0b0001_0000;
    const OSCRUN: u8 = 0b0010_0000;
    const LEAPYEAR: u8 = 0b0010_0000;
    const OUT: u8 = 0b1000_0000;
    const EXTOSC: u8 = 0b0000_1000;
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
            is_battery_power_enabled: false,
            control: Config {
                bits: BitFlags::OUT,
            },
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

    /// Returns whether the oscillator is running.
    pub fn is_oscillator_running(&mut self) -> Result<bool, Error<E>> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        Ok((data & BitFlags::OSCRUN) != 0)
    }

    /// Returns whether the primary power has failed.
    pub fn has_power_failed(&mut self) -> Result<bool, Error<E>> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        Ok((data & BitFlags::PWRFAIL) != 0)
    }

    /// Clears the power failed status flag.
    pub fn clear_power_failed(&mut self) -> Result<(), Error<E>> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        let data = data & !BitFlags::PWRFAIL;
        self.iface.write_register(Register::WEEKDAY, data)
    }

    /// Enable usage of backup battery power.
    ///
    /// Note that this clears the power failed flag.
    pub fn enable_backup_battery_power(&mut self) -> Result<(), Error<E>> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        let data = data | BitFlags::VBATEN;
        self.iface.write_register(Register::WEEKDAY, data)?;
        self.is_battery_power_enabled = true;
        Ok(())
    }

    /// Disable usage of backup battery power (default).
    ///
    /// Note that this clears the power failed flag.
    pub fn disable_backup_battery_power(&mut self) -> Result<(), Error<E>> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        let data = data & !BitFlags::VBATEN;
        self.iface.write_register(Register::WEEKDAY, data)?;
        self.is_battery_power_enabled = false;
        Ok(())
    }

    /// Enable usage of external oscillator source.
    pub fn enable_external_oscillator(&mut self) -> Result<(), Error<E>> {
        self.write_control(self.control.with_high(BitFlags::EXTOSC))
    }

    /// Disable usage of external oscillator source (Will use internal source).
    pub fn disable_external_oscillator(&mut self) -> Result<(), Error<E>> {
        self.write_control(self.control.with_low(BitFlags::EXTOSC))
    }

    fn write_control(&mut self, control: Config) -> Result<(), Error<E>> {
        self.iface.write_register(Register::CONTROL, control.bits)?;
        self.control = control;
        Ok(())
    }

    fn check_lt<T: PartialOrd>(value: T, reference: T) -> Result<(), Error<E>> {
        if value < reference {
            Ok(())
        } else {
            Err(Error::InvalidInputData)
        }
    }

    fn check_gt<T: PartialOrd>(value: T, reference: T) -> Result<(), Error<E>> {
        if value > reference {
            Ok(())
        } else {
            Err(Error::InvalidInputData)
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
