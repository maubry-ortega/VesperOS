// src/arch/x86_64/interrupts.S

.intel_syntax noprefix // Usar sintaxis de Intel para el ensamblador.

// Hacemos que los símbolos de nuestras funciones Rust sean visibles para el ensamblador.
.extern rust_timer_interrupt_handler
.extern rust_keyboard_interrupt_handler

// Macro para crear un manejador de interrupciones genérico.
// Esta es una práctica estándar en el desarrollo de sistemas operativos.
// No necesita manejar códigos de error para las interrupciones del PIC (timer, teclado).
.macro interrupt_handler_stub name, rust_handler_func
    .global \name
\name:
    // Guardar registros de propósito general
    push rax
    push rcx
    push rdx
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11

    // Llamar a la rutina en Rust
    call \rust_handler_func

    // Restaurar registros en orden inverso.
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rax

    // Retornar de la interrupción
    iretq
.endm

// Usamos la macro para crear los manejadores para el timer y el teclado.
interrupt_handler_stub timer_interrupt_stub, rust_timer_interrupt_handler
interrupt_handler_stub keyboard_interrupt_stub, rust_keyboard_interrupt_handler
