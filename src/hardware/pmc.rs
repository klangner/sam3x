// Power Management Controller code for Atmel SAM3X.
// See data sheet, chapter 28.

use volatile_register::{WO, RO};
use hardware::peripherals::{Peripheral};


/// Power Management Controller user interface. See data sheet, chapter 28.15.
#[repr(C)]
pub struct Pmc {
    system_clock_enable : u32,
    system_clock_disable: u32,
    system_clock_status : u32,

    _reserved_1: u32,

    peripheral_clock_enable_0 : WO<u32>,
    peripheral_clock_disable_0: WO<u32>,
    peripheral_clock_status_0 : RO<u32>,

    utmi_clock          : RO<u32>,
    main_oscillator     : RO<u32>,
    main_clock_frequency: RO<u32>,
    plla                : RO<u32>,

    _reserved_2: u32,

    master_clock: u32,

    _reserved_3: u32,

    usb_clock: u32,

    _reserved_4: u32,

    programmable_clock_0: u32,
    programmable_clock_1: u32,
    programmable_clock_2: u32,

    _reserved_5: [u32; 5],

    interrupt_enable : u32,
    interrupt_disable: u32,

    status: u32,

    interrupt_mask: u32,

    fast_startup_mode    : u32,
    fast_startup_polarity: u32,

    fault_output_clear: u32,

    _reserved_6: [u32; 26],

    write_protect_mode  : u32,
    write_protect_status: u32,

    _reserved_7: [u32; 5],

    peripheral_clock_enable_1 : WO<u32>,
    peripheral_clock_disable_1: WO<u32>,
    peripheral_clock_status_1 : WO<u32>,

    peripheral_control: u32,
}


pub const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;

// Constants for Clock Generator Main Clock Frequency Register. See data sheet,
// section 28.15.9.
pub const MAINFRDY  : u32 = 0x00010000;
pub const MAINF_MASK: u32 = 0x0000ffff;


pub const PMC: *mut Pmc = 0x400E0600 as *mut Pmc;

/// This function have to be called before we can use Pio is input
pub fn enable_peripheral_clock_0(peripheral: Peripheral) {
    unsafe {
        (*PMC).peripheral_clock_enable_0.write(peripheral.mask());
    }
}


pub fn main_clock_frequency_hz() -> u32 {
    let main_clock_frequency_within_16_slow_clock_cycles = unsafe {
        while (*PMC).main_clock_frequency.read() & MAINFRDY == 0 {}
        (*PMC).main_clock_frequency.read() & MAINF_MASK
    };

    (main_clock_frequency_within_16_slow_clock_cycles * SLOW_CLOCK_FREQUENCY_HZ) / 16
}