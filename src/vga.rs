//! Módulo para manejar la escritura en el framebuffer (salida de video).

use core::fmt;
use core::fmt::Write;
use embedded_graphics::image::Image;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::*,
    text::{Baseline, Text},
};
use profont::PROFONT_14_POINT;
use tinybmp::Bmp;
use limine::framebuffer::Framebuffer;

use crate::colors; // Importamos nuestro módulo de colores
use crate::branding; // Importamos el módulo de branding con el logo

/// Estructura para escribir en el Framebuffer.
/// Encapsula la lógica para dibujar píxeles y caracteres.
pub struct FramebufferWriter {
    framebuffer: &'static mut [u8],
    width: usize,
    height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
    x_pos: usize,
    y_pos: usize,
    color: colors::Color,
}

impl FramebufferWriter {
    /// Crea una nueva instancia de `FramebufferWriter`.
    pub fn new(fb: &Framebuffer) -> Self {  // Recibe &Framebuffer directamente
        let addr = fb.addr() as *mut u8;
        let len = (fb.pitch() * fb.height()) as usize;
        
        Self {
            framebuffer: unsafe { core::slice::from_raw_parts_mut(addr, len) },
            width: fb.width() as usize,
            height: fb.height() as usize,
            pitch: fb.pitch() as usize,
            bytes_per_pixel: (fb.bpp() / 8) as usize,
            x_pos: 20, // Margen izquierdo inicial
            y_pos: 20, // Margen superior inicial
            color: colors::TEXT_PRIMARY, // Color de texto por defecto
        }
    }

