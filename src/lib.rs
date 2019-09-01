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

/// Square-wave output frequency
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SqWFreq {
    /// 1 Hz (default)
    Hz1,
    /// 4.096 Hz
    Hz4_096,
    /// 8.192 Hz
    Hz8_192,
    /// 32.768 Hz
    Hz32_768,
}

/// General purpose output pin logic level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputPinLevel {
    /// High
    High,
    /// Low
    Low,
}

/// Alarm interrupt output pin polarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlarmOutputPinPolarity {
    /// High logic level when alarm asserted
    High,
    /// Low logic level when alarm asserted
    Low,
}

/// Alarm trigger rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlarmMatching {
    /// Alarm triggers when seconds match.
    SecondsMatch,
    /// Alarm triggers when minutes match.
    MinutesMatch,
    /// Alarm triggers when hours match.
    HoursMatch,
    /// Alarm triggers when weekday matches.
    WeekdayMatches,
    /// Alarm triggers when day (date/day of month) matches.
    DayMatches,
    /// Alarm triggers when seconds, minutes, hours, weekday, day and month match.
    AllMatch,
}

/// Alarm selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alarm {
    /// Alarm 0
    Zero,
    /// Alarm 1
    One,
}

/// Alarm date/time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlarmDateTime {
    /// Month [1-12]
    pub month: u8,
    /// Day [1-31]
    pub day: u8,
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour in 24h/12h format (format matches RTC)
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8,
}

/// Power fail date/time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerFailDateTime {
    /// Month [1-12]
    pub month: u8,
    /// Day [1-31]
    pub day: u8,
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour in 24h/12h format (format matches RTC)
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
}

/// MCP794xx RTCC driver
#[derive(Debug)]
pub struct Mcp794xx<DI> {
    iface: DI,
    is_enabled: bool,
    is_battery_power_enabled: bool,
    is_running_in_24h_mode: bool,
    control: Config,
    alarm_output_pin_polarity: AlarmOutputPinPolarity,
}

#[derive(Debug, Clone, Copy)]
struct Config {
    bits: u8,
}

const DEVICE_ADDRESS: u8 = 0b110_1111;

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

