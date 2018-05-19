; make the start label global, since it will be the entry point of the
; kernel (defined in linke.ld `ENTRY(start)`)
global start

; .text section is for executable code
section .text
; specifies that below is a 32 bit instruction
bits 32
; the label that were made global as the entrypoint
start:
    ; move 32-bit constant 0x2f4b2f4f to 0xb8000 to print OK
    mov dword [0xb8000], 0x2f4b2f4f

    ; halt command, stops the CPU until next interrupt
    hlt