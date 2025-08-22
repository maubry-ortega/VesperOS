#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

// Importamos nuestros módulos recién creados
mod vesperfetch;
mod vga;
mod colors;
mod branding; // Necesario para el logo

// Petición de framebuffer al gestor de arranque Limine.
static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();
// Petición del mapa de memoria para saber cuánta RAM tenemos.
static MEMMAP_REQUEST: limine::request::MemoryMapRequest = limine::request::MemoryMapRequest::new();

// Tu toolchain de Rust parece requerir que `no_mangle` esté dentro de un bloque `unsafe`.
// Esto no es estándar en todas las versiones, pero es necesario para que compile en tu entorno.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Ensure we got a framebuffer.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let mut writer = vga::FramebufferWriter::new(&framebuffer); 

            // --- Etapa 1: Pantalla de bienvenida con logo ---
            {
                writer.clear(colors::DEEP_BLACK);
                
                // 1. Dibuja el logo centrado en la parte superior de la pantalla.
                // Esto utiliza el método `draw_image`, que también mueve el cursor debajo.
                let logo_x = (framebuffer.width() as usize - branding::VESPER_LOGO_IMAGE.width) / 2;
                let logo_y = (framebuffer.height() as usize / 2) - 120; // Un poco por encima del centro
                writer.draw_image(&branding::VESPER_LOGO_IMAGE, logo_x, logo_y);

                // 2. Dibuja el texto centrado debajo del logo.
                // `writer.y_pos()` ahora está en la posición correcta gracias a `draw_image`.
                let text_y_start = writer.y_pos();
                writer.draw_centered_text(text_y_start, "Vesper OS v0.1.0", colors::TEXT_PRIMARY);
                writer.draw_centered_text(text_y_start + 25, "Portable WebAssembly OS", colors::NEON_GREEN);
                writer.draw_centered_text(text_y_start + 60, "Inicializando subsistemas...", colors::TEXT_PRIMARY);
            }

            // Pequeña pausa para que se vea la pantalla de bienvenida
            for _ in 0..500_000_000 { unsafe { asm!("nop"); } }

            // --- Etapa 2: Mostrar VesperFetch con información del sistema ---
            {
                writer.clear(colors::BACKGROUND_COLOR);

                let mut total_memory: u64 = 0;
                if let Some(memmap_response) = MEMMAP_REQUEST.get_response() {
                    for entry in memmap_response.entries() {
                        if entry.entry_type == limine::memory_map::EntryType::USABLE {
                            total_memory += entry.length;
                        }
                    }
                }

                let sys_info = vesperfetch::SystemInfo {
                    os_name: "Vesper OS",
                    os_version: "0.1.0 (Nocturna)",
                    kernel_version: "Vesper-Core",
                    cpu_info: "N/A", // Placeholder
                    uptime: "N/A",   // Placeholder
                    memory_total_mb: total_memory / 1024 / 1024,
                    resolution_width: framebuffer.width(),
                    resolution_height: framebuffer.height(),
                };

                // Dibujamos VesperFetch centrado
                let fetch_x = (framebuffer.width() as usize - 400) / 2;
                let fetch_y = (framebuffer.height() as usize - 200) / 2;

                // Se elimina la animación para evitar el parpadeo y se dibuja el estado final.
                let fetch = vesperfetch::VesperFetch::new(sys_info);
                fetch.display(&mut writer, fetch_x, fetch_y);
            }
        }
    }

    hcf();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf();
}

// Detiene el CPU de forma segura para siempre.
fn hcf() -> ! {
    // Primero, deshabilitamos las interrupciones para asegurar que `hlt` no sea interrumpido.
    unsafe { asm!("cli", options(nomem, nostack)); }
    loop {
        // Detenemos el CPU hasta la próxima interrupción (que nunca llegará).
        unsafe { asm!("hlt", options(nomem, nostack)); }
    }
}

// Secciones específicas para ayudar al linker a organizar la memoria correctamente
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