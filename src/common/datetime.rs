//! Common date/time function

use super::super::{
    BitFlags, Datelike, Error, Hours, Mcp794xx, NaiveDate, NaiveDateTime, NaiveTime, Register,
    Rtcc, Timelike,
};
use super::conversion::{
    decimal_to_packed_bcd, hours_from_register, hours_to_register, packed_bcd_to_decimal,
};
use crate::interface;

impl<DI, E, IC> Rtcc for Mcp794xx<DI, IC>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
{
    type Error = Error<E>;

    fn get_seconds(&mut self) -> Result<u8, Self::Error> {
        let seconds = self.iface.read_register(Register::SECONDS)?;
        let seconds = packed_bcd_to_decimal(seconds & !BitFlags::ST);
        Ok(seconds)
    }

    fn get_minutes(&mut self) -> Result<u8, Self::Error> {
        let minutes = self.iface.read_register(Register::MINUTES)?;
        Ok(packed_bcd_to_decimal(minutes))
    }

    fn get_hours(&mut self) -> Result<Hours, Self::Error> {
        let data = self.iface.read_register(Register::HOURS)?;
        Ok(hours_from_register(data))
    }

    fn get_time(&mut self) -> Result<NaiveTime, Self::Error> {
        let mut data = [0; 3];
        self.iface.read_data(0, &mut data)?;
        let hour = hours_from_register(data[Register::HOURS as usize]);
        let minute = packed_bcd_to_decimal(data[Register::MINUTES as usize]);
        let second = packed_bcd_to_decimal(data[Register::SECONDS as usize] & !BitFlags::ST);
        let h24 = match hour {
            Hours::H24(h) => h,
            Hours::AM(h) => h,
            Hours::PM(h) => h + 12,
        };
        Ok(NaiveTime::from_hms(
            h24.into(),
            minute.into(),
            second.into(),
        ))
    }

    fn get_weekday(&mut self) -> Result<u8, Self::Error> {
        let data = self.iface.read_register(Register::WEEKDAY)?;
        let weekday = packed_bcd_to_decimal(data & 0b111);
        Ok(weekday)
    }

    fn get_day(&mut self) -> Result<u8, Self::Error> {
        let day = self.iface.read_register(Register::DAY)?;
        Ok(packed_bcd_to_decimal(day))
    }

    fn get_month(&mut self) -> Result<u8, Self::Error> {
        let value = self.iface.read_register(Register::MONTH)?;
        Ok(packed_bcd_to_decimal(value & !BitFlags::LEAPYEAR))
    }

    /// This device can compensate for leap years up to 2399
    /// but only the two last year digits are stored so we will return
    /// the year as in the range 2000-2099.
    fn get_year(&mut self) -> Result<u16, Self::Error> {
        let value = self.iface.read_register(Register::YEAR)?;
        Ok(2000 + u16::from(packed_bcd_to_decimal(value)))
    }

    fn get_date(&mut self) -> Result<NaiveDate, Self::Error> {
        let mut data = [0; 3];
        self.iface.read_data(Register::DAY, &mut data)?;
        let year = 2000 + u16::from(packed_bcd_to_decimal(data[2]));
        let month = packed_bcd_to_decimal(data[1] & !BitFlags::LEAPYEAR);
        let day = packed_bcd_to_decimal(data[0]);
        Ok(NaiveDate::from_ymd(year.into(), month.into(), day.into()))
    }

    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error> {
        Self::check_lt(seconds, 60)?;
        let seconds = decimal_to_packed_bcd(seconds);
        let value = if self.is_enabled {
            seconds | BitFlags::ST
        } else {
            seconds
        };
        self.iface.write_register(Register::SECONDS, value)
    }

    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error> {
        Self::check_lt(minutes, 60)?;
        let minutes = decimal_to_packed_bcd(minutes);
        self.iface.write_register(Register::MINUTES, minutes)
    }

    #[allow(clippy::match_like_matches_macro)] // not available on the MSRV
    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error> {
        let value = hours_to_register(hours)?;
        self.iface.write_register(Register::HOURS, value)?;
        self.is_running_in_24h_mode = match hours {
            Hours::H24(_) => true,
            _ => false,
        };
        Ok(())
    }

    fn set_time(&mut self, time: &NaiveTime) -> Result<(), Self::Error> {
        if time.minute() > 59 || time.second() > 59 {
            return Err(Error::InvalidInputData);
        }
        let second = decimal_to_packed_bcd(time.second() as u8);
        let second = if self.is_enabled {
            second | BitFlags::ST
        } else {
            second
        };
        let payload = [
            Register::SECONDS,
            second,
            decimal_to_packed_bcd(time.minute() as u8),
            hours_to_register(Hours::H24(time.hour() as u8))?,
        ];
        self.iface.write_data(&payload)?;
        self.is_running_in_24h_mode = true;
        Ok(())
    }

