# Makefile para VesperOS

# --- Configuración ---

# Versión de Limine compatible con la librería de Rust limine v0.5.0
LIMINE_BRANCH = v7.x-binary

# Herramientas
CARGO = cargo
QEMU = qemu-system-x86_64
MKDIR = mkdir -p
RM = rm -rf
CP = cp
GIT = git
MAKE = make

# Estructura del proyecto
TARGET_DIR = ./target
ISO_DIR = ./isodir
ISO_FILE = ./VesperOS.iso
LIMINE_DIR = ./limine

# Kernel
KERNEL_TARGET = x86_64-unknown-none
KERNEL_BINARY = $(TARGET_DIR)/$(KERNEL_TARGET)/debug/VesperOS

# --- Reglas ---

.PHONY: all build clean iso run limine

all: build

build:
	@echo ">>> Compilando el kernel VesperOS..."
	@$(CARGO) build --target=$(KERNEL_TARGET)

clean:
	@echo ">>> Limpiando artefactos de compilación..."
	@$(RM) -r $(TARGET_DIR) $(ISO_DIR) $(ISO_FILE) $(LIMINE_DIR)

iso: build limine
	@echo ">>> Creando la imagen ISO..."
	@$(MKDIR) $(ISO_DIR)/boot
	@$(CP) $(KERNEL_BINARY) $(ISO_DIR)/boot/VesperOS
	@$(CP) ./limine.cfg $(ISO_DIR)/boot/limine.cfg
	@$(CP) $(LIMINE_DIR)/limine-bios.sys $(ISO_DIR)/boot/
	@$(CP) $(LIMINE_DIR)/limine-bios-cd.bin $(ISO_DIR)/boot/limine-cd.bin
	@$(CP) $(LIMINE_DIR)/limine-uefi-cd.bin $(ISO_DIR)/boot/limine-cd-efi.bin
	@xorriso -as mkisofs -b boot/limine-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine-cd-efi.bin -efi-boot-part --efi-boot-image --protective-msdos-label $(ISO_DIR) -o $(ISO_FILE)
	@$(LIMINE_DIR)/limine bios-install $(ISO_FILE)

run: iso
	@echo ">>> Ejecutando VesperOS en QEMU..."
	@$(QEMU) -cdrom $(ISO_FILE)

limine:
	@if [ ! -d "$(LIMINE_DIR)" ]; then \
		echo ">>> Descargando el gestor de arranque Limine..."; \
		$(GIT) clone https://github.com/limine-bootloader/limine.git --branch=$(LIMINE_BRANCH) --depth=1 $(LIMINE_DIR); \
		$(MAKE) -C $(LIMINE_DIR); \
	fi