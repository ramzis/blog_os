#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// This function is called on panic.
// ! (never type) means it's a diverging fn - no return.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Loop indefinitely
    loop {}
}

// Overwriting the main entry point.
// no_mangle ensures the compiler keeps the '_start' name to use for linking.
// extern "C" specifies the calling convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
