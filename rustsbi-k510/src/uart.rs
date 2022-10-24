use k510_pac::uart;
use core::fmt;
use spin::{once::Once, Mutex};

pub(crate) static SERIAL: Once<Uart0> = Once::new();

pub(crate) struct Uart0(Mutex<&'static uart::RegisterBlock>);

unsafe impl Send for Uart0 {}

unsafe impl Sync for Uart0 {}

impl Uart0 {
    pub unsafe fn new(base: usize) -> Self {
        Self(Mutex::new(&*(base as *mut uart::RegisterBlock)))
    }

    fn reset() {
        let uart0_reset_address: *mut u32 = 0x97002058 as *mut u32;
        unsafe {
            uart0_reset_address.write_volatile(0x80000000);
            uart0_reset_address.write_volatile(0x1);
            loop {
                if uart0_reset_address.read_volatile() & 0x80000000 == 0 {
                    break
                }
            }
        }
    }

    pub fn debug_init() {
        let data_width: u32 = 8;
        let stop_bit: u32 = 0;
        let parity: u32 = 0;
        let baud_rate: u32 = 115200;
        let __uart_brate_const: u32 = 16;
        Uart0::reset();

        // TODO - uint32_t freq = sysctl_clk_get_leaf_freq(SYSCTL_CLK_UART0_SCLK + channel);
        let freq: u32 = 0;

        let divisor: u32 = freq / baud_rate;
        let dlh: u8 = (divisor >> 12) as u8;
        let mut dll: u8 = ((divisor - ((dlh as u32) << 12)) / __uart_brate_const) as u8;
        let mut dlf: u8 = (divisor - ((dlh as u32) << 12) - dll as u32 * __uart_brate_const) as u8;

        if dlh == 0 && dll == 0 {
            dll = 1;
            dlf = 0;
        }

        //    /* Set UART registers */
        //
        //     uart[channel]->LCR |= 1u << 7;
        //     uart[channel]->DLH = dlh;
        //     uart[channel]->DLL = dll;
        //     uart[channel]->DLF = dlf;
        //     uart[channel]->LCR = 0;
        //     uart[channel]->LCR = (data_width - 5) | (stopbit_val << 2) | (parity_val << 3);
        //     uart[channel]->LCR &= ~(1u << 7);
        //     uart[channel]->IER |= 0x80; /* THRE */
        //     uart[channel]->FCR = UART_RECEIVE_FIFO_1 << 6 | UART_SEND_FIFO_8 << 4 | 0x1 << 3 | 0x1;
        if let Some(serial) = SERIAL.get() {
            let uart = serial.0.lock();
            uart.uart_lcr.write(|w| unsafe{ w.bits(1 << 7) });
            uart.uart_dlh().write(|w| unsafe{ w.bits(dlh as u32) });
            uart.uart_dll().write(|w| unsafe{ w.bits(dll as u32) });
            uart.uart_dlf.write(|w| unsafe{ w.bits(dlf as u32) });
            uart.uart_lcr.write(|w| unsafe{ w.bits(0) });
            uart.uart_lcr.write(
                |w| unsafe{ w.bits((data_width - 5) | (stop_bit << 2) | (parity << 3)) });
            let tmp_ier = uart.uart_ier().read().bits();
            uart.uart_ier().write(|w| unsafe{ w.bits(tmp_ier | 0x80) });
            uart.uart_fcr().write(|w| unsafe{ w.bits(0 << 6 | 2 << 4 | 0x1 << 3 | 0x1) });
        }

    }
}

pub(crate) fn init(uart0_base: usize) {
    SERIAL.call_once(|| unsafe { Uart0::new(uart0_base) });
    Uart0::debug_init();
}


struct Stdout;

impl fmt::Write for Stdout {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(serial) = SERIAL.get() {
            let serial = serial.0.lock();
            loop {
                if serial.uart_lsr.read().bits() & (1 << 5) == 0 {
                    break
                }
            }
            for byte in s.as_bytes() {
                serial.uart_thr().write(|w| unsafe{ w.bits(*byte as u32) });
            }
        }
        Ok(())
    }
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    Stdout.write_fmt(args).unwrap();
}

#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(core::format_args!($($arg)*)));
}

#[macro_export(local_inner_macros)]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::uart::_print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}
