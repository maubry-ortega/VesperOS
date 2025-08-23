//! Define los comandos y el parser para la shell de VesperOS.

use heapless::String;

/// Representa un comando que puede ser ejecutado por la shell.
///
/// Se utiliza `heapless::String` para evitar alocaciones de memoria en el heap,
/// lo cual es crucial en un entorno de kernel.
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Limpia la pantalla.
    Clear,
    /// Imprime los argumentos de vuelta en la pantalla.
    Echo(String<256>),
    /// Muestra la pantalla de VesperFetch.
    VesperFetch,
    /// Muestra la pantalla de información del sistema (alias de vesperfetch).
    Info,
    /// Muestra información de ayuda.
    Help,
    /// Comando no reconocido.
    Unknown(String<32>),
    /// Entrada vacía.
    None,
}

/// Parsea un string en un `Command`.
///
/// Esta función es simple y solo divide la entrada por el primer espacio.
/// No maneja argumentos entre comillas ni sintaxis compleja.
pub fn parse(input: &str) -> Command {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Command::None;
    }

    // `split_once` es una forma robusta de separar el comando de los argumentos.
    // Devuelve una tupla con el string antes y después del primer espacio.
    let (command, args_str) = trimmed.split_once(char::is_whitespace).unwrap_or((trimmed, ""));

    // Usamos `eq_ignore_ascii_case` para una comparación sin mayúsculas/minúsculas
    // que no necesita alocar memoria.
    if command.eq_ignore_ascii_case("clear") {
        Command::Clear
    } else if command.eq_ignore_ascii_case("echo") {
        // Creamos el string para los argumentos de forma segura.
        let mut s = String::new();
        // push_str puede fallar si el string es demasiado largo, pero lo ignoramos con .ok()
        // para que simplemente se trunque, que es un comportamiento aceptable aquí.
        let _ = s.push_str(args_str); // El `_` indica que ignoramos el Result deliberadamente.
        Command::Echo(s)
    } else if command.eq_ignore_ascii_case("vesperfetch") {
        Command::VesperFetch
    } else if command.eq_ignore_ascii_case("info") {
        Command::Info
    } else if command.eq_ignore_ascii_case("help") {
        Command::Help
    } else {
        let mut s = String::new();
        // Ignoramos el resultado, truncando el nombre del comando si es demasiado largo.
        let _ = s.push_str(command);
        Command::Unknown(s)
    }
}