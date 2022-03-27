#![no_std]

use core::panic::PanicInfo;


// This function is called on panic.
// ! (never type) means it's a diverging fn - no return.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Loop indefinitely
    loop {}
}

fn main() {}
