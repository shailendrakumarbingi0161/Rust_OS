// src/idt.rs
//! Interrupt Descriptor Table (IDT) for 32-bit x86.
//! Provides safe catch-all handlers so unexpected interrupts/faults do not triple-fault.

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_high: u16,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            zero: 0,
            type_attr: 0,
            offset_high: 0,
        }
    }

    pub fn set_handler(&mut self, handler: usize) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.selector = 0x08; // GDT Kernel Code Segment (0x08)
        self.zero = 0;
        self.type_attr = 0x8E; // Present, DPL 0, 32-bit Interrupt Gate
        self.offset_high = ((handler >> 16) & 0xFFFF) as u16;
    }
}

#[repr(C, packed)]
pub struct IdtPointer {
    limit: u16,
    base: u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];

#[naked]
unsafe extern "C" fn default_handler() {
    core::arch::asm!(
        "iretd",
        options(noreturn)
    );
}

#[naked]
unsafe extern "C" fn errcode_handler() {
    core::arch::asm!(
        "add esp, 4",
        "iretd",
        options(noreturn)
    );
}

pub unsafe fn init() {
    let def_h = default_handler as usize;
    let err_h = errcode_handler as usize;

    for i in 0..256 {
        // x86 exceptions that push an error code: 8, 10, 11, 12, 13, 14, 17, 21, 29, 30
        let is_err = matches!(i, 8 | 10 | 11 | 12 | 13 | 14 | 17 | 21 | 29 | 30);
        if is_err {
            IDT[i].set_handler(err_h);
        } else {
            IDT[i].set_handler(def_h);
        }
    }

    let ptr = IdtPointer {
        limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        base: IDT.as_ptr() as u32,
    };

    core::arch::asm!(
        "lidt [{0}]",
        in(reg) &ptr,
        options(readonly, nostack, preserves_flags)
    );
}
