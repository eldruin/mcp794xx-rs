//! Backup battery power methods

use crate::common::conversion::{hours_from_register, packed_bcd_to_decimal};
use crate::{interface, marker, BitFlags, Error, Mcp794xx, PowerFailDateTime, Register};

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
    IC: marker::WithBatteryPower,
{
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

    /// Returns date/time when the power went back up (above Vtrip).
    ///
    /// Note that the registers need to be cleared by calling
    /// [`clear_power_failed()`](#method.clear_power_failed)
    pub fn get_power_up_datetime(&mut self) -> Result<PowerFailDateTime, Error<E>> {
        self.get_power_fail(Register::PWRUPMIN)
    }

    fn get_power_fail(&mut self, starting_register: u8) -> Result<PowerFailDateTime, Error<E>> {
        let mut data = [0; 4];
        self.iface.read_data(starting_register, &mut data)?;
        Ok(PowerFailDateTime {
            minute: packed_bcd_to_decimal(data[0]),
            hour: hours_from_register(data[1]),
            day: packed_bcd_to_decimal(data[2]),
            weekday: data[3] >> 5,
            month: packed_bcd_to_decimal(data[3] & 0b0001_1111),
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
}
