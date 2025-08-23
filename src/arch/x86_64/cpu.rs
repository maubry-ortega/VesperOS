//! Implementaciones de bajo nivel para el control de la CPU x86_64.

use core::arch::asm;

/// Habilita las interrupciones de hardware (`sti`).
///
/// # Safety
///
/// Habilitar interrupciones debe hacerse solo cuando se ha configurado
/// correctamente una Tabla de Descriptores de Interrupciones (IDT).
#[inline]
pub unsafe fn enable_interrupts() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}

/// Detiene la CPU hasta la próxima interrupción (`hlt`).
///
/// Es una forma eficiente de esperar eventos sin consumir ciclos de CPU.
#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack));
    }
}

/// Lee un byte del puerto de E/S especificado.
///
/// # Safety
///
/// Leer de un puerto de E/S es una operación privilegiada y puede tener
/// efectos secundarios en el hardware. El puerto debe ser válido.
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack, preserves_flags));
    }
    value
}