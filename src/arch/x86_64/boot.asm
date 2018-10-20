; make the start label global, since it will be the entry point of the
; kernel (defined in linke.ld `ENTRY(start)`)
global start

; .text section is for executable code
section .text
; specifies that below is a 32 bit instruction
bits 32
; the label that were made global as the entrypoint
start:
    ; make the stack pointer points to stack_top
    mov esp, stack_top

    ; do all the checks by calling the functions
    call check_multiboot
    call check_cpuid
    call check_long_mode

    ; move 32-bit constant 0x2f4b2f4f to 0xb8000 to print OK
    mov dword [0xb8000], 0x2f4b2f4f

    ; halt command, stops the CPU until next interrupt
    hlt

error:
    ; 0xb8000 is where the vga text buffer starts
    ; 0x4f524f45 0x4f3a4f52 0x4f204f20 meaning:
    ; 0x4f means white text on red background
    ; 0x52 means  ASCII `R`
    ; 0x45 means ASCII `E`
    ; 0x3a means ASCII `:`
    ; 0x20 means ASCII space
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    ; al is the lower 8 bit of the 32 bit EAX register
    mov byte [0xb800a], al
    hlt

; a check for making sure the bootloader supports multiboot 2
check_multiboot:
    ; multiboot 2 compatible bootloader pushes 0x36d76289 
    ; to eax after successfuly loading the kernel, this value
    ; is what we check
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    ; move error code to al for `error:` to read
    mov al, "0"
    jmp error

; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
; in the FLAGS register. If we can flip it, CPUID is available.
check_cpuid:
    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

check_long_mode:
    ; CPUID works by taking argument from eax and loads information into the ecx and edx register
    ; test if extended processor info in available(older cpu does not have it)
    mov eax, 0x80000000    ; implicit argument for cpuid for requesting highest supported parameter value
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001 to support extended processor info
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

; .bss section is used for local common variable storage
section .bss
stack_bottom:
    ; resb reserves bytes, 64 bytes in this case
    resb  64
stack_top: