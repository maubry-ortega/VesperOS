ENTRY(_start)

SECTIONS {
    . = 1M;

    .text : {
        KEEP(*(.multiboot2_header))  /* Header primero */
        *(.text*)
    }

    .rodata : { *(.rodata*) }
    .data   : { *(.data*) }
    .bss    : { *(.bss*) *(COMMON) }
}
