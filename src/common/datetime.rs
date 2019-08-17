//! Common date/time function

use super::super::{BitFlags, DateTime, Error, Hours, Mcp794xx, Register, Rtcc};
use super::{decimal_to_packed_bcd, hours_from_register, hours_to_register, packed_bcd_to_decimal};
use interface;

impl<DI, E> Rtcc for Mcp794xx<DI>
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

    fn get_year(&mut self) -> Result<u16, Self::Error> {
        Err(Error::InvalidInputData)
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

    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error> {
        let value = hours_to_register(hours)?;
        self.iface.write_register(Register::HOURS, value)
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

    fn set_year(&mut self, year: u16) -> Result<(), Self::Error> {
        Err(Error::InvalidInputData)
    }

    fn get_datetime(&mut self) -> Result<DateTime, Self::Error> {
        Err(Error::InvalidInputData)
    }

    fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Self::Error> {
        Err(Error::InvalidInputData)
    }
}
