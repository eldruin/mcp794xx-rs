//! Communication interface
use crate::{private, Error, DEVICE_ADDRESS, EEPROM_ADDRESS};
use embedded_hal::blocking;

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

impl<I2C, E> I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>,
{
    fn read_byte(&mut self, device_address: u8, address: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(device_address, &[address], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    fn read_data(
        &mut self,
        device_address: u8,
        address: u8,
        payload: &mut [u8],
    ) -> Result<(), Error<E>> {
        self.i2c
            .write_read(device_address, &[address], &mut payload[..])
            .map_err(Error::Comm)
    }
}

impl<I2C, E> I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>,
{
    fn write_data(&mut self, device_address: u8, payload: &[u8]) -> Result<(), Error<E>> {
        self.i2c
            .write(device_address, &payload)
            .map_err(Error::Comm)
    }
}

impl<I2C, E> I2cInterface<I2C>
where
    I2C: blocking::i2c::Read<Error = E>,
{
    fn read(&mut self, device_address: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .read(device_address, &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }
}

/// Write data
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
    /// Write byte to EEPROM
    fn write_eeprom_byte(&mut self, address: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data to EEPROM. The first element corresponds to the starting address.
    fn write_eeprom_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>,
{
    type Error = Error<E>;

    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        self.write_data(DEVICE_ADDRESS, &[register, data])
    }

    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.write_data(DEVICE_ADDRESS, &payload)
    }

    fn write_eeprom_byte(&mut self, address: u8, data: u8) -> Result<(), Self::Error> {
        self.write_data(EEPROM_ADDRESS, &[address, data])
    }

    fn write_eeprom_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.write_data(EEPROM_ADDRESS, &payload)
    }
}

/// Read data
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data.
    fn read_data(&mut self, address: u8, payload: &mut [u8]) -> Result<(), Self::Error>;
    /// Read byte from EEPROM
    fn read_eeprom_byte(&mut self, address: u8) -> Result<u8, Self::Error>;
    /// Read some data from EEPROM.
    fn read_eeprom_data(&mut self, address: u8, payload: &mut [u8]) -> Result<(), Self::Error>;
}

/// Read current data
pub trait ReadCurrent: private::Sealed {
    /// Error type
    type Error;
    /// Read current address
    fn read(&mut self) -> Result<u8, Self::Error>;
    /// Read current address from EEPROM
    fn read_eeprom(&mut self) -> Result<u8, Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>,
{
    type Error = Error<E>;

    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        self.read_byte(DEVICE_ADDRESS, register)
    }

    fn read_data(&mut self, address: u8, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.read_data(DEVICE_ADDRESS, address, &mut payload[..])
    }

    fn read_eeprom_byte(&mut self, register: u8) -> Result<u8, Self::Error> {
        self.read_byte(EEPROM_ADDRESS, register)
    }

    fn read_eeprom_data(&mut self, address: u8, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.read_data(EEPROM_ADDRESS, address, &mut payload[..])
    }
}

impl<I2C, E> ReadCurrent for I2cInterface<I2C>
where
    I2C: blocking::i2c::Read<Error = E>,
{
    type Error = Error<E>;

    fn read(&mut self) -> Result<u8, Self::Error> {
        self.read(DEVICE_ADDRESS)
    }

    fn read_eeprom(&mut self) -> Result<u8, Self::Error> {
        self.read(EEPROM_ADDRESS)
    }
}
