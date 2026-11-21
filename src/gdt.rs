// src/gdt.rs
//! Minimal Global Descriptor Table for a 32-bit protected-mode kernel.

use core::arch::asm;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct Descriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

const fn descriptor(base: u32, limit: u32, access: u8, gran: u8) -> Descriptor {
    Descriptor {
        limit_low: (limit & 0xFFFF) as u16,
        base_low: (base & 0xFFFF) as u16,
        base_mid: ((base >> 16) & 0xFF) as u8,
        access,
        granularity: ((limit >> 16) & 0x0F) as u8 | (gran & 0xF0),
        base_high: ((base >> 24) & 0xFF) as u8,
    }
}

// GDT entries: null, code (0x9A), data (0x92).
static GDT: [Descriptor; 3] = [
    // Null descriptor
    descriptor(0, 0, 0, 0),
    // Code segment: base=0, limit=4GB, access=0x9A, gran=0xCF (4KB gran, 32-bit)
    descriptor(0, 0xFFFFF, 0x9A, 0xCF),
    // Data segment: access=0x92
    descriptor(0, 0xFFFFF, 0x92, 0xCF),
];

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u32,
}

/// Load the GDT and set up segment registers.
pub unsafe fn init() {
    let ptr = GdtPtr {
        limit: (core::mem::size_of_val(&GDT) - 1) as u16,
        base: &GDT as *const _ as u32,
    };
    asm!(
        "lgdt [{0}]",
        "mov ax, 0x10", // data selector (index 2 << 3)
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        "push 0x08",
        "lea eax, [2f]",
        "push eax",
        "retf",
        "2:",
        in(reg) &ptr,
        out("eax") _,
        options(preserves_flags)
    );
}
