// Binary needs to be freestanding (run without underlying OS),
// stdlib have dependency to OS, so we disable it
#![no_std]
// Freestanding binary have no access to rust's runtime and crt0.
// crt0 is the entry point for rust programs. no_main override the
// entry point, so crt0 and rust's runtime doesnt get called.
// default entry point (crt0 -> rust runtime -> main)
#![no_main]
use core::panic::PanicInfo;

// Hello word text in byte slices
static HELLO: &[u8] = b"Hello World";

// Load Bootloader
extern crate bootloader_precompiled;
// _start is the default entry point for programs in linux, so
// we make this function to serve as our program entry point.
//
// no_mange used to prevent name from being mangled during compilation
// extern "C" tells the compiler to use C calling convention for this function.
// Return type is never (`!`) aka never returns, because this function is
// called by the bootloader directly so doesn't need to return normally. It calls exit to stop.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // A SIMPLE HELLO WORLD TO VGA BUFFER
    // cast 0xb8000(default location of vga buffer) to raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    // iterates over the HELLO bytes
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // print into the vga buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

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

