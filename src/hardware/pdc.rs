// Peripheral DMA Controller code for Atmel SAM3X.
// See data sheet, chapter 26.


use volatile_register::{RO};


/// PDC user interface. See data sheet, chapter 26.5.
#[repr(C)]
pub struct Pdc {
    receive_pointer      : RO<u32>,
    receive_counter      : RO<u32>,
    transmit_pointer     : RO<u32>,
    transmit_counter     : RO<u32>,
    receive_next_pointer : RO<u32>,
    receive_next_counter : RO<u32>,
    transmit_next_pointer: RO<u32>,
    transmit_next_counter: RO<u32>,
    transfer_control     : RO<u32>,
    transfer_status      : RO<u32>,
}