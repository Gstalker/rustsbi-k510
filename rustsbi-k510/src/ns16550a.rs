use core::fmt;
use spin::{once::Once, Mutex};
use uart_16550::MmioSerialPort;

pub(crate) static SERIAL: Once<Ns16550a> = Once::new();

pub(crate) struct Ns16550a(Mutex<MmioSerialPort>);

impl Ns16550a {
    pub unsafe fn new(base: usize) -> Self {
        Self(Mutex::new(MmioSerialPort::new(base)))
    }
}

struct Stdout;

impl fmt::Write for Stdout {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(serial) = SERIAL.get() {
            let mut serial = serial.0.lock();
            for byte in s.as_bytes() {
                serial.send(*byte)
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
    ($($arg:tt)*) => ($crate::ns16550a::_print(core::format_args!($($arg)*)));
}

#[macro_export(local_inner_macros)]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::ns16550a::_print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}
