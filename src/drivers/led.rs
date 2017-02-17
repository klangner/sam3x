/// Led driver. Using this driver led can be:
///   * Switched on/off
///   * TBD - Set to blink with a given freq

pub struct Led {
    pin:    BinaryPin
}

impl Led {
    /// Connect driver to the led on a given binary port
    pub fn connect_binary(pin: BinaryPin) -> Led {
        Led { pin: pin }
    }

    /// Turn the led on
    pub fn on(&self) {
        pin.on();
    }

    /// Turn the led off
    pub fn off(&self) {
        pin.off();
    }
}

impl Driver for Led {
    fn tick(&self) { }
}
