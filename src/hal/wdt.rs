/// Watchdog Timer interface for Atmel SAM3X.
/// See Datasheet, chapter 15.


use volatile_register::{WO, RO};

extern {
    fn WDT_Restart(wdt: *mut u32);
}


/// User interface for the Watchdog Timer. See Datasheet, chapter 15.5.
#[repr(C)]
pub struct Wdt {
    control: WO<u32>,
    mode   : RO<u32>,
    status : RO<u32>,
}


pub const WDT: *mut Wdt = 0x400E1A50 as *mut Wdt;


/// Watch dog should be restarted around every 15 seconds.
/// Otherwise watchdog will restart the main program.
pub fn restart_watchdog() {
    unsafe {
        (*WDT).control.write(0xA5000001);
//        unsafe { WDT_Restart(0x400E1A50 as *mut u32) };
    }
}