/* Since integration tests are separate executables, 
 * we need to provide all the crate attributes 
 * (no_std, no_main, test_runner, etc.) again. 
 * We also need to create a new entry point function 
 * _start, which calls the test entry point function 
 * test_main. We don't need any cfg(test) attributes 
 * because integration test executables are never 
 * built in non-test mode.
 */
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(vailo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use vailo::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vailo::test_panic_handler(info)
}
