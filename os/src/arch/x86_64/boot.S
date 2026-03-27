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

section .text progbits alloc exec nowrite

global _start_rust
_start_rust:
    ; Point to kernel stack
    mov rsp, stack_top

    ; Call kernel entry (Rust will handle the rest)
    extern _start
    call _start

    ; If kernel returns, halt
.halt:
    hlt
    jmp .halt

; Load GDT function
global load_gdt
load_gdt:
    lgdt [rdi]
    ret

; Reload segments
global reload_segments
reload_segments:
    mov ax, 0x10                ; Kernel data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Far jump to reload CS
    push 0x08
    push $.reload_cs
    retfq
.reload_cs:
    ret

; Load IDT function
global load_idt
load_idt:
    lidt [rdi]
    ret

; Enable interrupts
global enable_interrupts
enable_interrupts:
    sti
    ret

; Disable interrupts
global disable_interrupts
disable_interrupts:
    cli
    ret

; Enable paging
global enable_paging
enable_paging:
    mov rax, cr0
    or rax, 0x80000000
    mov cr0, rax
    ret

section .bss nobits alloc write
align 16
stack_bottom:
    resb 16384                  ; 16KB stack
stack_top:
