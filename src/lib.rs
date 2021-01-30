//! This is a platform agnostic Rust driver for the MCP794xx real-time clock
//! / calendar family, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read and set date and time. See: [`get_datetime()`].
//! - Read and set date. See: [`get_date()`].
//! - Read and set time. See: [`get_time()`].
//! - Read and set date and time individual elements. For example, see: [`get_year()`].
//! - Enable and disable the real-time clock. See: [`enable()`].
//! - Read whether the oscillator is running. See: [`is_oscillator_running()`].
//! - Read whether the current year is a leap year. See: [`is_leap_year()`].
//! - Enable and disable the usage of an external oscillator source. See: [`enable_external_oscillator()`].
//! - Set the output pin logic level. See: [`set_output_pin()`].
//! - Enable and disable coarse trim. See: [`enable_coarse_trim()`].
//! - Set trimming value. See: [`set_trimming()`].
//! - Power:
//!     - Read whether the power has failed. See: [`has_power_failed()`].
//!     - Clear the has-power-failed flag. See: [`clear_power_failed()`].
//!     - Read the date/time when power went down. See: [`get_power_down_datetime()`].
//!     - Read the date/time when power went back up. See: [`get_power_up_datetime()`].
//!     - Enable and disable usage of backup battery power. See: [`enable_backup_battery_power()`].
//! - SRAM:
//!     - Read and write byte to SRAM. See: [`read_sram_byte()`].
//!     - Read and write byte array to SRAM. See: [`read_sram_data()`].
//!     - Read current position from SRAM. See: [`read_sram_current_byte()`].
//! - Alarms:
//!     - Enable and disable alarms. See: [`enable_alarm()`].
//!     - Set alarms with several matching policies and output pin polarities. See: [`set_alarm()`].
//!     - Read whether alarms have matched. See: [`has_alarm_matched()`].
//!     - Clear flag indicating that alarms have matched. See: [`clear_alarm_matched_flag()`].
//! - Wave generation:
//!     - Enable and disable the square-wave generation. See: [`enable_square_wave()`].
//!     - Select the square-wave frequency. See: [`set_square_wave_frequency()`].
//! - Protected EEPROM:
//!     - Read and write byte to the protected EEPROM. See: [`read_protected_eeprom_byte()`].
//!     - Read and write byte array to the protected EEPROM. See: [`read_protected_eeprom_data()`].
//!     - Read EUI-48. See: [`read_eui48()`].
//!     - Read EUI-64. See: [`read_eui64()`].
//! - EEPROM:
//!     - Read and write byte to the EEPROM. See: [`read_eeprom_byte()`].
//!     - Read and write byte array to the EEPROM. See: [`read_eeprom_data()`].
//!     - Set EEPROM block write protection. See: [`set_eeprom_write_protection()`].
//!     - Read current position from the EEPROM. See: [`read_eeprom_current_byte()`].
//!
//! [`get_datetime()`]: struct.Mcp794xx.html#method.get_datetime
//! [`get_date()`]: struct.Mcp794xx.html#method.get_date
//! [`get_time()`]: struct.Mcp794xx.html#method.get_time
//! [`get_year()`]: struct.Mcp794xx.html#method.get_year
//! [`enable()`]: struct.Mcp794xx.html#method.enable
//! [`is_oscillator_running()`]: struct.Mcp794xx.html#method.is_oscillator_running
//! [`is_leap_year()`]: struct.Mcp794xx.html#method.is_leap_year
//! [`enable_external_oscillator()`]: struct.Mcp794xx.html#method.enable_external_oscillator
//! [`set_output_pin()`]: struct.Mcp794xx.html#method.set_output_pin
//! [`enable_coarse_trim()`]: struct.Mcp794xx.html#method.enable_coarse_trim
//! [`set_trimming()`]: struct.Mcp794xx.html#method.set_trimming
//! [`has_power_failed()`]: struct.Mcp794xx.html#method.has_power_failed
//! [`clear_power_failed()`]: struct.Mcp794xx.html#method.clear_power_failed
//! [`get_power_down_datetime()`]: struct.Mcp794xx.html#method.get_power_down_datetime
//! [`get_power_up_datetime()`]: struct.Mcp794xx.html#method.get_power_up_datetime
//! [`enable_backup_battery_power()`]: struct.Mcp794xx.html#method.enable_backup_battery_power
//! [`read_sram_byte()`]: struct.Mcp794xx.html#method.read_sram_byte
//! [`read_sram_data()`]: struct.Mcp794xx.html#method.read_sram_data
//! [`read_sram_current_byte()`]: struct.Mcp794xx.html#method.read_sram_current_byte
//! [`enable_alarm()`]: struct.Mcp794xx.html#method.enable_alarm
//! [`set_alarm()`]: struct.Mcp794xx.html#method.set_alarm
//! [`has_alarm_matched()`]: struct.Mcp794xx.html#method.has_alarm_matched
//! [`clear_alarm_matched_flag()`]: struct.Mcp794xx.html#method.clear_alarm_matched_flag
//! [`enable_square_wave()`]: struct.Mcp794xx.html#method.enable_square_wave
//! [`set_square_wave_frequency()`]: struct.Mcp794xx.html#method.set_square_wave_frequency
//! [`read_protected_eeprom_byte()`]: struct.Mcp794xx.html#method.read_protected_eeprom_byte
//! [`read_protected_eeprom_data()`]: struct.Mcp794xx.html#method.read_protected_eeprom_data
//! [`read_eui48()`]: struct.Mcp794xx.html#method.read_eui48
//! [`read_eui64()`]: struct.Mcp794xx.html#method.read_eui64
//! [`read_eeprom_byte()`]: struct.Mcp794xx.html#method.read_eeprom_byte
//! [`read_eeprom_data()`]: struct.Mcp794xx.html#method.read_eeprom_data
//! [`set_eeprom_write_protection()`]: struct.Mcp794xx.html#method.set_eeprom_write_protection
//! [`read_eeprom_current_byte()`]: struct.Mcp794xx.html#method.read_eeprom_current_byte
//!
//! [Introductory blog post](https://blog.eldruin.com/mcp794xx-real-time-clock-rtc-driver-in-rust/)
//!
//! ## The devices
//!
//! This driver is compatible with the devices: MCP7940N, MCP7940M, MCP79400,
//! MCP79401, MCP79402, MCP79410, MCP79411 and MCP79412.
//!
//! The Real-Time Clock/Calendar (RTCC) tracks time using internal counters for
//! hours, minutes, seconds, days, months, years, and day of week. Alarms can
//! be configured on all counters up to and including months. For usage and
//! configuration, the devices support I2C communications up to 400 kHz.
//!
//! The open-drain, multi-functional output can be configured to assert on an
//! alarm match, to output a selectable frequency square wave, or as a general
//! purpose output.
//!
//! The devices are designed to operate using a 32.768 kHz tuning fork crystal
//! with external crystal load capacitors. On-chip digital trimming can be used
//! to adjust for frequency variance caused by crystal tolerance and temperature.
//!
//! SRAM and timekeeping circuitry are powered from the back-up supply when
//! main power is lost, allowing the device to maintain accurate time and the
//! SRAM contents. The times when the device switches over to the back-up supply
//! and when primary power returns are both logged by the power-fail time-stamp.
//!
//! Some of the devices feature 1 Kbit of internal non-volatile EEPROM with
//! software write-protectable regions. There is an additional 64 bits of
//! protected non-volatile memory which is only writable after an unlock
//! sequence, making it ideal for storing a unique ID or other
//! critical information.
//!
//! Some of the devices offer a pre-programmed with EUI-48 and EUI-64
//! addresses. Custom programming is also available.
//!
//! Datasheets:
//! - [MCP7940N](http://ww1.microchip.com/downloads/en/DeviceDoc/20005010F.pdf)
//! - [MCP7940M](http://ww1.microchip.com/downloads/en/DeviceDoc/20002292B.pdf)
//! - [MCP79400/MCP79401/MCP79402](http://ww1.microchip.com/downloads/en/DeviceDoc/MCP79400-MCP79401-MCP79402-Data-Sheet-20005009G.pdf)
//! - [MCP79410/MCP79411/MCP79412](http://ww1.microchip.com/downloads/en/DeviceDoc/20002266H.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//! The following examples use an instance of the device MCP7940N except when
//! using features specific to another IC.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Create a driver instance for the MCP7940N
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::Mcp794xx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let rtc = Mcp794xx::new_mcp7940n(dev);
//! // do something...
//!
//! // get the I2C device back
//! let dev = rtc.destroy();
//! ```
//!
//! ### Set the current date and time at once
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{Mcp794xx, NaiveDate, Hours, Rtcc};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! let datetime = NaiveDate::from_ymd(2018, 8, 20).and_hms(19, 59, 58);
//! rtc.set_datetime(&datetime).unwrap();
//! rtc.enable().unwrap();
//! ```
//!
//! ### Change the date and time at once
//!
//! Note that before changing the date/time the oscillators must be disabled
//! and you must be wait unter the oscillator reports not to be running anymore.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{Mcp794xx, NaiveDate, Hours, Rtcc};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! let datetime = NaiveDate::from_ymd(2018, 8, 20).and_hms(19, 59, 58);
//! rtc.set_datetime(&datetime).unwrap();
//! rtc.enable().unwrap();
//! // ...
//! // after running for a while disable before changing the time.
//!
//! rtc.disable().unwrap();
//! while (rtc.is_oscillator_running().unwrap()) {
//!     // some delay...
//! }
//! // now you can change the date/time
//! rtc.set_datetime(&datetime).unwrap();
//! rtc.enable().unwrap();
//! ```
//!
//! ### Get the current date and time at once
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{Mcp794xx, Rtcc, Datelike, Timelike};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//!
//! let dt = rtc.get_datetime().unwrap();
//! println!("{}-{}-{}, {} {}:{}:{}", dt.year(),
//!          dt.month(), dt.day(), dt.weekday().number_from_sunday(),
//!          dt.hour(), dt.minute(), dt.second());
//! // This will print something like: 2018-08-15, 4 19:59:58
//! ```
//!
//! ### Set / Get the year
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{ Mcp794xx, Hours, Rtcc };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! rtc.set_year(2019).unwrap();
//! let year = rtc.get_year().unwrap();
//! println!("Year: {}", year);
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Enable the square-wave output with a frequency of 4.096Hz
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{ Mcp794xx, SqWFreq };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! rtc.set_square_wave_frequency(SqWFreq::Hz4_096).unwrap();
//! rtc.enable_square_wave().unwrap();
//! ```
//!
//! ### Set the alarm 1 to each week on a week day at a specific time
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{Mcp794xx, Hours, Alarm, AlarmDateTime, AlarmMatching, AlarmOutputPinPolarity};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! let datetime = AlarmDateTime {
//!     month: 9,
//!     day: 17,
//!     weekday: 1,
//!     hour: Hours::H24(7),
//!     minute: 2,
//!     second: 15
//! };
//! rtc.set_alarm(
//!     Alarm::One,
//!     datetime,
//!     AlarmMatching::WeekdayMatches,
//!     AlarmOutputPinPolarity::High
//! ).unwrap();
//! rtc.enable_alarm(Alarm::One).unwrap();
//! ```
//!
//! ### Set output pin
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::{Mcp794xx, OutputPinLevel};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! rtc.set_output_pin(OutputPinLevel::High).unwrap();
//! ```
//!
//! ### Set trimming
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::Mcp794xx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! rtc.set_trimming(-50).unwrap();
//! rtc.enable_coarse_trim().unwrap();
//! ```
//!
//! ### Check power down date and time
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::Mcp794xx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! rtc.enable_backup_battery_power().unwrap();
//! loop {
//!     if rtc.has_power_failed().unwrap() {
//!         let datetime = rtc.get_power_down_datetime().unwrap();
//!         rtc.clear_power_failed().unwrap();
//!         //...
//!     }
//!     //...
//! }
//! ```
//!
//! ### Read/write SRAM
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::Mcp794xx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp7940n(dev);
//! let value = rtc.read_sram_byte(0x20).unwrap();
//! let data = [1, 2, 3, 4, 5];
//! rtc.write_sram_data(0x25, &data).unwrap();
//! ```
//!
//! ### Read/write EEPROM and protected EEPROM
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::Mcp794xx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp79410(dev);
//! let value = rtc.read_eeprom_byte(0x01).unwrap();
//! let data = [1, 2, 3, 4, 5];
//! rtc.write_eeprom_data(0x01, &data).unwrap();
//!
//! rtc.write_protected_eeprom_data(0xF0, &data).unwrap();
//! ```
//!
//! ### Read EUI-64
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mcp794xx::Mcp794xx;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Mcp794xx::new_mcp79402(dev);
//! let value = rtc.read_eui64().unwrap();
//! ```
//!

#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
use embedded_hal::blocking::i2c;
pub use rtcc::{Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike};

/// Feature markers
pub mod marker {
    use super::private;
    /// Supports backup battery power
    pub trait WithBatteryPower: private::Sealed {}
    /// Supports protected EEPROM
    pub trait WithProtectedEeprom: private::Sealed {}
    /// Supports EEPROM and protected EEPROM
    pub trait WithEeprom: private::Sealed {}
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
impl marker::WithEeprom for ic::Mcp79410 {}
impl marker::WithEeprom for ic::Mcp79411 {}
impl marker::WithEeprom for ic::Mcp79412 {}

mod types;
pub use crate::types::{
    Alarm, AlarmDateTime, AlarmMatching, AlarmOutputPinPolarity, EepromWriteProtection, Error,
    OutputPinLevel, PowerFailDateTime, SqWFreq,
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
use crate::interface::I2cInterface;
mod battery_power;
mod common;
mod eeprom;

macro_rules! create_destroy_i2c {
    ($ic:ident, $create:ident) => {
        impl<I2C, E> Mcp794xx<I2cInterface<I2C>, ic::$ic>
        where
            I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
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
            pub fn destroy(self) -> I2C {
                self.iface.i2c
            }
        }
    };
}
create_destroy_i2c!(Mcp7940n, new_mcp7940n);
create_destroy_i2c!(Mcp7940m, new_mcp7940m);
create_destroy_i2c!(Mcp79400, new_mcp79400);
create_destroy_i2c!(Mcp79401, new_mcp79401);
create_destroy_i2c!(Mcp79402, new_mcp79402);
create_destroy_i2c!(Mcp79410, new_mcp79410);
create_destroy_i2c!(Mcp79411, new_mcp79411);
create_destroy_i2c!(Mcp79412, new_mcp79412);

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
