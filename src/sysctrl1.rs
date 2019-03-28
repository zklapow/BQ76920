//use crate::registers::{RegisterWriter, RegisterBits, Register};

register_bit!(AdcEn, 4);
register_bit!(TempSel, 3);

register!(SysCtrl1, SYS_CTRL1, {
    adcen : AdcEn,
    tempsel: TempSel
});