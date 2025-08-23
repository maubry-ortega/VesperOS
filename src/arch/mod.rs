//! Módulo de abstracción de arquitectura.
//!
//! Este módulo proporciona una interfaz genérica para las funciones que dependen
//! del hardware, como el manejo de interrupciones o el control de la CPU. El
//! objetivo es aislar el código específico de la arquitectura del resto del kernel.

// Por ahora, solo soportamos x86_64. Este es el lugar donde se añadirían
// otras arquitecturas (como aarch64) en el futuro.
#[cfg(target_arch = "x86_64")]
pub mod x86_64; // Este módulo se compilará solo si el target es x86_64.

#[cfg(target_arch = "x86_64")]
/// Re-exporta el módulo de la arquitectura actual como `target`.
pub use self::x86_64 as target;

/// Inicializa las características específicas de la arquitectura (ej. interrupciones).
pub fn init() {
    target::init();
}

/// Pone la CPU en un estado de bajo consumo hasta la próxima interrupción.
pub fn wait_for_interrupt() {
    target::wait_for_interrupt();
}