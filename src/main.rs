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
    vailo::init();

    #[cfg(test)]
    test_main();

    //// trigger a page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;
    //};

    //fn stack_overflow() {
    //    stack_overflow(); // for each recursion, the return address is pushed
    //}

    //// trigger a stack overflow
    //stack_overflow();

    //// invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3(); // new

    //loop {}

    println!("It did not crash!");
    vailo::hlt_loop();
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
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    vailo::hlt_loop();
}
