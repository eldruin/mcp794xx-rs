//! EEPROM methods
use {interface, marker, Error, Mcp794xx};

impl<DI, E, IC> Mcp794xx<DI, IC>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
    IC: marker::WithProtectedEeprom,
{
    /// Read a single byte from an address in EEPROM.
    ///
    /// Valid addresses are from 0xF0 to 0xF7. Otherwise an
    /// Error::InvalidInputData will be returned.
    pub fn read_eeprom_byte(&mut self, address: u8) -> Result<u8, Error<E>> {
        if address < 0xF0 || address > 0xF7 {
            return Err(Error::InvalidInputData);
        }
        self.iface.read_eeprom_byte(address)
    }

    /// Read EEPROM starting in an address as many bytes as necessary to fill
    /// the data array provided.
    pub fn read_eeprom_data(&mut self, address: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        if address < 0xF0 || address > 0xF7 || data.len() > 8 || (data.len() as u8 + address) > 0xF8
        {
            return Err(Error::InvalidInputData);
        }
        self.iface.read_eeprom_data(address, data)
    }
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
