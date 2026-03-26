.PHONY: all clean iso

all: target/ai-os.elf target/ai-os.iso

target/ai-os.elf: src/arch/x86_64/boot.S src/arch/x86_64/linker.ld
	@mkdir -p target
	@echo "[1/4] Assembling boot sector..."
	@nasm -f elf64 src/arch/x86_64/boot.S -o target/boot.o
	@echo "[2/4] Compiling Rust kernel..."
	@cargo +nightly build --release --target x86_64-unknown-none
	@cp target/x86_64-unknown-none/release/ai-os-kernel target/kernel
	@echo "[3/4] Linking..."
	@ld -T src/arch/x86_64/linker.ld -o target/ai-os.elf target/boot.o target/kernel
	@echo "ELF binary: target/ai-os.elf"

target/ai-os.iso: target/ai-os.elf
	@echo "[4/4] Creating ISO..."
	@mkdir -p target/iso/boot/grub
	@cp target/ai-os.elf target/iso/boot/
	@echo 'set timeout=0' > target/iso/boot/grub/grub.cfg
	@echo 'set default=0' >> target/iso/boot/grub/grub.cfg
	@echo 'menuentry "AI-OS" { multiboot /boot/ai-os.elf; boot }' >> target/iso/boot/grub/grub.cfg
	@grub-mkrescue -o target/ai-os.iso target/iso 2>/dev/null || echo "grub-mkrescue not found, skipping ISO"
	@echo "ISO: target/ai-os.iso"

clean:
	rm -rf target

run: target/ai-os.elf
	qemu-system-x86_64 -kernel target/ai-os.elf -nographic

run-iso: target/ai-os.iso
	qemu-system-x86_64 -cdrom target/ai-os.iso -nographic
