//! Módulo VesperFetch: una utilidad para mostrar información del sistema.
//!
//! Inspirado en herramientas como Neofetch, VesperFetch presenta datos clave
//! del sistema operativo junto al logo de VesperOS de una forma estilizada.

use core::fmt::Write;
use crate::vga::FramebufferWriter;
use crate::branding;
use crate::colors;

/// Contiene la información del sistema que se mostrará en VesperFetch.
#[derive(Clone, Copy)]
pub struct SystemInfo {
    /// Nombre del sistema operativo.
    pub os_name: &'static str,
    /// Versión del sistema operativo.
    pub os_version: &'static str,
    /// Versión del kernel.
    pub kernel_version: &'static str,
    /// Información de la CPU (actualmente un placeholder).
    pub cpu_info: &'static str,
    /// Tiempo de actividad del sistema (actualmente un placeholder).
    pub uptime: &'static str,
    /// Memoria total usable en Megabytes.
    pub memory_total_mb: u64,
    /// Ancho de la resolución de pantalla en píxeles.
    pub resolution_width: u64,
    /// Alto de la resolución de pantalla en píxeles.
    pub resolution_height: u64,
}

/// Define la paleta de colores para la interfaz de VesperFetch.
pub struct VesperFetchTheme {
    /// Color para las etiquetas (ej: "Kernel:").
    pub label_color: colors::Color,
    /// Color para los valores (ej: "Vesper-Core").
    pub value_color: colors::Color,
    /// Color de acento para el nombre del SO.
    pub accent_color: colors::Color,
    /// Color para bordes o separadores.
    pub border_color: colors::Color,
}

impl Default for VesperFetchTheme {
    /// Proporciona el tema de colores por defecto para VesperFetch.
    fn default() -> Self {
        Self {
            label_color: colors::BRIGHT_VIOLET,
            value_color: colors::SMOKE_WHITE,
            accent_color: colors::NEON_GREEN,
            border_color: colors::COSMIC_BLUE,
        }
    }
}

/// El renderizador principal de VesperFetch.
///
/// Esta estructura toma la información del sistema y un tema, y se encarga
/// de dibujarlos en el framebuffer.
pub struct VesperFetch {
    theme: VesperFetchTheme,
    system_info: SystemInfo,
}

impl VesperFetch {
    /// Crea una nueva instancia de `VesperFetch` con la información del sistema dada.
    pub fn new(system_info: SystemInfo) -> Self {
        Self {
            theme: VesperFetchTheme::default(),
            system_info,
        }
    }

