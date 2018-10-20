# rust-os

OS built on rust.

## Requirements
1. Install rust `curl https://sh.rustup.rs -sSf | sh`
2. Install bootimage tool `cargo install bootimage`
3. QEMU for running as a VM.

## How to run
Thanks to bootimage, running and building the OS is now a breeze
1. Build with `bootimage build`
2. Run with `bootimage run` to run on QEMU