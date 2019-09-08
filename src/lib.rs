//! This is a platform agnostic Rust driver for the MCP794xx real-time clock
//! / calendar family, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
extern crate rtcc;
use core::marker::PhantomData;
pub use rtcc::{DateTime, Hours, Rtcc};

/// Feature markers
pub mod marker {
    use super::private;
    /// Supports backup battery power
    pub trait WithBatteryPower: private::Sealed {}
    /// Supports protected EEPROM
    pub trait WithProtectedEeprom: private::Sealed {}
    /// Contains EUI-48
    pub trait WithEui48: private::Sealed {}
    /// Contains EUI-64
    pub trait WithEui64: private::Sealed {}
}

/// IC markers
pub mod ic {
    /// MCP7940N IC marker
    pub struct Mcp7940n(());
    /// MCP7940M IC marker
    pub struct Mcp7940m(());
    /// MCP79400 IC marker
    pub struct Mcp79400(());
    /// MCP79401 IC marker
    pub struct Mcp79401(());
    /// MCP79402 IC marker
    pub struct Mcp79402(());
    /// MCP79410 IC marker
    pub struct Mcp79410(());
    /// MCP79411 IC marker
    pub struct Mcp79411(());
    /// MCP79412 IC marker
    pub struct Mcp79412(());
}

impl marker::WithBatteryPower for ic::Mcp7940n {}
impl marker::WithBatteryPower for ic::Mcp79400 {}
impl marker::WithBatteryPower for ic::Mcp79401 {}
impl marker::WithBatteryPower for ic::Mcp79402 {}
impl marker::WithBatteryPower for ic::Mcp79410 {}
impl marker::WithBatteryPower for ic::Mcp79411 {}
impl marker::WithBatteryPower for ic::Mcp79412 {}
impl marker::WithProtectedEeprom for ic::Mcp79400 {}
impl marker::WithProtectedEeprom for ic::Mcp79401 {}
impl marker::WithProtectedEeprom for ic::Mcp79402 {}
impl marker::WithProtectedEeprom for ic::Mcp79410 {}
impl marker::WithProtectedEeprom for ic::Mcp79411 {}
impl marker::WithProtectedEeprom for ic::Mcp79412 {}
impl marker::WithEui48 for ic::Mcp79401 {}
impl marker::WithEui48 for ic::Mcp79411 {}
impl marker::WithEui64 for ic::Mcp79402 {}
impl marker::WithEui64 for ic::Mcp79412 {}

mod types;
pub use types::{
    Alarm, AlarmDateTime, AlarmMatching, AlarmOutputPinPolarity, Error, OutputPinLevel,
    PowerFailDateTime, SqWFreq,
};

const DEVICE_ADDRESS: u8 = 0b110_1111;
const EEPROM_ADDRESS: u8 = 0b101_0111;

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
    const OSCTRIM: u8 = 0x08;
    const ALM0SEC: u8 = 0x0A;
    const ALM1SEC: u8 = 0x11;
    const ALM0WKDAY: u8 = 0x0D;
    const ALM1WKDAY: u8 = 0x14;
    const PWRDNMIN: u8 = 0x18;
    const PWRUPMIN: u8 = 0x1C;
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
    const SQWEN: u8 = 0b0100_0000;
    const EXTOSC: u8 = 0b0000_1000;
    const CRSTRIM: u8 = 0b0000_0100;
    const ALMPOL: u8 = 0b1000_0000;
    const ALM0EN: u8 = 0b0001_0000;
    const ALM1EN: u8 = 0b0010_0000;
    const ALMIF: u8 = 0b0000_1000;
}

/// MCP794xx RTCC driver
#[derive(Debug)]
pub struct Mcp794xx<DI, IC> {
    iface: DI,
    is_enabled: bool,
    is_battery_power_enabled: bool,
    is_running_in_24h_mode: bool,
    control: Config,
    alarm_output_pin_polarity: AlarmOutputPinPolarity,
    _ic: PhantomData<IC>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Config {
    bits: u8,
}

pub mod interface;
use interface::I2cInterface;
mod battery_power;
mod common;
mod eeprom;

macro_rules! create_destroy_i2c {
    ($ic:ident, $create:ident, $destroy:ident) => {
        impl<I2C, E> Mcp794xx<I2cInterface<I2C>, ic::$ic>
        where
            I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
        {
            /// Create a new instance of the device.
            pub fn $create(i2c: I2C) -> Self {
                Mcp794xx {
                    iface: I2cInterface { i2c },
                    is_enabled: false,
                    is_battery_power_enabled: false,
                    is_running_in_24h_mode: false,
                    control: Config {
                        bits: BitFlags::OUT,
                    },
                    alarm_output_pin_polarity: AlarmOutputPinPolarity::Low,
                    _ic: PhantomData,
                }
            }

            /// Destroy driver instance, return I²C bus instance.
            pub fn $destroy(self) -> I2C {
                self.iface.i2c
            }
        }
    };
}
create_destroy_i2c!(Mcp7940n, new_mcp7940n, destroy_mcp7940n);
create_destroy_i2c!(Mcp7940m, new_mcp7940m, destroy_mcp7940m);
create_destroy_i2c!(Mcp79400, new_mcp79400, destroy_mcp79400);
create_destroy_i2c!(Mcp79401, new_mcp79401, destroy_mcp79401);
create_destroy_i2c!(Mcp79402, new_mcp79402, destroy_mcp79402);
create_destroy_i2c!(Mcp79410, new_mcp79410, destroy_mcp79410);
create_destroy_i2c!(Mcp79411, new_mcp79411, destroy_mcp79411);
create_destroy_i2c!(Mcp79412, new_mcp79412, destroy_mcp79412);

mod private {
    use super::{ic, interface};
    pub trait Sealed {}

    impl<E> Sealed for interface::I2cInterface<E> {}
    impl Sealed for ic::Mcp7940n {}
    impl Sealed for ic::Mcp7940m {}
    impl Sealed for ic::Mcp79400 {}
    impl Sealed for ic::Mcp79401 {}
    impl Sealed for ic::Mcp79402 {}
    impl Sealed for ic::Mcp79410 {}
    impl Sealed for ic::Mcp79411 {}
    impl Sealed for ic::Mcp79412 {}
}
