/// Led driver. Using this driver led can be:
///   * Switched on/off
///   * TBD - Set to blink with a given freq

use drivers::driver::{Driver};
use hal::pio::{Pin};
use hal::peripherals::{Peripheral};


pub struct Button {
    pin: Pin
}

impl Button {
    /// Connect driver to the led on a pin
    pub fn connect(peripheral: Peripheral, pin: u32) -> Option<Button> {
        Pin::init(peripheral, pin).map(|p| {
            p.enable_input();
            Button { pin: p }
        })
    }

    /// Since SAM3X has pull up resistors then button is defined as pressed when
    /// the input is in low state.
    pub fn is_pressed(&self) -> bool {
        self.pin.is_off()
    }
}

impl Driver for Button {
    fn tick(&self) { }
}
