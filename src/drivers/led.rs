/// Led driver. Using this driver led can be:
///   * Switched on/off
///   * TBD - Set to blink with a given freq

use drivers::driver::{Driver};
use hardware::pio::{BinaryPin, Mode};
use hardware::peripherals::{Peripheral};

pub struct Led {
    pin:    BinaryPin
}

impl Led {
    /// Connect driver to the led on a pin
    pub fn connect(peripheral: Peripheral, line: u32) -> Option<Led> {
        BinaryPin::init(peripheral, line, Mode::Output).map(|p| {
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
