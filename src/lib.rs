#![no_std]

extern crate volatile_register;

pub mod hal {
    pub mod pdc;
    pub mod peripherals;
    pub mod pio;
    pub mod pmc;
    pub mod rtt;
    pub mod uart;
    pub mod wdt;

}
pub mod drivers{
    mod driver;
    pub mod led;
    pub mod button;
}


/// Table vector consist of:
///   * Reset handler
///   * Exception table (len = 14)
///   * Interrupt table (len = 30)
pub struct VectorTable {
    pub reset_handler   : fn()->!,
    pub exceptions      : [u32; 14]
}



