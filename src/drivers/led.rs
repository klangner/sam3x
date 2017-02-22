/// Led driver. Using this driver led can be:
///   * Switched on/off
///   * TBD - Set to blink with a given freq

use drivers::driver::{Driver};
use hal::pio::{Pin};
use hal::peripherals::{Peripheral};

pub struct Led {
    pin: Pin
}

impl Led {
    /// Connect driver to the led on a pin
    pub fn connect(peripheral: Peripheral, line: u32) -> Option<Led> {
        Pin::init(peripheral, line).map(|p| {
            p.enable_output();
            Led { pin: p }
        })
    }

    /// Turn the led on
    pub fn on(&self) {
        self.pin.on();
    }

    /// Turn the led off
    pub fn off(&self) {
        self.pin.off();
    }
}

impl Driver for Led {
    fn tick(&self) { }
}
