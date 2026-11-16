/* Simple linker script for a flat binary kernel */
OUTPUT_FORMAT(elf32-i386)
ENTRY(_start)

SECTIONS {
    . = 1M; /* load address */
    .text : {
        *(.multiboot)
        *(.text*)
    }
    .rodata : { *(.rodata*) }
    .data   : { *(.data*) }
    .bss    : {
        *(COMMON)
        *(.bss*)
    }
}
