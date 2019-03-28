use crate::registers::{RegisterWriter, Register};

struct UvTrip {
    bits: u8
}

impl RegisterWriter for UvTrip {
    fn from_u8(val: u8) -> Self {
        UvTrip { bits: val }
    }

    fn register() -> Register {
        Register::UV_TRIP
    }

    fn update(&mut self, val: u8) {
        self.bits = val;
    }

    fn value(&self) -> u8 {
        self.bits
    }
}

