use super::conversion::{convert_hours_to_format, decimal_to_packed_bcd, hours_to_register};
use {
    interface, Alarm, AlarmDateTime, AlarmMatching, AlarmOutputPinPolarity, BitFlags, Error,
    Mcp794xx, Register,
};

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
{
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
}
