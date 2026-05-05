#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

mod kprint;
mod sbi;

// Declare symbols defined in the linker script.
unsafe extern "C" {
    static mut __bss_start: u8;
    static mut __bss_end: u8;
    static __stack_top: u8;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
extern "C" fn boot() -> ! {
    naked_asm!(
        "la sp, {sp}",
        "j {main}",
        sp = sym __stack_top,
        main = sym kernel_main,
    );
}

fn kernel_main() -> ! {
    // Zero out the BSS region.
    unsafe {
        let bss_start = &raw mut __bss_start;
        let bss_end = &raw mut __bss_end;
        let bss_len = bss_end.offset_from(bss_start) as usize;
        // The linker script guarantess that `__bss <= __bss_end`,
        // hence the previous cast to `usize` is safe.
        core::ptr::write_bytes(bss_start, 0, bss_len);
    }

    kprint!("hello world!\n");
    kprint!("a = {}, b = {:?}\n", 42, core::f64::consts::PI);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
