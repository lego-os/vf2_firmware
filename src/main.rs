#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{arch::global_asm, sync::atomic::AtomicBool};
use lego_spec::arch::riscv::mhartid;
use log::info;

use vf2_firmware::{init, load_bootloader, println};
global_asm!(include_str!("./entry.S"));
global_asm!(include_str!("jump.S"));
extern "C" {
    static _bss_start: usize;
    static _bss_end: usize;
    fn jump_loader(size: usize, addr: usize) -> !;
}
static BLOCK: AtomicBool = AtomicBool::new(true);
const LOAD_ADDRESS: usize = 0x40000000 + 2 * 1024 * 1024 * 1024;
static LOADER_SIZE: AtomicUsize = AtomicUsize::new(0);
#[no_mangle]
pub extern "C" fn rust_entry(code_end: usize) -> ! {
    if mhartid::read() == 1 {
        clear_bss();
        init(code_end);
        let size = load_bootloader(LOAD_ADDRESS);
        LOADER_SIZE.store(size, Ordering::SeqCst);
        BLOCK.store(false, Ordering::Relaxed);
        info!("prepare into loader program.");
    } else {
        while BLOCK.load(Ordering::Relaxed) {
            core::hint::spin_loop();
        }
    }
    unsafe { jump_loader(LOADER_SIZE.load(Ordering::SeqCst), LOAD_ADDRESS) }
}

fn clear_bss() {
    let mut bss = unsafe { _bss_start as *mut usize };
    let bss_end = unsafe { _bss_end as *mut usize };
    unsafe {
        while bss.lt(&bss_end) {
            (*bss) = 0;
            bss = bss.add(1);
        }
    }
}

#[panic_handler]
pub fn panic(println: &PanicInfo) -> ! {
    if let Some(location) = println.location() {
        println!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        println!("panic occurred but can't get location printlnrmation...");
    }

    println!("panic message: {:?}", println.message());
    loop {}
}
