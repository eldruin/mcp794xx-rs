use {
    interface, Alarm, AlarmDateTime, AlarmMatching, AlarmOutputPinPolarity, BitFlags, Config,
    Error, Mcp794xx, OutputPinLevel, Register, SqWFreq,
};
pub mod conversion;
use self::conversion::{convert_hours_to_format, decimal_to_packed_bcd, hours_to_register};
pub mod datetime;
pub mod sram;

impl Config {
    pub(crate) fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    pub(crate) fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl<DI, E, IC> Mcp794xx<DI, IC>
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
        let payload = [
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
        self.iface.write_data(&payload)?;
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
