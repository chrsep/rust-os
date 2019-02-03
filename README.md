# rust-os

Repo for learning and experimenting with building
OS using rust, I try to put a lot of comments so that the codebase
can be easier to read for programmers that are unfamiliar with rust 
or OS development (just like me when i started this). Based on https://os.phil-opp.com/.

## Requirements
1. rust, install with `curl https://sh.rustup.rs -sSf | sh`
2. bootimage tool, install with `cargo install bootimage`
3. QEMU for running as a VM.
4. Linux, this codebase is only tested to compile and run correctly on Linux.

## How to run
Thanks to bootimage, running and building the OS is now a breeze
1. Build with `bootimage build`
2. Run with `bootimage run` to run on QEMU

## Goals

### Done
- Bootable
- Display text

### Todo
- Unit Tests
- Integration Test
- CPU Exceptions
- Double Faults
- Hardware Interrupts
- Paging
