//! La lógica de la aplicación VesperFetch.

use crate::colors;
use crate::MEMMAP_REQUEST;
use crate::vesperfetch;
use crate::vga::FramebufferWriter;

/// Ejecuta la aplicación VesperFetch.
///
/// Esta función borra la pantalla, recopila información del sistema y la
/// muestra utilizando el módulo VesperFetch.
pub fn run(writer: &mut FramebufferWriter) {
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
        resolution_width: writer.width() as u64,
        resolution_height: writer.height() as u64,
    };

    // Dibuja VesperFetch centrado
    let fetch_x = (writer.width() - 400) / 2;
    let fetch_y = (writer.height() - 200) / 2;

    let fetch = vesperfetch::VesperFetch::new(sys_info);
    fetch.display(writer, fetch_x, fetch_y);
}