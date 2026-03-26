; AI-OS Bootloader
; Multiboot2 compliant boot sector (NASM syntax)

section .multiboot progbits alloc noexec write
align 8

multiboot2_header:
    dd 0xe85250d6              ; Magic
    dd 0                        ; Architecture (0 = i386)
    dd multiboot2_end - multiboot2_header ; Length
    dd 0x100000000 - (0xe85250d6 + 0 + (multiboot2_end - multiboot2_header)) ; Checksum

    ; Required end tag
    dw 0                        ; Type
    dw 0                        ; Flags
    dd 8                        ; Size
multiboot2_end:

section .text progbits alloc exec no write

global _start
_start:
    ; Point to kernel stack
    mov rsp, stack_top

    ; Call kernel entry
    extern _start_rust
    call _start_rust

    ; If kernel returns, halt
.halt:
    hlt
    jmp .halt

section .bss progbits alloc noexec write
align 16
stack_bottom:
    resb 16384                  ; 16KB stack
stack_top:
