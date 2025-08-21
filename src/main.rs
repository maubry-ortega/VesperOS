#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// ===== Cabecera Multiboot2 válida (GRUB) =====
/// Debe estar alineada a 8 bytes y dentro de los primeros 32 KiB del binario.
#[repr(C)]
struct EndTag {
    typ: u16,
    flags: u16,
    size: u32,
}

#[repr(C, align(8))]
struct Multiboot2Header {
    magic: u32,
    arch: u32,
    length: u32,
    checksum: u32,
    end_tag: EndTag,
}

const MBOOT2_MAGIC: u32 = 0xE85250D6;
const MBOOT2_ARCH_I386: u32 = 0;

const HEADER_LEN: u32 = core::mem::size_of::<Multiboot2Header>() as u32;
const CHECKSUM: u32 = 0u32
    .wrapping_sub(MBOOT2_MAGIC)
    .wrapping_sub(MBOOT2_ARCH_I386)
    .wrapping_sub(HEADER_LEN);

#[unsafe(link_section = ".multiboot2_header")]
#[unsafe(no_mangle)]
pub static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
    magic: MBOOT2_MAGIC,
    arch: MBOOT2_ARCH_I386,
    length: HEADER_LEN,
    checksum: CHECKSUM,
    end_tag: EndTag { typ: 0, flags: 0, size: 8 },
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // VGA texto
    let vga = 0xb8000 as *mut u8;
    unsafe {
        *vga.add(0) = b'H'; *vga.add(1) = 0x0f;
        *vga.add(2) = b'o'; *vga.add(3) = 0x0f;
        *vga.add(4) = b'l'; *vga.add(5) = 0x0f;
        *vga.add(6) = b'a'; *vga.add(7) = 0x0f;
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
