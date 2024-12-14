use super::*;
use embedded_hal_02::blocking::i2c::{Operation, Read, Transactional, Write, WriteRead};

impl<I2C: Instance> Write for BlockingI2c<I2C> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write(addr, bytes)
    }
}

impl<I2C: Instance> Read for BlockingI2c<I2C> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.read(addr, buffer)
    }
}

impl<I2C: Instance> WriteRead for BlockingI2c<I2C> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write_read(addr, bytes, buffer)
    }
}

impl<I2C: Instance> Transactional for BlockingI2c<I2C> {
    type Error = Error;

    fn exec(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        self.transaction_slice_hal_02(address, operations)
    }
}
