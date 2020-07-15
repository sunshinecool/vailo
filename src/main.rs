#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(vailo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use vailo::println;


/* This function is the entry point, since the linker
 * looks for a function named `_start` by default
 */
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Loading Vailo...");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vailo::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
