/* src/boot/multiboot2_header.S */
/* Cabecera Multiboot2 en GAS syntax (para GRUB2) */

.set ALIGN,    8
.set MAGIC,    0xE85250D6
.set ARCH,     0
.set HEADER_LEN, header_end - header_start
.set CHECKSUM, -(MAGIC + ARCH + HEADER_LEN)

.section .multiboot2_header, "a"
.align ALIGN

header_start:
    .long MAGIC
    .long ARCH
    .long HEADER_LEN
    .long CHECKSUM

    /* Tag de terminación */
    .short 0
    .short 0
    .long 8
header_end:
