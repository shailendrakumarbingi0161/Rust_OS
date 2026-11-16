// src/exceptions.rs
//! Simple exception handlers that print the exception number on VGA.
//! In a real OS you would handle each case properly; here we just display a message and halt.

use crate::vga::println;

macro_rules! exc_handler {
    ($num:expr, $name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            println!("[Exception] {} (vector {})", stringify!($name), $num);
            // Halt the CPU after reporting
            loop { unsafe { core::arch::asm!("hlt"); } }
        }
    };
}

exc_handler!(0, isr0_handler);
exc_handler!(1, isr1_handler);
exc_handler!(2, isr2_handler);
exc_handler!(3, isr3_handler);
exc_handler!(4, isr4_handler);
exc_handler!(5, isr5_handler);
exc_handler!(6, isr6_handler);
exc_handler!(7, isr7_handler);
exc_handler!(8, isr8_handler);
exc_handler!(9, isr9_handler);
exc_handler!(10, isr10_handler);
exc_handler!(11, isr11_handler);
exc_handler!(12, isr12_handler);
exc_handler!(13, isr13_handler);
exc_handler!(14, isr14_handler);
exc_handler!(15, isr15_handler);
exc_handler!(16, isr16_handler);
exc_handler!(17, isr17_handler);
exc_handler!(18, isr18_handler);
exc_handler!(19, isr19_handler);
exc_handler!(20, isr20_handler);
exc_handler!(21, isr21_handler);
exc_handler!(22, isr22_handler);
exc_handler!(23, isr23_handler);
exc_handler!(24, isr24_handler);
exc_handler!(25, isr25_handler);
exc_handler!(26, isr26_handler);
exc_handler!(27, isr27_handler);
exc_handler!(28, isr28_handler);
exc_handler!(29, isr29_handler);
exc_handler!(30, isr30_handler);
exc_handler!(31, isr31_handler);
