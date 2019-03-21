use core::cmp::max;

pub enum Register {
    SysCtrl1,
    SysCtrl2,
}

impl Register {
    pub fn value(&self) -> u8 {
        match *self {
            Register::SysCtrl1 => 0x04,
            Register::SysCtrl2 => 0x05,
        }
    }
}

pub enum SysCtrl2Flag {
    DsgOn,
    ChgOn,
}

impl SysCtrl2Flag {
    pub fn mask(&self) -> u8 {
        match *self {
            SysCtrl2Flag::ChgOn => 0,
            SysCtrl2Flag::DsgOn => 1,
        }
    }
}
