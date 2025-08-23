# 🌌 Vesper OS

<img src="assets/vesper_pet_Nox.png" alt="Nox, el búho minimalista de Vesper OS" style="max-width: 200px; border: 3px solid #A259FF; border-radius: 10px; margin: 20px auto;" />

**Vesper OS** es un sistema operativo experimental escrito en **Rust**, diseñado para ser **portable, ligero y persistente**, con la capacidad de ejecutar aplicaciones en formato **WebAssembly (.wasm)**.

---

## 🚀 Concepto Principal

- 🦉 Crear un **sistema operativo personalizado** en Rust.
- 🦉 Soporte para **aplicaciones WebAssembly** ejecutadas como programas nativos.
- 🦉 **Portabilidad en USB** con persistencia de datos (inspirado en NomadBSD, pero más ligero).
- 🦉 **Identidad propia** centrada en portabilidad y aprendizaje.
s
---

## 🎨 Identidad Visual

- **Nombre**: <i>Vesper</i> (estrella vespertina, elegancia nocturna).
- **Paleta de Colores**:
    - Negro Profundo (`#0B0B0D`)
    - Morado Oscuro (`#2C1A47`)
    - Azul Cósmico (`#1E2A78`)
    - Violeta Brillante (`#A259FF`)
    - Blanco Humo (`#EAEAEA`)
    - Verde Neón (`#3DFFB4`)

- **Mascota / Símbolo**:  
  - Búho minimalista 🦉, representando sabiduría, vigilancia nocturna y una identidad fuerte.

- **Logo Conceptual**:
  - **Tipografía**: Moderna y geométrica (<i>Orbitron</i> o <i>Montserrat</i>).
  - **Símbolo**: Búho o estrella vespertina estilizada en diseño plano.
  - **Estilo**: Versátil para CLI (arte ASCII) e interfaces gráficas.

---

## ✅ Progreso Actual

- 🦉 Configuración mínima con **Rust estable** (sin dependencia de nightly).
- 🦉 **Multiboot2** válido para arranque con GRUB2.
- 🦉 Ejemplo funcional que imprime `Hola` en modo texto VGA.
- 🦉 **Makefile** para:
    - Compilar el kernel.
    - Crear ISO con GRUB2.
    - Ejecutar en QEMU.

---

## 📚 Documentación

El código fuente está completamente documentado siguiendo las convenciones de `rustdoc`. Para generar y ver la documentación localmente, ejecuta:

```bash
cargo doc --open
```

Además, puedes encontrar documentación de alto nivel sobre la arquitectura y guías de compilación en el directorio `docs/`.
---

## 🔧 Pendiente por Implementar

- 🦉 Limpieza de pantalla y control avanzado de salida VGA.
- 🦉 Manejo de interrupciones (IDT/GDT en Rust).
- 🦉 Controlador básico de teclado.
- 🦉 Sistema de archivos persistente en USB.
- 🦉 Soporte para ejecutar archivos `.wasm`.
- 🦉 Sistema de empaquetado y gestión de aplicaciones WebAssembly.
- 🦉 Logo y branding en ASCII/CLI y gráficos.

---

## 🎯 Próximos Pasos

1. Mejorar salida en pantalla (drivers VGA, framebuffer).
2. Implementar IDT y manejo de interrupciones.
3. Agregar soporte para entrada de teclado.
4. Diseñar el logo inicial en SVG.
5. Probar persistencia en USB (arranque real).
6. Integrar soporte básico para `.wasm`.

---

## ✍️ Meta Personal

> Aprender **Rust** a profundidad, dominando tanto el desarrollo de bajo nivel (OSDev) como el de alto nivel (ejecución de WebAssembly), para crear un **sistema operativo único** con una identidad propia y sólida.