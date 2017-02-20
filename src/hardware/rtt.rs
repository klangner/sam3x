#![allow(dead_code)]
/// Based on:
/// https://github.com/hannobraun/embedded
///
/// Real-time Timer code for Atmel SAM3X.
/// See data sheet, chapter 13.


use hardware::pmc;


// Real-time Timer user interface. See data sheet, chapter 13.5.
#[repr(C)]
struct Timer {
    mode  : u32,
    alarm : u32,
    value : u32,
    status: u32,
}


// Mode register flags. See data sheet, section 13.5.1.
const ALMIEN   : u32 = 0x1 << 16; // Alarm Interrupt Enable
const RTTINCIEN: u32 = 0x1 << 17; // RTT Increment Interrupt Enable
const RTTRST   : u32 = 0x1 << 18; // RTT Restart

// Status register flags. See data sheet, section 13.5.4.
const ALMS  : u32 = 0x1 << 0; // Real-time Alarm Status
const RTTINC: u32 = 0x1 << 1; // Real-time Timer Increment

const RTT: *mut Timer = 0x400E1A30 as *mut Timer;


/// Initialization is needed to set timer resolution
/// The resolution will be set to 1ms.
pub fn init_timer() {
    unsafe {
        let mode: u32 = pmc::SLOW_CLOCK_FREQUENCY_HZ / 1000;
        (*RTT).mode = mode;
    }
}

/// Wait burning cycles for a given amount of milliseconds
pub fn wait_ms(sleep_ms: u32) {
    unsafe {
        let sleep_until = (*RTT).value + sleep_ms;
        // The value of (RTT+sleep_ms) can overflow if this is the case then first
        // wait for value to overflow too
        if sleep_until < (*RTT).value {

        }
        // No we can burn cycles.
        while (*RTT).value < sleep_until {}
    }
}
