#![no_std]

pub mod pio;
pub mod rtt;


/// Table vector consist of:
///   * Reset handler
///   * Exception table (len = 14)
///   * Interrupt table (len = 30)
pub struct VectorTable {
    pub reset_handler   : fn()->!,
    pub exceptions      : [u32; 14]
}



