//! Implementación mínima y personalizada de la Tabla de Descriptores de Interrupciones (IDT).

use core::mem::size_of;
use core::ops::{Index, IndexMut};

/// Representa una entrada en la IDT.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    pointer_low: u16,
    gdt_selector: u16,
    options: u16,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl IdtEntry {
    /// Crea una nueva entrada de IDT vacía.
    pub const fn new() -> Self {
        IdtEntry {
            pointer_low: 0,
            gdt_selector: 0,
            options: 0,
            pointer_middle: 0,
            pointer_high: 0,
            reserved: 0,
        }
    }

    /// Establece la función manejadora para esta interrupción.
    pub fn set_handler(&mut self, handler_addr: u64) -> &mut Self {
        self.pointer_low = handler_addr as u16;
        self.pointer_middle = (handler_addr >> 16) as u16;
        self.pointer_high = (handler_addr >> 32) as u32;
        // El selector de segmento 0x8 es el segmento de código en nuestra GDT.
        self.gdt_selector = 0x8;
        // Opciones: Presente=1, Nivel de Privilegio=0 (kernel), Tipo=Puerta de Interrupción de 32 bits.
        self.options = 0x8E00;
        self
    }
}

/// Representa la IDT, un array de 256 entradas.
#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    entries: [IdtEntry; 256],
}

impl InterruptDescriptorTable {
    /// Crea una nueva IDT con todas las entradas a cero.
    pub const fn new() -> Self {
        InterruptDescriptorTable {
            entries: [IdtEntry::new(); 256],
        }
    }

    /// Carga la IDT en la CPU usando la instrucción `lidt`.
    ///
    /// # Safety
    ///
    /// Esta función es insegura porque modifica un registro global de la CPU (IDTR).
    /// Debe llamarse solo una vez durante la inicialización.
    pub fn load(&'static self) {
        #[repr(C, packed(2))]
        struct DescriptorTablePointer {
            size: u16,
            address: u64,
        }

        let pointer = DescriptorTablePointer {
            size: (size_of::<Self>() - 1) as u16,
            address: self as *const _ as u64,
        };

        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));
        }
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = IdtEntry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}