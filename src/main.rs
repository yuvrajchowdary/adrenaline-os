#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(adrenaline::test_runner)]
#![reexport_test_harness_main = "test_main"]

use adrenaline::println;
use adrenaline::println_err;
use core::panic::PanicInfo;
use adrenaline::vga_buffer::{SCREENER, BUFFER_HEIGHT, BUFFER_WIDTH};

#[no_mangle]
pub extern "C"  fn _start() -> ! {
    println!("Welcome to Adrenaline-OS\nYou are in the R--U--S--H Kernel version");

    adrenaline::init();

    #[cfg(test)]
    test_main();

    println!("Done !");
    adrenaline::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println_err!("{}", _info);
    adrenaline::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    adrenaline::test_panic_handler(_info);
}



