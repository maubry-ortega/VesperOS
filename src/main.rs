//! Punto de entrada principal del kernel de VesperOS.
//!
//! Este archivo contiene la función `_start`, que es la primera pieza de código
//! Rust que se ejecuta después de que el gestor de arranque (Limine) carga el
//! kernel. Se encarga de inicializar los subsistemas básicos, mostrar la
//! pantalla de bienvenida y entrar en el bucle principal del sistema.

#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

// --- Módulos del Kernel ---
mod vesperfetch;
mod vga;
mod colors;
mod branding;
mod shell;
mod app;
mod arch;

/// Petición al gestor de arranque Limine para obtener un framebuffer.
///
/// El framebuffer es una región de memoria que representa los píxeles de la pantalla,
/// permitiendo el dibujo en modo gráfico.
static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();

/// Petición al gestor de arranque Limine para obtener el mapa de memoria.
///
/// El mapa de memoria nos informa sobre qué regiones de la RAM están
/// disponibles para ser usadas por el kernel.
pub static MEMMAP_REQUEST: limine::request::MemoryMapRequest = limine::request::MemoryMapRequest::new();

/// Punto de entrada del kernel, llamado por el gestor de arranque.
///
/// Esta función no debe retornar nunca, por eso su tipo de retorno es `!`.
///
/// # Safety
///
/// El atributo `no_mangle` asegura que el nombre de esta función no sea alterado
/// por el compilador, para que el enlazador (linker) pueda encontrarla con el
/// nombre `_start`. El bloque `unsafe` es requerido por la edición de Rust 2024
/// para este atributo.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // En este punto, las interrupciones de hardware están deshabilitadas.
    // Primero, nos aseguramos de que el gestor de arranque nos haya proporcionado
    // un framebuffer para poder dibujar en la pantalla.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let mut writer = vga::FramebufferWriter::new(&framebuffer); 

            // --- Etapa 1: Pantalla de bienvenida y carga ---
            {
                writer.clear(colors::DEEP_BLACK);
                
                let logo_x = (framebuffer.width() as usize - branding::VESPER_LOGO_IMAGE.width) / 2;
                let logo_y = (framebuffer.height() as usize / 2) - 120; // Un poco por encima del centro
                writer.draw_image(&branding::VESPER_LOGO_IMAGE, logo_x, logo_y);

                let text_y_start = writer.y_pos();
                writer.draw_centered_text(text_y_start, "Vesper OS v0.1.0", colors::TEXT_PRIMARY);
                writer.draw_centered_text(text_y_start + 25, "Portable WebAssembly OS", colors::NEON_GREEN);

                // --- Barra de carga ---
                let bar_width = 300;
                let bar_height = 15;
                let bar_x = (writer.width() - bar_width) / 2;
                let bar_y = text_y_start + 70;

                // Dibuja el fondo/borde de la barra
                writer.draw_rect(bar_x, bar_y, bar_width, bar_height, colors::COSMIC_BLUE);

                let inner_bar_width = bar_width - 4;
                // Simula la carga llenando la barra progresivamente
                for i in 0..=inner_bar_width {
                    // Dibuja la parte de progreso de la barra
                    writer.draw_rect(bar_x + 2, bar_y + 2, i, bar_height - 4, colors::NEON_GREEN);
                    // Pequeña pausa para que la animación sea visible. Ajusta el valor para cambiar la velocidad.
                    for _ in 0..1_500_000 { unsafe { asm!("nop"); } }
                }
            }

            // --- Etapa 2: Inicializar Interrupts y Shell ---
            arch::init(); // Configura la IDT, el PIC y habilita las interrupciones.

            writer.clear(colors::BACKGROUND_COLOR);
            let mut shell = shell::Shell::new();
            shell.draw_prompt(&mut writer);

            // --- Etapa 3: Bucle principal del Kernel ---
            // Ahora que las interrupciones están habilitadas, podemos
            // recibir entrada del teclado.
            loop {
                // Sondea en busca de una tecla presionada.
                if let Some(key) = arch::target::keyboard::poll_key() {
                    if let pc_keyboard::DecodedKey::Unicode(character) = key {
                        // Pasa el carácter a la shell para que lo procese.
                        shell.handle_input_char(character, &mut writer);
                    }
                }
                // Detiene la CPU hasta la próxima interrupción para ahorrar energía.
                arch::wait_for_interrupt();
            }
        }
    }

    // Si no hay framebuffer o algo falla, detenemos el sistema.
    hcf();
}

/// Manejador de pánicos.
///
/// Esta función se llama cuando el kernel entra en pánico.
/// Su única tarea es detener la CPU de forma segura para prevenir más daños.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Por ahora, simplemente llamamos a hcf(). En el futuro, podría imprimir
    // la información del pánico en la pantalla.
    hcf();
}

/// Detiene la CPU de forma segura para siempre (Halt and Catch Fire).
///
/// Esta función deshabilita las interrupciones y luego entra en un bucle infinito
/// que pone a la CPU en un estado de bajo consumo (`hlt`). Esto detiene toda
/// ejecución de manera efectiva y segura.
fn hcf() -> ! {
    // Deshabilitamos las interrupciones para asegurar que `hlt` no sea interrumpido.
    unsafe { asm!("cli", options(nomem, nostack)); }
    loop {
        // Detenemos el CPU hasta la próxima interrupción (que nunca llegará).
        unsafe { asm!("hlt", options(nomem, nostack)); }
    }
}

// --- Secciones del Linker ---
// Estos símbolos estáticos vacíos ayudan al script del linker a organizar
// el binario del kernel en las secciones de memoria correctas:
// .text: Código ejecutable
// .rodata: Datos de solo lectura
// .data: Datos inicializados
// .bss: Datos no inicializados (inicializados a cero)

#[used]
#[unsafe(link_section = ".text.start")]
static TEXT_START: [u8; 0] = [];

#[used]
#[unsafe(link_section = ".rodata")]
static RODATA_START: [u8; 0] = [];

#[used]
#[unsafe(link_section = ".data")]
static DATA_START: [u8; 0] = [];

#[used]
#[unsafe(link_section = ".bss")]
static BSS_START: [u8; 0] = [];