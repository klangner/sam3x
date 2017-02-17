#![no_std]

pub mod hardware {
    pub mod pio;
    pub mod rtt;
}
pub mod drivers{
    pub mod led;
    mod driver;
}


/// Table vector consist of:
///   * Reset handler
///   * Exception table (len = 14)
///   * Interrupt table (len = 30)
pub struct VectorTable {
    pub reset_handler   : fn()->!,
    pub exceptions      : [u32; 14]
}



