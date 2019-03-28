#![no_std]

use core::ptr::read;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::registers::{Register, RegisterWriter};

#[macro_use]
pub mod registers;
pub mod sysctrl1;
pub mod sysctrl2;

use sysctrl1::SysCtrl1;
use sysctrl2::SysCtrl2;

pub struct BQ76920<I2C> {
    addr: u8,
    i2c: I2C,
}

impl<I2C, E> BQ76920<I2C> where I2C: WriteRead<Error=E> + Write<Error=E> {
    pub fn new(addr: u8, i2c: I2C) -> Result<Self, E> {
        // TODO: Configure device

        Ok(BQ76920 { addr, i2c })
    }

    pub fn dsg_on(&mut self) -> Result<SysCtrl2, E> {
        self.modify(|w: &mut SysCtrl2| {
            w.dsgon().set_bit();
        })
    }

    pub fn chg_on(&mut self) -> Result<SysCtrl2, E> {
        self.modify(|w: &mut SysCtrl2| {
            w.chgon().set_bit();
        })
    }

    pub fn adc_on(&mut self) -> Result<SysCtrl1, E> {
        self.modify(|w: &mut SysCtrl1| {
            w.adcen().set_bit();
        })
    }

    pub fn write<F, W>(&mut self, f: F) -> Result<W, E> where F: FnOnce(&mut W), W: RegisterWriter {
        let mut writer = W::from_u8(0);

        f(&mut writer);

        let cmd: [u8; 2] = [W::register().addr(), writer.value()];
        self.i2c.write(self.addr, &cmd)?;

        Ok(writer)
    }

    pub fn modify<F, W>(&mut self, f: F) -> Result<W, E> where F: FnOnce(&mut W), W: RegisterWriter {
        let r = self.read(W::register())?;

        let mut writer = W::from_u8(r);
        f(&mut writer);

        let cmd: [u8; 2] = [W::register().addr(), writer.value()];
        self.i2c.write(self.addr, &cmd)?;

        Ok(writer)
    }

    pub fn read(&mut self, reg: Register) -> Result<u8, E> {
        let cmd: [u8; 1] = [reg.addr()];
        let mut buf: [u8; 1] = [0; 1];

        self.i2c.write_read(self.addr, &cmd, &mut buf)?;

        Ok(buf[0])
    }
}
