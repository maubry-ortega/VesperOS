/* linker.ld - Con PHDRS explícitos para un mapeo de memoria correcto */
ENTRY(_start)
PAGE_SIZE = 0x1000;

PHDRS
{
    text PT_LOAD FLAGS(5);   /* R+X */
    rodata PT_LOAD FLAGS(4); /* R */
    data PT_LOAD FLAGS(6);   /* R+W */
}

SECTIONS {
    . = 0xffffffff80000000;

    .text : ALIGN(PAGE_SIZE) {
        KEEP(*(.text.start))
        *(.text .text.*)
    } :text

    .rodata : ALIGN(PAGE_SIZE) {
        KEEP(*(.rodata))
        *(.rodata .rodata.*)
    } :rodata

    .data : ALIGN(PAGE_SIZE) {
        KEEP(*(.data))
        *(.data .data.*)
    } :data

    .bss : ALIGN(PAGE_SIZE) {
        KEEP(*(.bss))
        *(.bss .bss.*)
    } :data

    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
    }
}