    /// Establece la posición actual del cursor para la escritura de texto.
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.x_pos = x;
        self.y_pos = y;
    }

    /// Devuelve la posición vertical actual del cursor.
    pub fn y_pos(&self) -> usize {
        self.y_pos
    }

    /// Cambia el color de texto actual.
    pub fn set_color(&mut self, color: colors::Color) {
        self.color = color;
    }

    /// Limpia toda la pantalla con el color especificado.
    pub fn clear(&mut self, color: colors::Color) {
        let color_bytes = color.to_le_bytes();
        let pixel = &color_bytes[..self.bytes_per_pixel];
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = y * self.pitch + x * self.bytes_per_pixel;
                // Usamos `copy_from_slice` para escribir el píxel.
                self.framebuffer[offset..offset + self.bytes_per_pixel].copy_from_slice(pixel);
            }
        }
        // Reiniciamos la posición del cursor después de limpiar.
        self.x_pos = 20;
        self.y_pos = 20;
    }

    /// Dibuja un píxel. Esta es la función base para todo el renderizado.
    fn write_pixel(&mut self, x: usize, y: usize, color: colors::Color) {
        // Aseguramos que las coordenadas estén dentro de los límites de la pantalla.
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * self.bytes_per_pixel;
        let pixel_bytes = color.to_le_bytes();
        self.framebuffer[offset..offset + self.bytes_per_pixel]
            .copy_from_slice(&pixel_bytes[..self.bytes_per_pixel]);
    }

    /// Dibuja un carácter en la posición actual del cursor usando `embedded-graphics`.
    fn write_char(&mut self, c: char) {
        // Define el estilo del carácter usando la fuente ProFont y el color actual.
        // Convertimos el color u32 a componentes R, G, B para Rgb888.
        let r = ((self.color >> 16) & 0xFF) as u8;
        let g = ((self.color >> 8) & 0xFF) as u8;
        let b = (self.color & 0xFF) as u8;
        let char_style = MonoTextStyle::new(&PROFONT_14_POINT, Rgb888::new(r, g, b));

        match c {
            '\n' => {
                self.y_pos += PROFONT_14_POINT.character_size.height as usize;
                self.x_pos = 20;
            }
            c => {
                let char_width = PROFONT_14_POINT.character_size.width as usize;
                if self.x_pos + char_width > self.width {
                    self.y_pos += PROFONT_14_POINT.character_size.height as usize;
                    self.x_pos = 20;
                }

                // Crea un buffer de 1 carácter para dibujar
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);

                // Dibuja el texto usando embedded-graphics
                Text::with_baseline(s, Point::new(self.x_pos as i32, self.y_pos as i32), char_style, Baseline::Top)
                    .draw(self)
                    .unwrap();

                self.x_pos += char_width;
            }
        }
    }

    /// Dibuja una imagen de píxeles crudos en una posición específica.
    pub fn draw_image(&mut self, image: &branding::RawImage, start_x: usize, start_y: usize) {
        for y in 0..image.height {
            for x in 0..image.width {
                let color = image.data[y * image.width + x];
                // Si el color no es transparente, lo dibujamos.
                if color != colors::TRANSPARENT {
                    self.write_pixel(start_x + x, start_y + y, color);
                }
            }
        }
        // Movemos el cursor de texto debajo de la imagen.
        self.x_pos = 20;
        self.y_pos = start_y + image.height + 20; // 20px de margen
    }

    /// Dibuja texto centrado horizontalmente en una posición Y específica.
    pub fn draw_centered_text(&mut self, y: usize, text: &str, color: colors::Color) {
        let text_len_chars = text.chars().count();
        // Assuming average character width for centering. ProFont 14pt is 8px wide.
        let estimated_text_width_pixels = text_len_chars * 8; 
        let x = (self.width - estimated_text_width_pixels) / 2;
        self.set_cursor_position(x, y);
        self.set_color(color);
        write!(self, "{}", text).unwrap();
    }

    /// Dibuja una imagen de píxeles crudos sin afectar la posición del cursor de texto.
    pub fn blit_raw_image(&mut self, image: &branding::RawImage, start_x: usize, start_y: usize) {
        for y in 0..image.height {
            for x in 0..image.width {
                let color = image.data[y * image.width + x];
                // Si el color no es transparente, lo dibujamos.
                if color != colors::TRANSPARENT {
                    self.write_pixel(start_x + x, start_y + y, color);
                }
            }
        }
    }

    /// Dibuja un bloque de píxeles RGB888 crudos.
    #[allow(dead_code)]
    pub fn blit_rgb888(
        &mut self,
        x0: usize,
        y0: usize,
        w: usize,
        h: usize,
        data: &[u8], // len = w*h*3
    ) {
        let stride = w * 3;
        for row in 0..h {
            let off = row * stride;
            let line = &data[off..off + stride];
            for col in 0..w {
                let i = col * 3;
                let r = line[i];
                let g = line[i + 1];
                let b = line[i + 2];

                // Construimos 0x00RRGGBB
                let color_u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                self.write_pixel(x0 + col, y0 + row, color_u32);
            }
        }
    }

    /// Dibuja una imagen desde un buffer de bytes en formato BMP.
    #[allow(dead_code)]
    pub fn blit_bmp(&mut self, x0: i32, y0: i32, bmp_bytes: &[u8]) {
        if let Ok(bmp) = Bmp::from_slice(bmp_bytes) {
            let img = Image::new(&bmp, Point::new(x0, y0));
            let _ = img.draw(self);
        }
    }
}

/// Implementación del trait `core::fmt::Write` para `FramebufferWriter`.
/// Esto permite usar macros de formato como `write!`, `writeln!`, etc.
impl fmt::Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

/// Implementación de `DrawTarget` para que `FramebufferWriter` sea compatible con `embedded-graphics`.
impl DrawTarget for FramebufferWriter {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // `coord` es un `Point` con coordenadas i32. Las convertimos a usize.
            // Hacemos una comprobación para evitar un pánico si las coordenadas son negativas.
            if coord.x >= 0 && coord.y >= 0 {
                let x = coord.x as usize;
                let y = coord.y as usize;
                let color_u32 = (color.r() as u32) << 16 | (color.g() as u32) << 8 | color.b() as u32;
                self.write_pixel(x, y, color_u32);
            }
        }
        Ok(())
    }
}

/// Implementación de `OriginDimensions` para definir el tamaño del área de dibujo.
impl OriginDimensions for FramebufferWriter {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}