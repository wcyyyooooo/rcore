# Makefile
# Rust target triple  
TARGET = riscv64gc-unknown-none-elf
# Build directory  
BUILD_DIR = target/$(TARGET)/release
# Binary output  
BINARY = $(BUILD_DIR)/os
BINARY_STRIPPED = $(BUILD_DIR)/os.bin
# QEMU and rust-objcopy commands  
QEMU = qemu-system-riscv64
RUST_OBJCOPY = rust-objcopy
# Default target  
all: build run
# Build the project  
build:
	cargo build --release --target $(TARGET)
	$(RUST_OBJCOPY) --strip-all $(BINARY) -O binary $(BINARY_STRIPPED)
# Run the project in QEMU  
run:  
	@echo "BINARY_STRIPPED: $(BINARY_STRIPPED)"  
	$(QEMU) -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=$(BINARY_STRIPPED),addr=0x80200000
# Clean up build artifacts  
clean:  
	cargo clean  
	rm -f $(BINARY_STRIPPED)  
  
.PHONY: all build run clean