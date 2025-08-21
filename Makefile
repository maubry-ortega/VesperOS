TARGET = x86_64-unknown-none
KERNEL = target/$(TARGET)/debug/kernel_minimo
ISO_DIR = iso
ISO = kernel.iso

all: $(ISO)

$(KERNEL):
	cargo build --target $(TARGET)

$(ISO): $(KERNEL)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/kernel_minimo
	echo 'set timeout=0' > $(ISO_DIR)/boot/grub/grub.cfg
	echo 'set default=0' >> $(ISO_DIR)/boot/grub/grub.cfg
	echo 'menuentry "Kernel Minimo" {' >> $(ISO_DIR)/boot/grub/grub.cfg
	echo '    multiboot2 /boot/kernel_minimo' >> $(ISO_DIR)/boot/grub/grub.cfg
	echo '    boot' >> $(ISO_DIR)/boot/grub/grub.cfg
	echo '}' >> $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) $(ISO_DIR)

run: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO)

clean:
	cargo clean
	rm -rf $(ISO_DIR) $(ISO)
