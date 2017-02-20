/// Based on:
/// https://github.com/hannobraun/embedded
///
/// Parallel I/O code for Atmel SAM3X.
/// See Datasheet, chapter 31.


use volatile_register::{RO, WO};
use hardware::peripherals::Peripheral;


/// Pin is specific line in specific controller
/// This structure is used to access functions on given pin.
pub struct Pin {
    controller: *mut Controller,
    mask: u32
}

/// Structure of PIO controller
#[allow(dead_code)]
struct Controller {
    /// Activate Line.
    pio_enable : WO<u32>,
    pio_disable: WO<u32>,
    pio_status : RO<u32>,

    _reserved_1: RO<u32>,

    /// Set this line as output
    output_enable : WO<u32>,
    output_disable: WO<u32>,
    output_status : RO<u32>,

    _reserved_2: RO<u32>,

    glitch_input_filter_enable : u32,
    glitch_input_filter_disable: u32,
    glitch_input_filter_status : u32,

    _reserved_3: u32,

    /// Set line value to 1
    set_output_data   : WO<u32>,
    /// Set line value to 0
    clear_output_data : WO<u32>,
    /// Get line value
    output_data_status: RO<u32>,
    pin_data_status   : RO<u32>,

    interrupt_enable : u32,
    interrupt_disable: u32,
    interrupt_mask   : u32,
    interrupt_status : u32,

    multi_driver_enable : u32,
    multi_driver_disable: u32,
    multi_driver_status : u32,

    _reserved_4: u32,

    pull_up_disable   : WO<u32>,
    pull_up_enable    : WO<u32>,
    pull_up_status    : RO<u32>,

    _reserved_5: u32,

    peripheral_ab_select: u32,

    _reserved_6: [u32; 3],

    system_clock_glitch_input_filter_select                 : u32,
    debouncing_input_filter_select                          : u32,
    glitch_or_debouncing_input_filter_clock_selection_status: u32,
    slow_clock_divider_debouncing                           : u32,

    _reserved_7: [u32; 4],

    output_write_enable : u32,
    output_write_disable: u32,
    output_write_status : u32,

    _reserved_8: u32,

    additional_interrupt_modes_enable : u32,
    additional_interrupt_modes_disable: u32,
    additional_interrupt_modes_mask   : u32,

    _reserved_9: u32,

    edge_select      : u32,
    level_select     : u32,
    edge_level_status: u32,

    _reserved_a: u32,

    falling_edge_low_level_select: u32,
    rising_edge_high_level_select: u32,
    fall_rise_low_high_status    : u32,

    _reserved_b: u32,

    lock_status         : u32,
    write_protect_mode  : u32,
    write_protect_status: u32,
}


/// Addresses of the PIO controllers. See chapters 31.7 and 31.7.1.
/// SAM3X 30 lines
const PIO_A: *mut Controller = 0x400E0E00 as *mut Controller;
/// SAM3X 32 lines
const PIO_B: *mut Controller = 0x400E1000 as *mut Controller;
/// SAM3X 31 lines
const PIO_C: *mut Controller = 0x400E1200 as *mut Controller;
/// SAM3X 10 lines
const PIO_D: *mut Controller = 0x400E1400 as *mut Controller;


impl Pin {

    /// Since different controllers have different line numbers, it can happen then
    /// the provided parameters will not match with existing architecture.
    /// That's why we return Option
    pub fn init(peripheral: Peripheral, line: u32) -> Option<Pin> {
        match peripheral {
            Peripheral::PioA => if line < 30 { Some(PIO_A) } else { None },
            Peripheral::PioB => if line < 32 { Some(PIO_B) } else { None },
            Peripheral::PioC => if line < 31 { Some(PIO_C) } else { None },
            Peripheral::PioD => if line < 10 { Some(PIO_D) } else { None },
            _ => None
        }
        .map(|c| {
            let mask = 0x1 << line;
            Pin { controller: c, mask: mask }
        })
    }

    pub fn enable_output(&self) {
        unsafe {
            (*self.controller).pio_enable.write((*self.controller).pio_status.read() | self.mask);
            (*self.controller).output_enable.write((*self.controller).output_status.read() | self.mask);
        }
    }

    pub fn enable_input(&self) {
        unsafe { (*self.controller).pio_enable.write((*self.controller).pio_status.read() | self.mask); }
        self.enable_pull_up();
    }

    pub fn enable_pull_up(&self) {
        unsafe {
            (*self.controller).pull_up_enable.write((*self.controller).pull_up_status.read() | self.mask);
        }
    }

    pub fn on(&self) {
        unsafe {
            (*self.controller).set_output_data.write((*self.controller).output_data_status.read() | self.mask);
        }
    }

    pub fn off(&self) {
        unsafe {
            (*self.controller).clear_output_data.write((*self.controller).output_data_status.read() | self.mask);
        }
    }

    pub fn is_on(&self) -> bool {
        unsafe {
            (*self.controller).pin_data_status.read() & self.mask > 0
        }
    }

    pub fn is_off(&self) -> bool {
        unsafe {
            (*self.controller).pin_data_status.read() & self.mask == 0
        }
    }
}

