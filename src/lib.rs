#![no_std]

pub mod pio;


/// Table vector consist of:
///   * Reset handler
///   * Exception table (len = 14)
///   * Interrupt table (len = 30)
pub struct VectorTable {
    pub reset_handler               : fn()->!,
    pub other_interrupt_vectors     : [u32; 44],
}

// Addresses of several registers used to control the real-time timer.
pub const TIMER_MODE_REGISTER : *mut   u32 = 0x400E1A30 as *mut   u32;
pub const TIMER_VALUE_REGISTER: *const u32 = 0x400E1A38 as *const u32;


