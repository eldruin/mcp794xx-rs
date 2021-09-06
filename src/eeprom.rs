//! EEPROM methods
use crate::{interface, marker, EepromWriteProtection, Error, Mcp794xx};
const EEUNLOCK: u8 = 0b0000_1001;
const EEPROM_STATUS: u8 = 0xFF;

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
    IC: marker::WithProtectedEeprom,
{
    /// Read a single byte from an address in the protected EEPROM.
    ///
    /// Valid addresses are in the range `[0xF0-0xF7]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses.
    pub fn read_protected_eeprom_byte(&mut self, address: u8) -> Result<u8, Error<E>> {
        if !is_protected_eeprom_address(address) {
            return Err(Error::InvalidInputData);
        }
        self.iface.read_eeprom_byte(address)
    }

    /// Read protected EEPROM starting in an address as many bytes as
    /// necessary to fill the data array provided.
    ///
    /// Valid addresses are in the range `[0xF0-0xF7]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses or
    /// if the reading would overflow the size of the protected EEPROM.
    pub fn read_protected_eeprom_data(
        &mut self,
        address: u8,
        data: &mut [u8],
    ) -> Result<(), Error<E>> {
        if !is_protected_eeprom_address(address)
            || data.len() > 8
            || (data.len() as u8 + address) > 0xF8
        {
            return Err(Error::InvalidInputData);
        }
        self.iface.read_eeprom_data(address, data)
    }

    /// Unlock protected EEPROM and write a single byte to an address.
    ///
    /// Valid addresses are in the range `[0xF0-0xF7]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses.
    pub fn write_protected_eeprom_byte(&mut self, address: u8, data: u8) -> Result<(), Error<E>> {
        if !is_protected_eeprom_address(address) {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(EEUNLOCK, 0x55)?;
        self.iface.write_register(EEUNLOCK, 0xAA)?;
        self.iface.write_eeprom_byte(address, data)
    }

    /// Write data array starting in an address in the protected EEPROM.
    ///
    /// Valid addresses are in the range `[0xF0-0xF7]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses or
    /// if the writing would overflow the size of the protected EEPROM.
    pub fn write_protected_eeprom_data(
        &mut self,
        address: u8,
        data: &[u8],
    ) -> Result<(), Error<E>> {
        if !is_protected_eeprom_address(address)
            || data.len() > 8
            || (data.len() as u8 + address) > 0xF8
        {
            return Err(Error::InvalidInputData);
        }
        let mut payload = [0; 9]; // max size
        payload[0] = address;
        payload[1..=data.len()].copy_from_slice(data);
        self.iface.write_register(EEUNLOCK, 0x55)?;
        self.iface.write_register(EEUNLOCK, 0xAA)?;
        self.iface.write_eeprom_data(&payload[..=data.len()])
    }
}

#[allow(clippy::manual_range_contains)]
fn is_protected_eeprom_address(address: u8) -> bool {
    address >= 0xF0 && address <= 0xF7
}

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
    IC: marker::WithEeprom,
{
    /// Set the EEPROM block write protection
    pub fn set_eeprom_write_protection(
        &mut self,
        protection: EepromWriteProtection,
    ) -> Result<(), Error<E>> {
        let value = match protection {
            EepromWriteProtection::None => 0,
            EepromWriteProtection::UpperQuarter => 0b0000_0100,
            EepromWriteProtection::UpperHalf => 0b0000_1000,
            EepromWriteProtection::All => 0b0000_1100,
        };
        self.iface.write_eeprom_byte(EEPROM_STATUS, value)
    }

    /// Read a single byte from an address in EEPROM.
    ///
    /// Valid addresses are in the range `[0x00-0x7F]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses.
    pub fn read_eeprom_byte(&mut self, address: u8) -> Result<u8, Error<E>> {
        if is_eeprom_address(address) {
            self.iface.read_eeprom_byte(address)
        } else {
            Err(Error::InvalidInputData)
        }
    }

    /// Read EEPROM starting in an address as many bytes as necessary to fill
    /// the data array provided.
    ///
    /// Valid addresses are in the range `[0x00-0x7F]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses or
    /// if the reading would overflow the size of the EEPROM.
    pub fn read_eeprom_data(&mut self, address: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        if is_eeprom_address(address) && data.len() <= 128 && (address + data.len() as u8) < 0x80 {
            self.iface.read_eeprom_data(address, data)
        } else {
            Err(Error::InvalidInputData)
        }
    }

    /// Write a single byte to an address in EEPROM.
    ///
    /// Valid addresses are in the range `[0x00-0x7F]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses.
    pub fn write_eeprom_byte(&mut self, address: u8, data: u8) -> Result<(), Error<E>> {
        if is_eeprom_address(address) {
            self.iface.write_eeprom_byte(address, data)
        } else {
            Err(Error::InvalidInputData)
        }
    }

    /// Write data array starting in an address in EEPROM.
    ///
    /// Valid addresses are in the range `[0x00-0x7F]`.
    /// `Error::InvalidInputData` will be returned for invalid addresses or
    /// if the writing would overflow the size of the EEPROM.
    pub fn write_eeprom_data(&mut self, address: u8, data: &[u8]) -> Result<(), Error<E>> {
        if is_eeprom_address(address) && data.len() <= 128 && (address + data.len() as u8) < 0x80 {
            let mut payload = [0; 128]; // max size
            payload[0] = address;
            payload[1..=data.len()].copy_from_slice(data);
            self.iface.write_eeprom_data(&payload[..=data.len()])
        } else {
            Err(Error::InvalidInputData)
        }
    }
}

fn is_eeprom_address(address: u8) -> bool {
    address < 0x80
}

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::ReadCurrent<Error = Error<E>>,
{
    /// Read a single byte from the current address in EEPROM.
    ///
    /// The current address corresponds to the last accessed address
    /// (including addresses accessed in SRAM/RTCC) incremented by 1.
    pub fn read_eeprom_current_byte(&mut self) -> Result<u8, Error<E>> {
        self.iface.read_eeprom()
    }
}

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::ReadData<Error = Error<E>>,
    IC: marker::WithEui48,
{
    /// Read pre-programmed EUI-48 node address from EEPROM.
    pub fn read_eui48(&mut self) -> Result<[u8; 6], Error<E>> {
        let mut data = [0; 6];
        self.iface.read_eeprom_data(0xF2, &mut data).and(Ok(data))
    }
}

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::ReadData<Error = Error<E>>,
    IC: marker::WithEui64,
{
    /// Read pre-programmed EUI-64 node address from EEPROM.
    pub fn read_eui64(&mut self) -> Result<[u8; 8], Error<E>> {
        let mut data = [0; 8];
        self.iface.read_eeprom_data(0xF0, &mut data).and(Ok(data))
    }
}
