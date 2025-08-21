#![no_std]
#![no_main]

use core::panic::PanicInfo;
 
// Petición de framebuffer al gestor de arranque Limine.
static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();

// Declaramos la función `hlt` que estará definida en un archivo de ensamblador externo.
// En la edición 2024, los bloques extern deben marcarse como `unsafe`.
unsafe extern "C" {
    fn hlt();
}

// En la edición 2024, `no_mangle` es un atributo inseguro.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Ensure we got a framebuffer.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            // Note: we assume the framebuffer model is RGB with 32-bit pixels.
            for i in 0..100 {
                // Escribir directamente en la memoria del framebuffer es una operación insegura.
                unsafe {
                    core::ptr::write_volatile(
                        (framebuffer.addr() as *mut u32).add(i * framebuffer.pitch() as usize / 4 + i),
                        0xFFFFFFFF,
                    );
                }
            }
        }
    }

    hcf();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf();
}

// Una función simple para detener el CPU de forma segura.
fn hcf() -> ! {
    loop {
        // Llamamos a nuestra función `hlt` externa. Es `unsafe` porque el compilador
        // no puede verificar lo que hace el código de ensamblador.
        unsafe { hlt(); }
    }
}
