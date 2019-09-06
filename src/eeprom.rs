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
}