    /// Permite personalizar el tema de colores de VesperFetch.
    #[allow(dead_code)]
    pub fn with_theme(mut self, theme: VesperFetchTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Dibuja la interfaz de VesperFetch en el framebuffer en la posición especificada.
    ///
    /// # Arguments
    ///
    /// * `writer`: Una referencia mutable al `FramebufferWriter` donde se dibujará.
    /// * `x`, `y`: Las coordenadas de la esquina superior izquierda del área de VesperFetch.
    pub fn display(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        // El diseño se divide en dos columnas: el logo a la izquierda y la info a la derecha.
        self.draw_logo(writer, x + 20, y + 20);
        self.draw_info(writer, x + 140, y + 25);
    }

    /// Dibuja el logo de VesperOS en el área designada.
    fn draw_logo(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        // Centra la imagen del logo dentro del espacio asignado.
        let logo_x = x + (120 - branding::VESPER_LOGO_IMAGE.width) / 2;
        writer.blit_raw_image(&branding::VESPER_LOGO_IMAGE, logo_x, y);
    }

    /// Dibuja la lista de información del sistema.
    fn draw_info(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        let mut current_y = y;
        const LINE_HEIGHT: usize = 20;
        const VALUE_OFFSET: usize = 100; // Desplazamiento horizontal para los valores.

        // OS + Version
        writer.set_color(self.theme.accent_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "{}", self.system_info.os_name).unwrap();
        writer.set_color(self.theme.value_color);
        write!(writer, "@{}", self.system_info.os_version).unwrap();
        current_y += LINE_HEIGHT;

        // Separador
        writer.set_color(self.theme.border_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "------------------").unwrap();
        current_y += LINE_HEIGHT;

        // Dibuja cada par etiqueta-valor.
        self.draw_info_line(writer, x, &mut current_y, LINE_HEIGHT, VALUE_OFFSET, "Kernel:", self.system_info.kernel_version);
        self.draw_info_line(writer, x, &mut current_y, LINE_HEIGHT, VALUE_OFFSET, "CPU:", self.system_info.cpu_info);
        self.draw_info_line(writer, x, &mut current_y, LINE_HEIGHT, VALUE_OFFSET, "Uptime:", self.system_info.uptime);

        // Memory (con formato)
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "Memory:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{} MB", self.system_info.memory_total_mb).unwrap();
        current_y += LINE_HEIGHT;

        // Resolution (con formato)
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "Resolution:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{}x{}", self.system_info.resolution_width, self.system_info.resolution_height).unwrap();
    }

    /// Función de ayuda para dibujar una línea de información (etiqueta y valor).
    fn draw_info_line(&self, writer: &mut FramebufferWriter, x: usize, y: &mut usize, line_height: usize, value_offset: usize, label: &str, value: &str) {
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, *y);
        write!(writer, "{}", label).unwrap();

        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + value_offset, *y);
        write!(writer, "{}", value).unwrap();

        *y += line_height;
    }

    /// Muestra una versión minimalista de VesperFetch (actualmente igual a la principal).
    #[allow(dead_code)]
    pub fn display_minimal(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        self.display(writer, x, y);
    }
}

/// Muestra VesperFetch con una animación de aparición gradual.
///
/// **Nota:** Esta función está marcada con `dead_code` porque actualmente no se usa
/// en el flujo de arranque principal para evitar parpadeos y complejidad.
#[allow(dead_code)]
pub fn show_animated_fetch(writer: &mut FramebufferWriter, system_info: SystemInfo, x: usize, y: usize) {
    // Animación simple de aparición (fade-in).
    for i in 0..=100 {
        writer.clear(colors::DEEP_BLACK);
        
        let alpha = i as f32 / 100.0;
        let theme = VesperFetchTheme {
            label_color: blend_color(colors::DEEP_BLACK, colors::BRIGHT_VIOLET, alpha),
            value_color: blend_color(colors::DEEP_BLACK, colors::SMOKE_WHITE, alpha),
            accent_color: blend_color(colors::DEEP_BLACK, colors::NEON_GREEN, alpha),
            border_color: blend_color(colors::DEEP_BLACK, colors::COSMIC_BLUE, alpha),
        };
        
        let vesper_fetch = VesperFetch::new(system_info).with_theme(theme);
        vesper_fetch.display(writer, x, y);
        
        // Pequeña pausa para controlar la velocidad de la animación.
        for _ in 0..10000 { unsafe { core::arch::asm!("nop"); } }
    }
}

/// Mezcla linealmente dos colores.
///
/// `alpha` es el factor de mezcla: 0.0 da `color1`, 1.0 da `color2`.
///
/// **Nota:** Esta función está marcada con `dead_code` porque solo la usa
/// `show_animated_fetch`, que tampoco se utiliza.
#[allow(dead_code)]
fn blend_color(color1: colors::Color, color2: colors::Color, alpha: f32) -> colors::Color {
    let r1 = ((color1 >> 16) & 0xFF) as f32;
    let g1 = ((color1 >> 8) & 0xFF) as f32;
    let b1 = (color1 & 0xFF) as f32;
    
    let r2 = ((color2 >> 16) & 0xFF) as f32;
    let g2 = ((color2 >> 8) & 0xFF) as f32;
    let b2 = (color2 & 0xFF) as f32;
    
    let r = (r1 + (r2 - r1) * alpha) as u32;
    let g = (g1 + (g2 - g1) * alpha) as u32;
    let b = (b1 + (b2 - b1) * alpha) as u32;
    
    (r << 16) | (g << 8) | b
}