# <span style="font-family: Orbitron, sans-serif; color: #A259FF; font-size: 2em;">🌌 Vesper OS</span>

<div style="text-align: center;">
    <img src="assets/vesper_pet_Nox.png" alt="Nox, el búho minimalista de Vesper OS" style="max-width: 200px; border: 3px solid #A259FF; border-radius: 10px; margin: 20px auto;" />
</div>

**Vesper OS** es un sistema operativo experimental escrito en **Rust**, diseñado para ser **portable, ligero y persistente**, con la capacidad de ejecutar aplicaciones en formato **WebAssembly (.wasm)**.

---

## <span style="color: #3DFFB4; font-family: Montserrat, sans-serif; border-bottom: 2px solid #A259FF;">🚀 Concepto Principal</span>

<ul style="list-style: none; padding-left: 0;">
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Crear un <b>sistema operativo personalizado</b> en Rust.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Soporte para <b>aplicaciones WebAssembly</b> ejecutadas como programas nativos.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> <b>Portabilidad en USB</b> con persistencia de datos (inspirado en NomadBSD, pero más ligero).</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> <b>Identidad propia</b> centrada en portabilidad y aprendizaje.</li>
</ul>

---

## <span style="color: #3DFFB4; font-family: Montserrat, sans-serif; border-bottom: 2px solid #A259FF;">🎨 Identidad Visual</span>

- **Nombre**: <i>Vesper</i> (estrella vespertina, elegancia nocturna).
- **Paleta de Colores**:
  <div style="display: flex; flex-wrap: wrap; gap: 10px;">
    <div style="background-color: #0B0B0D; color: #EAEAEA; padding: 5px 10px; border-radius: 5px;">Negro Profundo (#0B0B0D)</div>
    <div style="background-color: #2C1A47; color: #EAEAEA; padding: 5px 10px; border-radius: 5px;">Morado Oscuro (#2C1A47)</div>
    <div style="background-color: #1E2A78; color: #EAEAEA; padding: 5px 10px; border-radius: 5px;">Azul Cósmico (#1E2A78)</div>
    <div style="background-color: #A259FF; color: #0B0B0D; padding: 5px 10px; border-radius: 5px;">Violeta Brillante (#A259FF)</div>
    <div style="background-color: #EAEAEA; color: #0B0B0D; padding: 5px 10px; border-radius: 5px;">Blanco Humo (#EAEAEA)</div>
    <div style="background-color: #3DFFB4; color: #0B0B0D; padding: 5px 10px; border-radius: 5px;">Verde Neón (#3DFFB4)</div>
  </div>

- **Mascota / Símbolo**:  
  - Búho minimalista 🦉, representando sabiduría, vigilancia nocturna y una identidad fuerte.

- **Logo Conceptual**:
  - **Tipografía**: Moderna y geométrica (<i>Orbitron</i> o <i>Montserrat</i>).
  - **Símbolo**: Búho o estrella vespertina estilizada en diseño plano.
  - **Estilo**: Versátil para CLI (arte ASCII) e interfaces gráficas.

---

## <span style="color: #3DFFB4; font-family: Montserrat, sans-serif; border-bottom: 2px solid #A259FF;">✅ Progreso Actual</span>

<ul style="list-style: none; padding-left: 0;">
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Configuración mínima con <b>Rust estable</b> (sin dependencia de nightly).</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> <b>Multiboot2</b> válido para arranque con GRUB2.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Ejemplo funcional que imprime <code>Hola</code> en modo texto VGA.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> <b>Makefile</b> para:
        <ul style="list-style: disc; padding-left: 20px;">
            <li>Compilar el kernel.</li>
            <li>Crear ISO con GRUB2.</li>
            <li>Ejecutar en QEMU.</li>
        </ul>
    </li>
</ul>

---

## <span style="color: #3DFFB4; font-family: Montserrat, sans-serif; border-bottom: 2px solid #A259FF;">🔧 Pendiente por Implementar</span>

<ul style="list-style: none; padding-left: 0;">
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Limpieza de pantalla y control avanzado de salida VGA.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Manejo de interrupciones (IDT/GDT en Rust).</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Controlador básico de teclado.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Sistema de archivos persistente en USB.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Soporte para ejecutar archivos <code>.wasm</code>.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Sistema de empaquetado y gestión de aplicaciones WebAssembly.</li>
    <li style="position: relative; padding-left: 30px; margin-bottom: 10px;"><span style="position: absolute; left: 0; color: #A259FF;">🦉</span> Logo y branding en ASCII/CLI y gráficos.</li>
</ul>

---

## <span style="color: #3DFFB4; font-family: Montserrat, sans-serif; border-bottom: 2px solid #A259FF;">🎯 Próximos Pasos</span>

<ol style="padding-left: 20px;">
    <li style="margin-bottom: 10px;">Mejorar salida en pantalla (drivers VGA, framebuffer).</li>
    <li style="margin-bottom: 10px;">Implementar IDT y manejo de interrupciones.</li>
    <li style="margin-bottom: 10px;">Agregar soporte para entrada de teclado.</li>
    <li style="margin-bottom: 10px;">Diseñar el logo inicial en SVG.</li>
    <li style="margin-bottom: 10px;">Probar persistencia en USB (arranque real).</li>
    <li style="margin-bottom: 10px;">Integrar soporte básico para <code>.wasm</code>.</li>
</ol>

---

## <span style="color: #3DFFB4; font-family: Montserrat, sans-serif; border-bottom: 2px solid #A259FF;">✍️ Meta Personal</span>

<div style="background-color: #2C1A47; padding: 15px; border-radius: 10px; color: #EAEAEA;">
    Aprender <b>Rust</b> a profundidad, dominando tanto el desarrollo de bajo nivel (OSDev) como el de alto nivel (ejecución de WebAssembly), para crear un <b>sistema operativo único</b> con una identidad propia y sólida.
</div>