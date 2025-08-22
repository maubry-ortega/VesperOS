//! Módulo VesperFetch - Información del sistema al estilo Neofetch pero propio

use core::fmt::Write;
use crate::vga::FramebufferWriter;
use crate::branding;
use crate::colors;

/// Información del sistema para mostrar en VesperFetch
#[derive(Clone, Copy)]
pub struct SystemInfo {
    pub os_name: &'static str,
    pub os_version: &'static str,
    pub kernel_version: &'static str,
    pub cpu_info: &'static str,
    pub uptime: &'static str,
    pub memory_total_mb: u64,
    pub resolution_width: u64,
    pub resolution_height: u64,
}

/// Estilo y tema de VesperFetch
pub struct VesperFetchTheme {
    pub label_color: colors::Color,
    pub value_color: colors::Color,
    pub accent_color: colors::Color,
    pub border_color: colors::Color,
}

impl Default for VesperFetchTheme {
    fn default() -> Self {
        Self {
            label_color: colors::BRIGHT_VIOLET,
            value_color: colors::SMOKE_WHITE,
            accent_color: colors::NEON_GREEN,
            border_color: colors::COSMIC_BLUE,
        }
    }
}

/// Renderizador de VesperFetch
pub struct VesperFetch {
    theme: VesperFetchTheme,
    system_info: SystemInfo,
}

impl VesperFetch {
    /// Crear nueva instancia de VesperFetch
    pub fn new(system_info: SystemInfo) -> Self {
        Self {
            theme: VesperFetchTheme::default(),
            system_info,
        }
    }

    /// Cambiar tema
    #[allow(dead_code)]
    pub fn with_theme(mut self, theme: VesperFetchTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Mostrar VesperFetch en el framebuffer
    pub fn display(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        // La ventana de VesperFetch es de aprox. 300x200 píxeles.
        self.draw_logo(writer, x + 20, y + 20);
        self.draw_info(writer, x + 140, y + 25);
    }

    /*
    /// Dibujar borde decorativo
    fn draw_border(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        writer.set_text_color(self.theme.border_color);
        
        // Esquinas
        writer.set_cursor_position(x, y);
        write!(writer, "╔").unwrap();
        writer.set_cursor_position(x + 299, y);
        write!(writer, "╗").unwrap();
        writer.set_cursor_position(x, y + 199);
        write!(writer, "╚").unwrap();
        writer.set_cursor_position(x + 299, y + 199);
        write!(writer, "╝").unwrap();

        // Bordes horizontales
        for i in 1..299 {
            writer.set_cursor_position(x + i, y);
            write!(writer, "═").unwrap();
            writer.set_cursor_position(x + i, y + 199);
            write!(writer, "═").unwrap();
        }

        // Bordes verticales
        for i in 1..199 {
            writer.set_cursor_position(x, y + i);
            write!(writer, "║").unwrap();
            writer.set_cursor_position(x + 299, y + i);
            write!(writer, "║").unwrap();
        }
    }
    */

    /// Dibuja el logo de VesperOS usando la imagen del módulo `branding`.
    fn draw_logo(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        // El área del logo es de aprox. 120px de ancho. Centramos la imagen de 32x32.
        let logo_x = x + (120 - branding::VESPER_LOGO_IMAGE.width) / 2;
        writer.blit_raw_image(&branding::VESPER_LOGO_IMAGE, logo_x, y);
    }

    /// Dibujar información del sistema
    fn draw_info(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        let mut current_y = y;
        const LINE_HEIGHT: usize = 20;
        const VALUE_OFFSET: usize = 100;

        // OS + Version
        writer.set_color(self.theme.accent_color); // Usamos accent_color para el nombre del OS
        writer.set_cursor_position(x, current_y);
        write!(writer, "{}", self.system_info.os_name).unwrap();
        writer.set_color(self.theme.value_color); // Volvemos a value_color para la versión
        write!(writer, "@{}", self.system_info.os_version).unwrap();
        current_y += LINE_HEIGHT;

        // Separador
        writer.set_color(self.theme.border_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "------------------").unwrap();
        current_y += LINE_HEIGHT;

        // Kernel
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "Kernel:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{}", self.system_info.kernel_version).unwrap();
        current_y += LINE_HEIGHT;

        // CPU
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "CPU:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{}", self.system_info.cpu_info).unwrap();
        current_y += LINE_HEIGHT;

        // Uptime
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "Uptime:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{}", self.system_info.uptime).unwrap();
        current_y += LINE_HEIGHT;

        // Memory
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "Memory:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{} MB", self.system_info.memory_total_mb).unwrap();
        current_y += LINE_HEIGHT;

        // Resolution
        writer.set_color(self.theme.label_color);
        writer.set_cursor_position(x, current_y);
        write!(writer, "Resolution:").unwrap();
        writer.set_color(self.theme.value_color);
        writer.set_cursor_position(x + VALUE_OFFSET, current_y);
        write!(writer, "{}x{}", self.system_info.resolution_width, self.system_info.resolution_height).unwrap();
    }

    /// Versión minimalista para pantallas pequeñas
    #[allow(dead_code)]
    pub fn display_minimal(&self, writer: &mut FramebufferWriter, x: usize, y: usize) {
        // Por ahora, simplemente llamamos a la versión principal.
        // Se puede personalizar más adelante si es necesario.
        self.display(writer, x, y);
    }
}

/// Versión animada
#[allow(dead_code)]
pub fn show_animated_fetch(writer: &mut FramebufferWriter, system_info: SystemInfo, x: usize, y: usize) {
    // Animación simple de aparición
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
        
        // Pequeña pausa para la animación
        for _ in 0..10000 { unsafe { core::arch::asm!("nop"); } }
    }
}

/// Mezclar colores para animaciones
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