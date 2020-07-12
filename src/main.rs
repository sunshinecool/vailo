#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]

use core::panic::PanicInfo;

/* This function is the entry point, since the linker 
 * looks for a function named `_start` by default
 */
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}

mod vga_buffer; // see src/vga_buffer.rs

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
