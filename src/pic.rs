// src/pic.rs
//! Programmable Interrupt Controller (8259 PIC) initialization and masking.

use crate::vga::outb;

pub fn remap(offset1: u8, offset2: u8) {
    unsafe {
        // ICW1: Start initialization sequence in cascade mode
        outb(0x20, 0x11);
        outb(0xA0, 0x11);

        // ICW2: Master PIC vector offset
        outb(0x21, offset1);
        // ICW2: Slave PIC vector offset
        outb(0xA1, offset2);

        // ICW3: Tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
        outb(0x21, 0x04);
        // ICW3: Tell Slave PIC its cascade identity (0000 0010)
        outb(0xA1, 0x02);

        // ICW4: 8086/88 mode
        outb(0x21, 0x01);
        outb(0xA1, 0x01);

        // Mask all hardware IRQs on Master (0x21) and Slave (0xA1)
        outb(0x21, 0xFF);
        outb(0xA1, 0xFF);
    }
}
