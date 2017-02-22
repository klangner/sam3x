// Peripheral identifiers for Atmel SAM3X.
// See data sheet, chapter 9.


const SUPC      : u32 = 0 ; // Supply Controller
const RSTC      : u32 = 1 ; // Reset Controller
const RTC       : u32 = 2 ; // Real-time Clock
const RTT       : u32 = 3 ; // Real-time Timer
const WDT       : u32 = 4 ; // Watchdog Timer
const PMC       : u32 = 5 ; // Power Management Controller
const EEFC0     : u32 = 6 ; // Enhanced Embedded Flash Controller 0
const EEFC1     : u32 = 7 ; // Enhanced Embedded Flash Controller 1
const UART      : u32 = 8 ; // Universal Asynchronous Receiver Transmitter
const SMC_SDRAMC: u32 = 9 ; // Static Memory Controller / Synchronous
// Dynamic RAM Controller
const SDRAMC    : u32 = 10; // Synchronous Dynamic RAM Controller
const PIOA      : u32 = 11; // Parallel I/O Controller A
const PIOB      : u32 = 12; // Parallel I/O Controller B
const PIOC      : u32 = 13; // Parallel I/O Controller C
const PIOD      : u32 = 14; // Parallel I/O Controller D
const PIOE      : u32 = 15; // Parallel I/O Controller E
const PIOF      : u32 = 16; // Parallel I/O Controller F
const USART0    : u32 = 17; // Universal Synchronous Asynchronous Receiver
// Transmitter 0
const USART1    : u32 = 18; // Universal Synchronous Asynchronous Receiver
// Transmitter 1
const USART2    : u32 = 19; // Universal Synchronous Asynchronous Receiver
// Transmitter 2
const USART3    : u32 = 20; // Universal Synchronous Asynchronous Receiver
// Transmitter 3
const HSMCI     : u32 = 21; // High Speed Multimedia Card Interface
const TWI0      : u32 = 22; // Two-Wire Interface 0
const TWI1      : u32 = 23; // Two-Wire Interface 1
const SPI0      : u32 = 24; // Serial Peripheral Interface 0
const SPI1      : u32 = 25; // Serial Peripheral Interface 1
const SSC       : u32 = 26; // Synchronous Serial Controller
const TC0       : u32 = 27; // Timer Counter Channel 0
const TC1       : u32 = 28; // Timer Counter Channel 1
const TC2       : u32 = 29; // Timer Counter Channel 2
const TC3       : u32 = 30; // Timer Counter Channel 3
const TC4       : u32 = 31; // Timer Counter Channel 4
const TC5       : u32 = 32; // Timer Counter Channel 5
const TC6       : u32 = 33; // Timer Counter Channel 6
const TC7       : u32 = 34; // Timer Counter Channel 7
const TC8       : u32 = 35; // Timer Counter Channel 8
const PWM       : u32 = 36; // Pulse Width Modulation Controller
const ADC       : u32 = 37; // ADC Controller
const DACC      : u32 = 38; // DAC Controller
const DMAC      : u32 = 39; // DMA Controller
const UOTGHS    : u32 = 40; // USB OTG High Speed
const TRNG      : u32 = 41; // True Random Number Generator
const EMAC      : u32 = 42; // Ethernet MAC
const CAN0      : u32 = 43; // CAN Controller 0
const CAN1      : u32 = 44; // CAN Controller 1


#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Peripheral {
    Supc      = SUPC,
    Rstc      = RSTC,
    Rtc       = RTC,
    Rtt       = RTT,
    Wdt       = WDT,
    Pmc       = PMC,
    Eefc0     = EEFC0,
    Eefc1     = EEFC1,
    Uart      = UART,
    SmcSdramc = SMC_SDRAMC,
    Sdramc    = SDRAMC,
    PioA      = PIOA,
    PioB      = PIOB,
    PioC      = PIOC,
    PioD      = PIOD,
    PioE      = PIOE,
    PioF      = PIOF,
    Usart0    = USART0,
    Usart1    = USART1,
    Usart2    = USART2,
    Usart3    = USART3,
    Hsmci     = HSMCI,
    Twi0      = TWI0,
    Twi1      = TWI1,
    Spi0      = SPI0,
    Spi1      = SPI1,
    Ssc       = SSC,
    Tc0       = TC0,
    Tc1       = TC1,
    Tc2       = TC2,
    Tc3       = TC3,
    Tc4       = TC4,
    Tc5       = TC5,
    Tc6       = TC6,
    Tc7       = TC7,
    Tc8       = TC8,
    Pwm       = PWM,
    Adc       = ADC,
    Dacc      = DACC,
    Dmac      = DMAC,
    UtogHs    = UOTGHS,
    Trng      = TRNG,
    Emac      = EMAC,
    Can0      = CAN0,
    Can1      = CAN1,
}

impl Peripheral {
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn index(&self) -> usize {
        self.id() as usize / 32
    }

    pub fn mask(&self) -> u32 {
        0x1 << (self.id() % 32)
    }
}
