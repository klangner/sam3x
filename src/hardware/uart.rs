#![allow(dead_code)]
// Universal Asynchronous Receiver/Transmitter code for Atmel SAM3X.
// See data sheet, chapter 34.


use core::fmt;
use volatile_register::{WO, RO, RW};
use hardware::peripherals::{Peripheral};
use hardware::pdc::{Pdc};
use hardware::pio::{Pin};
use hardware::pmc;


pub struct Tx {
    pin: Pin
}


/// UART user interface. See data sheet, chapter 34.6.
#[repr(C)]
struct Uart {
    control            : WO<u32>,
    mode               : RW<u32>,
    interrupt_enable   : WO<u32>,
    interrupt_disable  : WO<u32>,
    interrupt_mask     : RO<u32>,
    status             : RO<u32>,
    receive_holding    : RO<u32>,
    transmit_holding   : WO<u32>,
    baud_rate_generator: RW<u32>,

    _reserved: [RO<u32>; 55],

    pdc: Pdc,
}


// Available bound rates
pub const BR_1200: u32 = 1200;
pub const BR_9600: u32 = 9600;
pub const BR_19200: u32 = 19200;

// Control register bits. See data sheet, chapter 34.6.1.
const RSTRX : u32 = 0x1 << 2; // Reset Receiver
const RSTTX : u32 = 0x1 << 3; // Reset Transmitter
const RXEN  : u32 = 0x1 << 4; // Receiver Enable
const RXDIS : u32 = 0x1 << 5; // Receiver Disable
const TXEN  : u32 = 0x1 << 6; // Transmitter Enable
const TXDIS : u32 = 0x1 << 7; // Transmitter Disable
const RSTSTA: u32 = 0x1 << 8; // Reset Status Bits

// Parity configuration, to be written into the mode register. See data sheet,
// chapter 34.6.2.
const PARITY_EVEN : u32 = 0x0 << 9;
const PARITY_ODD  : u32 = 0x1 << 9;
const PARITY_SPACE: u32 = 0x2 << 9; // parity forced to 0
const PARITY_MARK : u32 = 0x3 << 9; // parity forced to 1
const PARITY_NONE: u32 = 0x4 << 9;

// UART modes, to be written into the mode register. See data sheet, chapter
// 34.6.2.
const MODE_NORMAL         : u32 = 0x0 << 14;
const MODE_AUTOMATIC_ECHO : u32 = 0x1 << 14;
const MODE_LOCAL_LOOPBACK : u32 = 0x2 << 14;
const MODE_REMOTE_LOOPBACK: u32 = 0x3 << 14;

// Status register bits. See data cheet, chapter 34.6.6.
const RXRDY  : u32 = 0x1 << 0 ; // Receiver Ready
const TXRDY  : u32 = 0x1 << 1 ; // Transmitter Ready
const ENDRX  : u32 = 0x1 << 3 ; // End of Receiver Transfer
const ENDTX  : u32 = 0x1 << 4 ; // End of Transmitter Transfer
const OVRE   : u32 = 0x1 << 5 ; // Overrun Error
const FRAME  : u32 = 0x1 << 6 ; // Framing Error
const PARE   : u32 = 0x1 << 7 ; // Parity Error
const TXEMPTY: u32 = 0x1 << 9 ; // Transmitter Empty
const TXBUFE : u32 = 0x1 << 11; // Transmission Buffer Empty
const RXBUFF : u32 = 0x1 << 12; // Receive Buffer Full


const UART: *mut Uart = 0x400E0800 as *mut Uart;


impl Tx {
    pub fn init(bound_rate: u32) -> Tx {
        let pin = Pin::init(Peripheral::PioA, 9).expect("Can't connect to he Tx pin");
        pin.disable();
        pin.enable_pull_up();
        pin.select_peripheral_a();
        unsafe {
            pmc::enable_peripheral_clock_0(Peripheral::Uart);
            (*UART).control.write(RSTRX | RSTTX | RXDIS | TXDIS);
            // Set bound rate
            let clock_divisor = pmc::main_clock_frequency_hz() / (bound_rate * 16);
            (*UART).baud_rate_generator.write(clock_divisor);
            // Put UART into normal mode and use a parity bit.
            (*UART).mode.write(MODE_NORMAL | PARITY_NONE);
            (*UART).control.write(RXEN | TXEN);
        }
        Tx { pin: pin }
    }

    pub fn write_byte(&self, b: u8) {
        unsafe {
            // Wait until transmitter is ready. See data sheet, sections
            // 34.5.3.3 and 34.6.6.
            while (*UART).status.read() & TXRDY == 0 {}

            // Send byte. See data sheet, sections 34.5.3.3 and 34.6.8.
            (*UART).transmit_holding.write(b as u32);
        }
    }

    pub fn write(&self, s: &str){
        for &b in s.as_bytes() {
            self.write_byte(b);
        }
        unsafe {
            while (*UART).status.read() & TXEMPTY == 0 {}
        }
    }

}


impl fmt::Write for Tx {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            self.write_byte(b);
        }
        unsafe {
            while (*UART).status.read() & TXEMPTY == 0 {}
        }
        Ok(())
    }
}