#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    SYS_CTRL1 = 0x04,
    SYS_CTRL2 = 0x05,
    OV_TRIP = 0x09,
    UV_TRIP = 0x0A,
    ADCOFFSET = 0x51,
    ADCGAIN1 = 0x50,
    ADCGAIN2 = 0x59,
    BAT_HI = 0x2A,
    BAT_LO = 0x2B,
    CC_CFG = 0x0B,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

pub trait RegisterWriter {
    fn from_u8(val: u8) -> Self;
    fn register() -> Register;
    fn update(&mut self, val: u8);
    fn value(&self) -> u8;
}

pub trait RegisterBits {
    fn mask() -> u8;
    //fn from_u8(val: u8) -> Self;
    fn to_u8(&self) -> u8;
}

macro_rules! register_val {
    ($name: ident, $lsb: expr, $len: expr) => {

        pub struct $name {
            val: u8,
        }

        impl From<u8> for $name {
            fn from(val: u8) -> Self {
                let masked_val = val & $name::mask();
                let shifted = masked_val >> $lsb;

                $name { val: shifted }
            }
        }

        impl $crate::registers::RegisterBits for $name {
            fn mask() -> u8 {
                let mut mask = 0;
                for i in 0..$len {
                    mask = mask & (1 << $lsb + i);
                }

                mask
            }

            fn to_u8(&self) -> u8 {
                let shifted = self.val << $lsb;

                shifted & $name::mask()
            }
        }

        impl $name {
            pub fn value(&self) -> u8 {
                self.val
            }

            pub fn update(&mut self, val: u8) {
                self.val = val;
            }
        }
    };
}

macro_rules! register_bit {
    ($name: ident, $mask: expr) => {
        pub struct $name {
            bit: bool,
        }

        impl From<u8> for $name {
            fn from(val: u8) -> Self {
                let bit = val & $name::mask() > 0;

                $name { bit }
            }
        }

        impl $crate::registers::RegisterBits for $name {
            fn mask() -> u8 {
                1 << $mask
            }

            fn to_u8(&self) -> u8 {
                if self.bit {
                    return $name::mask();
                }

                0
            }
        }

        impl $name {
            pub fn bit_is_set(&self) -> bool {
                self.bit
            }

            pub fn set_bit(&mut self) {
                self.bit = true;
            }

            pub fn clear_bit(&mut self) {
                self.bit = false;
            }
        }
    };
}

macro_rules! register {
    ( $name:ident, $register:ident, { $( $bit_name:ident : $bit:ident ),* }) => {
        use $crate::registers::Register::$register;
        use $crate::registers::RegisterBits as _RegisterBits;

        pub struct $name {
            $(
            $bit_name: $bit,
            )*
        }

        impl $name {
            $(
            pub fn $bit_name(&mut self) -> &mut $bit {
                &mut self.$bit_name
            }
            )*
        }

        impl $crate::registers::RegisterWriter for $name {
            fn from_u8(val: u8) -> Self {
                $name {
                    $(
                    $bit_name: $bit::from(val),
                    )*
                }
            }

            fn register() -> $crate::registers::Register {
                $register
            }

            fn update(&mut self, val: u8) {
                $(
                self.$bit_name = $bit::from(val);
                )*
            }

            fn value(&self) -> u8 {
                // The final | 0 here is to make macro expansion easier (or 0 is a noop)
                $(self.$bit_name.to_u8() | )* 0u8
            }
        }
    };
}

macro_rules! raw_register {
    ( $name:ident, $register:ident) => {
        use $crate::registers::Register::$register;

        pub struct $name {
            bits: u8,
        }

        impl $crate::registers::RegisterWriter for $name {
            fn from_u8(val: u8) -> Self {
                $name { bits: val }
            }

            fn register() -> $crate::registers::Register {
                $register
            }

            fn update(&mut self, val: u8) {
                self.bits = val;
            }

            fn value(&self) -> u8 {
                self.bits
            }
        }
    };
}