    /// Note that this clears the power failed flag.
    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error> {
        Self::check_lt(weekday, 8)?;
        Self::check_gt(weekday, 0)?;
        let value = decimal_to_packed_bcd(weekday);
        let value = if self.is_battery_power_enabled {
            value | BitFlags::VBATEN
        } else {
            value
        };
        self.iface.write_register(Register::WEEKDAY, value)
    }

    fn set_day(&mut self, day: u8) -> Result<(), Self::Error> {
        Self::check_lt(day, 32)?;
        Self::check_gt(day, 0)?;
        let data = decimal_to_packed_bcd(day);
        self.iface.write_register(Register::DAY, data)
    }

    fn set_month(&mut self, month: u8) -> Result<(), Self::Error> {
        Self::check_lt(month, 13)?;
        Self::check_gt(month, 0)?;
        let data = decimal_to_packed_bcd(month);
        self.iface.write_register(Register::MONTH, data)
    }

    /// This device can compensate for leap years up to 2399
    /// but only the two last year digits are stored so we only
    /// support the range 2000-2099.
    fn set_year(&mut self, year: u16) -> Result<(), Self::Error> {
        Self::check_lt(year, 2100)?;
        let value = decimal_to_packed_bcd((year - 2000) as u8);
        self.iface.write_register(Register::YEAR, value)
    }

    /// This device can compensate for leap years up to 2399
    /// but only the two last year digits are stored so we will return
    /// the year as in the range 2000-2099.
    fn get_datetime(&mut self) -> Result<NaiveDateTime, Self::Error> {
        let mut data = [0; 7];
        self.iface.read_data(0, &mut data)?;
        let year = 2000 + u16::from(packed_bcd_to_decimal(data[Register::YEAR as usize]));
        let month = packed_bcd_to_decimal(data[Register::MONTH as usize] & !BitFlags::LEAPYEAR);
        let day = packed_bcd_to_decimal(data[Register::DAY as usize]);
        let hour = hours_from_register(data[Register::HOURS as usize]);
        let minute = packed_bcd_to_decimal(data[Register::MINUTES as usize]);
        let second = packed_bcd_to_decimal(data[Register::SECONDS as usize] & !BitFlags::ST);
        let h24 = match hour {
            Hours::H24(h) => h,
            Hours::AM(h) => h,
            Hours::PM(h) => h + 12,
        };
        Ok(
            NaiveDate::from_ymd(year.into(), month.into(), day.into()).and_hms(
                h24.into(),
                minute.into(),
                second.into(),
            ),
        )
    }

    fn set_date(&mut self, date: &NaiveDate) -> Result<(), Self::Error> {
        if date.year() < 2000 || date.year() > 2099 {
            return Err(Error::InvalidInputData);
        }
        let payload = [
            Register::WEEKDAY,
            date.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(date.day() as u8),
            decimal_to_packed_bcd(date.month() as u8),
            decimal_to_packed_bcd((date.year() - 2000) as u8),
        ];
        self.iface.write_data(&payload)
    }

    /// Note that this clears the power failed flag.
    /// This device can compensate for leap years up to 2399
    /// but only the two last year digits are stored so we only
    /// support the range 2000-2099.
    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error> {
        if datetime.year() < 2000 || datetime.year() > 2099 {
            return Err(Error::InvalidInputData);
        }
        let second = decimal_to_packed_bcd(datetime.second() as u8);
        let second = if self.is_enabled {
            second | BitFlags::ST
        } else {
            second
        };
        let payload = [
            Register::SECONDS,
            second,
            decimal_to_packed_bcd(datetime.minute() as u8),
            hours_to_register(Hours::H24(datetime.hour() as u8))?,
            datetime.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(datetime.day() as u8),
            decimal_to_packed_bcd(datetime.month() as u8),
            decimal_to_packed_bcd((datetime.year() - 2000) as u8),
        ];
        self.iface.write_data(&payload)?;
        self.is_running_in_24h_mode = true;
        Ok(())
    }
}

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::ReadData<Error = Error<E>>,
{
    /// Returns whether the current year is a leap year.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_leap_year(&mut self) -> Result<bool, Error<E>> {
        let data = self.iface.read_register(Register::MONTH)?;
        Ok((data & BitFlags::LEAPYEAR) != 0)
    }
}
