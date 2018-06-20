// freestanding binary implementation (requires nightly toolchain)
#![no_std] // dont link standard library
#![feature(panic_implementation)] // enable feature for defining panic handler
#![no_main] // disable the normal entry point chain (crt0 -> rust -> main) so we can define our own
use core::panic::PanicInfo;

// this function is called on panic.
#[panic_implementation]
#[no_mangle] // prevent function name from baing mangled
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// on macOS: default entry point used by the linker is main()
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
