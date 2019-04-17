// ENTRY POINT
// GOAL:
// Acts as the entry point for the OS, this setup all the
// basic rules, requirements, and modules that will allow the OS to be freestanding,
// working and bootable.

// when in test, allow unused imports
#![cfg_attr(test, allow(unused_imports))]

// Binary needs to be freestanding (runs without underlying OS),
// stdlib have dependency to OS, so we disable it
#![cfg_attr(not(test), no_std)]

// Freestanding binary have no access to rust's runtime and crt0.
// crt0 is the entry point for rust programs. no_main override the
// entry point, so crt0 and rust's runtime doesn't get called.
// "default entry point (crt0 -> rust runtime -> main)"
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

// Hello word text in byte slices
static HELLO: &[u8] = b"Hello World";

// Load Bootloader
extern crate bootloader_precompiled;

// Only compile when test flag not set
#[cfg(not(test))]
// _start is the default entry point for programs in linux, so
// we make this function to serve as our program entry point.
//
// no_mangle used to prevent name from being mangled during compilation
// extern "C" tells the compiler to use C calling convention for this function.
// Return type is never (`!`) aka never returns, because this function is
// called by the bootloader directly so doesn't need to return normally. It calls exit to stop.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");

    loop {}
}

// Only compile when test flag not set
#[cfg(not(test))]
// Handles Panic
// A diverging function, return type is never (`!`) aka never returns
//
// # Param
// _info: location and message from the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// Load vga_buffers module.
// An abstraction that encapsulate the un-safety of using the VGA text buffer
// through memory-mapped I/O
mod vga_buffers;
