/* defines the entry point, bootloader will jump here after loading the kernel */
ENTRY(start)

/* tells the linker how to map the input section(from the assembly) to the output section (in the binary) */
SECTIONS {

    /* sets the load address of the first section to 1 MiB, which is a conventional place to load a kernel */
    . = 1M;

    /* contains all input sections named .boot */
    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    }

    /* contains all input sections named .text */
    .text :
    {
        *(.text)
    }
}