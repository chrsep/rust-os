/* Defines the entry point, bootloader will jump here after loading the kernel*/
ENTRY(start)

SECTIONS {
    . = 1M;
    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}