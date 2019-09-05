//! Communication interface
use super::{private, Error, DEVICE_ADDRESS};
use hal::blocking;

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// Write data
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>,
{
    type Error = Error<E>;

    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
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
}

/// Read current data
pub trait ReadCurrent: private::Sealed {
    /// Error type
    type Error;
    /// Read
    fn read(&mut self) -> Result<u8, Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>,
{
    type Error = Error<E>;

    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    fn read_data(&mut self, address: u8, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write_read(DEVICE_ADDRESS, &[address], &mut payload[..])
            .map_err(Error::Comm)
    }
}

impl<I2C, E> ReadCurrent for I2cInterface<I2C>
where
    I2C: blocking::i2c::Read<Error = E>,
{
    type Error = Error<E>;

    fn read(&mut self) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .read(DEVICE_ADDRESS, &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }
}
