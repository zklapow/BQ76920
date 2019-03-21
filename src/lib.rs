#![no_std]

pub mod registers;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use crate::registers::{SysCtrl2Flag, Register};

pub struct BQ76920<I2C> {
    addr: u8,
    i2c: I2C
}

impl<I2C, E> BQ76920<I2C> where I2C: WriteRead<Error=E> + Write<Error=E> {
    pub fn new(addr: u8, i2c: I2C) -> Result<Self, E> {
        // TODO: Configure device

        Ok(BQ76920{addr, i2c})
    }

    pub fn all_on(&mut self, on: bool) -> Result<(), E> {
        let register_val = match on {
            true => 1 << SysCtrl2Flag::ChgOn.mask() | 1 << SysCtrl2Flag::DsgOn.mask(),
            false => 0,
        };

        let cmd: [u8; 2] = [Register::SysCtrl2.value(), register_val];
        self.i2c.write(self.addr, &cmd)
    }

    pub fn dsg(&mut self, on: bool) -> Result<(), E> {
        let flag = on as u8;
        let cmd: [u8; 2] = [Register::SysCtrl2.value(), flag << SysCtrl2Flag::DsgOn.mask()];

        self.i2c.write(self.addr, &cmd)
    }

    pub fn chg(&mut self, on: bool) -> Result<(), E> {
        let flag = on as u8;
        let cmd: [u8; 2] = [Register::SysCtrl2.value(), flag << SysCtrl2Flag::ChgOn.mask()];

        self.i2c.write(self.addr, &cmd)
    }
}
