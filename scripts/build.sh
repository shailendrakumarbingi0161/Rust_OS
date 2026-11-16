#!/usr/bin/env bash
set -euo pipefail

# ----------------------------------------------------------------------
# Working directory is the repository root (mounted as /src in Docker)
# ----------------------------------------------------------------------
cd /src

# Use the custom linker script (load address 1M)
export RUSTFLAGS="-C link-arg=-Tsrc/linker.ld"

# ----------------------------------------------------------------------
# Build the kernel using the custom target specification
# ----------------------------------------------------------------------
cargo +nightly-2024-09-01 build --release -Zbuild-std=core,alloc --target i686-unknown-none.json

TARGET_DIR="target/i686-unknown-none/release"

# ----------------------------------------------------------------------
# Package the resulting ELF into a GRUB ISO (multiboot2)
# ----------------------------------------------------------------------
mkdir -p iso/boot/grub

# Copy the ELF to the location GRUB expects
if [ -f "$TARGET_DIR/rustos" ]; then
    cp "$TARGET_DIR/rustos" iso/boot/rustos.elf
elif [ -f "$TARGET_DIR/rustos.elf" ]; then
    cp "$TARGET_DIR/rustos.elf" iso/boot/rustos.elf
else
    echo "Error: kernel binary not found in $TARGET_DIR"
    exit 1
fi

# Minimal GRUB configuration – use multiboot2 (required for our header)
cat > iso/boot/grub/grub.cfg <<'EOF'
set timeout=3
set default=0

menuentry "RustOS" {
    multiboot2 /boot/rustos.elf
    boot
}
EOF

mkdir -p build
# Create the ISO using grub-mkrescue (fallback to grub2-mkrescue if needed)
if command -v grub-mkrescue > /dev/null 2>&1; then
    grub-mkrescue -o build/rustos.iso iso
elif command -v grub2-mkrescue > /dev/null 2>&1; then
    grub2-mkrescue -o build/rustos.iso iso
else
    echo "Error: grub-mkrescue not found in container"
    exit 1
fi

echo "✅ ISO built successfully at build/rustos.iso"
