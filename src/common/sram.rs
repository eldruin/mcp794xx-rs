use super::super::interface;
use super::super::{Error, Mcp794xx};

impl<DI, E> Mcp794xx<DI>
where
    DI: interface::WriteData<Error = Error<E>> + interface::ReadData<Error = Error<E>>,
{
    /// Read a single byte from an address.
    ///
    /// Valid addresses are from 0x20 to 0x5F. Otherwise an
    /// Error::InvalidInputData will be returned.
    pub fn read_sram_byte(&mut self, address: u8) -> Result<u8, Error<E>> {
        if address < 0x20 || address > 0x5F {
            return Err(Error::InvalidInputData);
        }
        self.iface.read_register(address)
    }

    /// Write a single byte to an address.
    ///
    /// Valid addresses are from 0x20 to 0x5F. Otherwise an
    /// Error::InvalidInputData will be returned.
    pub fn write_sram_byte(&mut self, address: u8, data: u8) -> Result<(), Error<E>> {
        if address < 0x20 || address > 0x5F {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(address, data)
    }

    /// Read SRAM starting in an address as many bytes as necessary to fill
    /// the data array provided.
    pub fn read_sram_data(&mut self, address: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        if address < 0x20
            || address > 0x5F
            || data.len() > 64
            || (data.len() as u8 + address) > 0x60
        {
            return Err(Error::InvalidInputData);
        }
        self.iface.read_data(address, data)
    }
}
