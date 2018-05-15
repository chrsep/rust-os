# rust-os
Simple OS built on rust, see https://os.phil-opp.com/

## Running
1. `make iso`
2. `qemu-system-x86_64 -cdrom build/os-x86_64.iso`

## Details
### Multiboot Kernel
The kernel source file is located in `src/arch/x86_64`. There are 3 files involved in building the kernel:
1. `multiboot_header.asm`: An assembly file containing the multiboot header to tell the bootloader that the kernel supports multiboot.
2. `boot.asm`: An assembly file containing the code to be exececuted when booting the kernel.
3. `linker.ld`: A script to link together assembled `multiboot_header.asm` and `boot.asm` into a single executable (`kernel-x86_64.bin`)

Grub2 is used as the bootloader.
