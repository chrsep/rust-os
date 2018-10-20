// Binary needs to be freestanding (run without underlying OS),
// stdlib have dependency to OS, so we disable it
#![no_std]
// Freestanding binary have no access to rust's runtime and crt0.
// crt0 is the entry point for rust programs. no_main override the
// entry point, so crt0 and rust's runtime doesnt get called.
#![no_main]
use core::panic::PanicInfo;

// _start is the default entry point for programs in linux, so
// we make this function to serve as our program entry point.
//
// no_mange used to prevent name from being mangled during compilation
// extern "C" tells the compiler to use C calling convention for this function.
// Return type is never (`!`) aka never returns, because this function is
// called by the bootloader directly so doesn't need to return. call exit to stop.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Called when panic occurs
/// A diverging function, return type is never (`!`) aka never returns
///
/// # Param
/// _info: location and message from the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

