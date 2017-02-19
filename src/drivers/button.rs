/// Led driver. Using this driver led can be:
///   * Switched on/off
///   * TBD - Set to blink with a given freq

use drivers::driver::{Driver};
use hardware::pio::{BinaryPin, Mode};
use hardware::peripherals::{Peripheral};


pub struct Button {
    pin:    BinaryPin
}

impl Button {
    /// Connect driver to the led on a pin
    pub fn connect(peripheral: Peripheral, pin: u32) -> Option<Button> {
        BinaryPin::init(peripheral, pin, Mode::Input).map(|p| {
            Button { pin: p }
        })
    }

    pub fn is_pressed(&self) -> bool {
        self.pin.is_off()
    }
}

impl Driver for Button {
    fn tick(&self) { }
}