pub mod interface;
use interface::I2cInterface;
mod common;
use common::conversion::{
    convert_hours_to_format, decimal_to_packed_bcd, hours_from_register, hours_to_register,
    packed_bcd_to_decimal,
};

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
            is_running_in_24h_mode: false,
            control: Config {
                bits: BitFlags::OUT,
            },
            alarm_output_pin_polarity: AlarmOutputPinPolarity::Low,
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

    /// Clears the power failed status flag and power-fail time-stamp registers.
    pub fn clear_power_failed(&mut self) -> Result<(), Error<E>> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        let data = data & !BitFlags::PWRFAIL;
        self.iface.write_register(Register::WEEKDAY, data)
    }

    /// Returns date/time when the power failed went down (under Vtrip).
    ///
    /// Note that the registers need to be cleared by calling
    /// [`clear_power_failed()`](#method.clear_power_failed)
    pub fn get_power_down_datetime(&mut self) -> Result<PowerFailDateTime, Error<E>> {
        self.get_power_fail(Register::PWRDNMIN)
    }

    fn get_power_fail(&mut self, starting_register: u8) -> Result<PowerFailDateTime, Error<E>> {
        let mut data = [0; 5];
        data[0] = starting_register;
        self.iface.read_data(&mut data)?;
        Ok(PowerFailDateTime {
            minute: packed_bcd_to_decimal(data[1]),
            hour: hours_from_register(data[2]),
            day: packed_bcd_to_decimal(data[3]),
            weekday: data[4] >> 5,
            month: packed_bcd_to_decimal(data[4] & 0b0001_1111),
        })
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

    /// Enable square-wave output.
    ///
    /// Note that this is not available when running on backup battery power.
    pub fn enable_square_wave(&mut self) -> Result<(), Error<E>> {
        self.write_control(self.control.with_high(BitFlags::SQWEN))
    }

    /// Disable square-wave output.
    pub fn disable_square_wave(&mut self) -> Result<(), Error<E>> {
        self.write_control(self.control.with_low(BitFlags::SQWEN))
    }

    /// Set square-wave output frequency.
    ///
    /// Note that this setting will be ignored if the square-wave output is not
    /// enabled or digital trimming is enabled.
    pub fn set_square_wave_frequency(&mut self, frequency: SqWFreq) -> Result<(), Error<E>> {
        let bits = match frequency {
            SqWFreq::Hz1 => 0,
            SqWFreq::Hz4_096 => 1,
            SqWFreq::Hz8_192 => 2,
            SqWFreq::Hz32_768 => 3,
        };
        let control = Config {
            bits: (self.control.bits & 0b1111_1100) | bits,
        };
        self.write_control(control)
    }

    /// Set output pin logic level.
    ///
    /// Note that this setting will be ignored if the square-wave output or any
    /// of the alarm interrupt outputs are enabled.
    pub fn set_output_pin(&mut self, level: OutputPinLevel) -> Result<(), Error<E>> {
        let control = match level {
            OutputPinLevel::High => self.control.with_high(BitFlags::OUT),
            OutputPinLevel::Low => self.control.with_low(BitFlags::OUT),
        };
        self.write_control(control)
    }

    /// Enable coarse trim mode.
    pub fn enable_coarse_trim(&mut self) -> Result<(), Error<E>> {
        self.write_control(self.control.with_high(BitFlags::CRSTRIM))
    }

    /// Disable coarse trim mode.
    pub fn disable_coarse_trim(&mut self) -> Result<(), Error<E>> {
        self.write_control(self.control.with_low(BitFlags::CRSTRIM))
    }

    /// Set digital trimming value.
    ///
    /// The sign determines whether the value will be added or substracted
    /// to or from the 32.768kHz clock signal.
    /// The argument value is always multiplied by two, so a value of 127
    /// will add 254 clock cycles and a value of -50 will substract 100 cycles.
    /// Depending on the digital trimming setting, this will be applied
    /// either once per minute or 128 times per second.
    /// Set to 0 or -128 to disable digital trimming.
    pub fn set_trimming(&mut self, value: i8) -> Result<(), Error<E>> {
        if value < 0 && value != -128 {
            let rest = !(value - 1) as u8;
            self.iface
                .write_register(Register::OSCTRIM, 0b1000_0000 | rest)
        } else {
            self.iface.write_register(Register::OSCTRIM, value as u8)
        }
    }

    /// Enable alarm
    pub fn enable_alarm(&mut self, alarm: Alarm) -> Result<(), Error<E>> {
        match alarm {
            Alarm::Zero => self.write_control(self.control.with_high(BitFlags::ALM0EN)),
            Alarm::One => self.write_control(self.control.with_high(BitFlags::ALM1EN)),
        }
    }

    /// Disable alarm
    pub fn disable_alarm(&mut self, alarm: Alarm) -> Result<(), Error<E>> {
        match alarm {
            Alarm::Zero => self.write_control(self.control.with_low(BitFlags::ALM0EN)),
            Alarm::One => self.write_control(self.control.with_low(BitFlags::ALM1EN)),
        }
    }

    /// Set alarm for date/time with a trigger rate and an output pin polarity.
    ///
    /// Note that this clears the alarm has matched flag and the alarm needs to be
    /// enabled separately.
    /// Note that the output pin polarity will be set to the same value for both alarms.
    pub fn set_alarm(
        &mut self,
        alarm: Alarm,
        when: AlarmDateTime,
        matching: AlarmMatching,
        polarity: AlarmOutputPinPolarity,
    ) -> Result<(), Error<E>> {
        if when.month < 1
            || when.month > 12
            || when.day < 1
            || when.day > 31
            || when.weekday < 1
            || when.weekday > 7
            || when.minute > 59
            || when.second > 59
        {
            return Err(Error::InvalidInputData);
        }
        let hours = convert_hours_to_format(self.is_running_in_24h_mode, when.hour)?;
        let mut weekday = decimal_to_packed_bcd(when.weekday);
        if polarity != self.alarm_output_pin_polarity && alarm == Alarm::One {
            let data = self.iface.read_register(Register::ALM0WKDAY)?;
            let data = match polarity {
                AlarmOutputPinPolarity::Low => data & !BitFlags::ALMPOL,
                AlarmOutputPinPolarity::High => data | BitFlags::ALMPOL,
            };
            self.iface.write_register(Register::ALM0WKDAY, data)?;
            self.alarm_output_pin_polarity = polarity;
        }

        if polarity == AlarmOutputPinPolarity::High {
            weekday |= BitFlags::ALMPOL;
        }
        let mask = match matching {
            AlarmMatching::SecondsMatch => 0,
            AlarmMatching::MinutesMatch => 1 << 4,
            AlarmMatching::HoursMatch => 2 << 4,
            AlarmMatching::WeekdayMatches => 3 << 4,
            AlarmMatching::DayMatches => 4 << 4,
            AlarmMatching::AllMatch => 7 << 4,
        };
        weekday |= mask;
        let mut payload = [
            if alarm == Alarm::Zero {
                Register::ALM0SEC
            } else {
                Register::ALM1SEC
            },
            decimal_to_packed_bcd(when.second),
            decimal_to_packed_bcd(when.minute),
            hours_to_register(hours)?,
            weekday,
            decimal_to_packed_bcd(when.day),
            decimal_to_packed_bcd(when.month),
        ];
        self.iface.write_data(&mut payload)?;
        self.alarm_output_pin_polarity = polarity;
        Ok(())
    }

    /// Returns whether the alarm has matched.
    ///
    /// Once this is true, it will stay as such until cleared. e.g. with
    /// [`clear_alarm_matched_flag()`](#method.clear_alarm_matched_flag)
    pub fn has_alarm_matched(&mut self, alarm: Alarm) -> Result<bool, Error<E>> {
        let reg = match alarm {
            Alarm::Zero => Register::ALM0WKDAY,
            Alarm::One => Register::ALM1WKDAY,
        };
        let data = self.iface.read_register(reg)?;
        Ok((data & BitFlags::ALMIF) != 0)
    }

    /// Clears the alarm matched flag.
    pub fn clear_alarm_matched_flag(&mut self, alarm: Alarm) -> Result<(), Error<E>> {
        let reg = match alarm {
            Alarm::Zero => Register::ALM0WKDAY,
            Alarm::One => Register::ALM1WKDAY,
        };
        let data = self.iface.read_register(reg)?;
        self.iface.write_register(reg, data & !BitFlags::ALMIF)
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
    impl<E> Sealed for dyn interface::ReadData<Error = E> {}
    impl<E> Sealed for dyn interface::WriteData<Error = E> {}
}
