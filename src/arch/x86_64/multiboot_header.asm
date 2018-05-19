; This is an X86 assembly file
; .multiboot_header is used on the linker.ld for linking the binary
section .multiboot_header

; header_start and header_end is a label that marks a memory location (here 
; it marks the beginning and end of the header in memory)
header_start:
    ; dd = define double
    dd 0xe85250d6                   ; magic number (to indicate multiboot 2 support)
    dd 0                            ; architecture 0 (0 idicates protected mode i386)
    dd header_end - header_start    ; header length
    ; checksum, 0x100000000(2^32) is used to avoid compiler warning caused by 
    ; integer underflow in checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; dd = define word
    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dw 8    ; size
header_end: