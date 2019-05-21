#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

use adc::{AdcGain1, AdcGain2, AdcOffset};
use bat::{BatHi, BatLo};
use protect::UvTrip;
use sysctrl1::SysCtrl1;
use sysctrl2::SysCtrl2;

use crate::registers::RegisterWriter;

#[macro_use]
pub mod registers;
pub mod adc;
pub mod bat;
pub mod protect;
pub mod sysctrl1;
pub mod sysctrl2;
pub mod cccfg;

use cccfg::CcCfg;

pub struct BQ76920<I2C> {
    addr: u8,
    i2c: I2C,
    adccal: Option<AdcCalibration>,
}

impl<I2C, E> BQ76920<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(addr: u8, i2c: I2C) -> Result<Self, E> {
        // TODO: Configure device

        // See Table 7-14, CC_CFG should be set to 0x19 at start
        let mut bq = BQ76920 {
            addr,
            i2c,
            adccal: None,
        };

        bq.write(|w: &mut CcCfg| {
            w.val().update(0x19 as u8);
        })?;

        Ok(bq)
    }

    pub fn set_uvtrip(&mut self, millivolts: i32) -> Result<i32, E> {
        let adccal = self.adccal();

        let uv_trip_full =
            ((millivolts as f32 - (adccal.offset as f32)) * 1000f32) / (adccal.gain as f32);

        let uv_trip = (uv_trip_full as u32 >> 4) & 0b0011111111;

        self.modify(|w: &mut UvTrip| {
            w.update(uv_trip as u8);
        })
        .map(|_| millivolts)
    }

    pub fn adccal(&mut self) -> AdcCalibration {
        self.adccal
            .or_else(|| self.read_adc().ok())
            .expect("Could not load ADC calibration")
    }

    fn read_adc(&mut self) -> Result<AdcCalibration, E> {
        let offset: AdcOffset = self.read()?;
        let gain1: AdcGain1 = self.read()?;
        let gain2: AdcGain2 = self.read()?;

        // From data sheet Table 7-22
        let gain1masked = (gain1.value() & 0b00001100) << 1;
        let gain2masked = (gain2.value() & 0b11100000) >> 5;
        let gain = (gain1masked | gain2masked) as u16 + 365u16;

        Ok(AdcCalibration {
            gain,
            offset: offset.value() as i8,
        })
    }

    pub fn bat_voltage(&mut self) -> Result<i32, E> {
        let (bh, bl): (BatHi, BatLo) = self.read16()?;

        let bh16: u16 = (bh.value() as u16) << 8;
        let adcval = bh16 | (bl.value() as u16);

        // Datasheet section 7.3.1.1.6
        // Offset is in mv, ADC is in uV
        let uv = 4 * self.adccal().gain as i32 * adcval as i32
            + (3 * 1000 * self.adccal().offset as i32);

        Ok(uv / 1000)
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

    pub fn write<F, W>(&mut self, f: F) -> Result<W, E>
    where
        F: FnOnce(&mut W),
        W: RegisterWriter,
    {
        let mut writer = W::from_u8(0);

        f(&mut writer);

        let cmd: [u8; 2] = [W::register().addr(), writer.value()];
        self.i2c.write(self.addr, &cmd)?;

        Ok(writer)
    }

    pub fn modify<F, W>(&mut self, f: F) -> Result<W, E>
    where
        F: FnOnce(&mut W),
        W: RegisterWriter,
    {
        let mut r = self.read()?;
        f(&mut r);

        let cmd: [u8; 2] = [W::register().addr(), r.value()];
        self.i2c.write(self.addr, &cmd)?;

        Ok(r)
    }

    pub fn read<W>(&mut self) -> Result<W, E>
    where
        W: RegisterWriter,
    {
        let cmd: [u8; 1] = [W::register().addr()];
        let mut buf: [u8; 1] = [0; 1];

        self.i2c.write_read(self.addr, &cmd, &mut buf)?;

        Ok(W::from_u8(buf[0]))
    }

    fn read16<WH, WL>(&mut self) -> Result<(WH, WL), E>
    where
        WH: RegisterWriter,
        WL: RegisterWriter,
    {
        //TODO: we should be able to do this with address auto inc

        let cmdh: [u8; 1] = [WH::register().addr()];
        let cmdl: [u8; 1] = [WL::register().addr()];
        let mut buf: [u8; 1] = [0; 1];

        self.i2c.write_read(self.addr, &cmdh, &mut buf)?;
        let h = buf[0].clone();

        self.i2c.write_read(self.addr, &cmdl, &mut buf)?;
        let l = buf[0].clone();

        Ok((WH::from_u8(h), WL::from_u8(l)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AdcCalibration {
    gain: u16,
    offset: i8,
}
