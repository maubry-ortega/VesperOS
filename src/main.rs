#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};
use core::panic::PanicInfo;
// Importamos los componentes necesarios de la nueva librería de fuentes
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

// Petición de framebuffer al gestor de arranque Limine.
static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Ensure we got a framebuffer.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let mut writer = FramebufferWriter::new(&framebuffer);

            // Limpiamos la pantalla a un color azul oscuro.
            writer.clear(0x00000088);

            // Usamos la macro `writeln!` para escribir en el framebuffer.
            writeln!(writer, "  Hola desde VesperOS!").unwrap();
            writeln!(writer, "Esta fuente viene de un crate de Cargo.").unwrap();
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

// --- Estructura para escribir en el Framebuffer ---

struct FramebufferWriter {
    framebuffer: &'static mut [u8],
    width: usize,
    height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
    x_pos: usize,
    y_pos: usize,
}

impl FramebufferWriter {
    fn new(fb: &limine::framebuffer::Framebuffer) -> Self {
        Self {
            framebuffer: unsafe {
                core::slice::from_raw_parts_mut(fb.addr(), (fb.pitch() * fb.height()) as usize)
            },
            width: fb.width() as usize,
            height: fb.height() as usize,
            pitch: fb.pitch() as usize,
            bytes_per_pixel: (fb.bpp() / 8) as usize,
            x_pos: 20, // Margen izquierdo
            y_pos: 20, // Margen superior
        }
    }

    fn clear(&mut self, color: u32) {
        let color_bytes = color.to_le_bytes();
        let pixel = &color_bytes[..self.bytes_per_pixel];
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = y * self.pitch + x * self.bytes_per_pixel;
                self.framebuffer[offset..offset + self.bytes_per_pixel].copy_from_slice(pixel);
            }
        }
        self.x_pos = 20;
        self.y_pos = 20;
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * self.bytes_per_pixel;
        let pixel_bytes = color.to_le_bytes();
        self.framebuffer[offset..offset + self.bytes_per_pixel]
            .copy_from_slice(&pixel_bytes[..self.bytes_per_pixel]);
    }

    fn write_char(&mut self, c: char) {
        // Definimos el estilo de la fuente que queremos usar del crate.
        const FONT_WEIGHT: FontWeight = FontWeight::Regular;
        const FONT_HEIGHT: RasterHeight = RasterHeight::Size16;
        // Obtenemos el ancho de los caracteres para esta fuente.
        let font_width = get_raster_width(FONT_WEIGHT, FONT_HEIGHT);

        match c {
            '\n' => {
                self.y_pos += FONT_HEIGHT.val();
                self.x_pos = 20;
            }
            c => {
                if self.x_pos + font_width > self.width {
                    self.y_pos += FONT_HEIGHT.val();
                    self.x_pos = 20;
                }
                if self.y_pos + FONT_HEIGHT.val() > self.height {
                    return; // Evita escribir fuera de la pantalla
                }

                // Busca el glifo del carácter en la librería.
                // Si no lo encuentra, usa el glifo para '?'
                let char_raster = get_raster(c, FONT_WEIGHT, FONT_HEIGHT)
                    .unwrap_or_else(|| get_raster('?', FONT_WEIGHT, FONT_HEIGHT).unwrap());

                for (dy, row) in char_raster.raster().iter().enumerate() {
                    for (dx, &intensity) in row.iter().enumerate() {
                        // La nueva versión usa escala de grises (0-255). Dibujamos si no es totalmente negro.
                        if intensity > 0 {
                            self.write_pixel(self.x_pos + dx, self.y_pos + dy, 0x00FFFFFF); // Blanco
                        }
                    }
                }
                self.x_pos += char_raster.width();
            }
        }
    }
}

impl fmt::Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}


// ¡El módulo `font` ya no es necesario! Lo hemos reemplazado con el crate `noto_sans_mono_bitmap`.

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