.PHONY: all clean run

KERNEL := target/x86_64-unknown-none/release/ai-os-kernel

all: $(KERNEL)
	@echo "Build complete: $(KERNEL)"

$(KERNEL):
	@cargo +nightly build --release --target x86_64-unknown-none

run: $(KERNEL)
	@echo "Running AI-OS in QEMU..."
	@qemu-system-x86_64 -kernel $(KERNEL) -nographic -m 256

clean:
	rm -rf target
