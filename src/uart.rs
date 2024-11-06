use lego_spec::driver::CharDevice;
use uart8250::Uart;

pub const UART_BASE: usize = 0x10000000;
static mut UART: Uart = Uart::new(UART_BASE, 13);
#[macro_export]
macro_rules! println {
    () => {
        writeln!(unsafe{$crate::char_dev_mut()}).unwrap();
    };
    ($($arg:tt)*) => {
        writeln!(unsafe{$crate::char_dev_mut()},$($arg)*).unwrap();
    };
}

pub fn init() {
    let uart_mut = unsafe { char_dev_mut() };
    uart_mut.init().unwrap()
}

/// # Safety
/// The type that implements CharDevice should be declared as a global variable.
#[inline]
pub unsafe fn char_dev_mut() -> &'static mut dyn CharDevice {
    (&raw mut UART as *mut dyn CharDevice).as_mut().unwrap()
}
