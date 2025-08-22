from PIL import Image

# Asegúrate de que tu imagen sea pequeña (ej. 32x32 o 64x64)
img = Image.open("assets/vesper_pet_Nox_ansi.png").convert("RGBA")
pixels = list(img.getdata())

with open("src/logo_data.rs", "w") as f:
    f.write("[\n")
    for r, g, b, a in pixels:
        if a < 128:  # Si es mayormente transparente
            f.write("    colors::TRANSPARENT,\n")
        else:
            # Formato 0x00RRGGBB
            f.write(f"    0x00{r:02X}{g:02X}{b:02X},\n")
    f.write("];\n")

print("¡Archivo src/logo_data.rs generado!")
