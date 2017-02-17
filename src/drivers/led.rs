/// Led driver. Using this driver led can be:
///   * Switched on/off
///   * TBD - Set to blink with a given freq

use drivers::driver::{Driver};
use hardware::pio::{BinaryPin, Mode};
pub use hardware::pio::Port;

pub struct Led {
    pin:    BinaryPin
}

impl Led {
    /// Connect driver to the led on a pin
    pub fn connect(port: Port, pin: u32) -> Option<Led> {
        BinaryPin::init(port, pin, Mode::Output).map(|p| {
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
