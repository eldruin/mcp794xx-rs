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
        Err(Error::InvalidInputData)
    }

    fn get_day(&mut self) -> Result<u8, Self::Error> {
        Err(Error::InvalidInputData)
    }

    fn get_month(&mut self) -> Result<u8, Self::Error> {
        Err(Error::InvalidInputData)
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

    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error> {
        Err(Error::InvalidInputData)
    }

    fn set_day(&mut self, day: u8) -> Result<(), Self::Error> {
        Err(Error::InvalidInputData)
    }

    fn set_month(&mut self, month: u8) -> Result<(), Self::Error> {
        Err(Error::InvalidInputData)
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
