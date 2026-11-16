# Minimal multiboot header and entry point
.section .multiboot
.align 4
.long 0x1BADB002          # magic number
.long 0x00                # flags (none)
.long -(0x1BADB002 + 0x00) # checksum

.section .bss
.align 16
stack_bottom:
    .skip 8192            # 8 KiB stack
stack_top:

.section .text
.intel_syntax noprefix
.global _start
_start:
    cli                     # disable interrupts
    lea esp, [stack_top]    # set stack pointer
    call rust_main          # jump to Rust kernel entry
hang:
    hlt
    jmp hang
