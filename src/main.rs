//! Kernel entry point - uses VGA, serial, and hardware drivers.
#![no_std]
#![no_main]
#![feature(naked_functions)]

#[macro_use]
extern crate alloc;

#[macro_use]
mod vga;
mod serial;
mod gdt;
mod idt;
mod pic;
mod pit;
mod keyboard;
mod rtc;
mod fs;
mod shell;
mod menu;
mod snake;
mod snakes_and_ladders;
mod car_dodge;
mod info;
mod deadlock_lab;
mod net;
mod hw;
mod allocator;

use core::panic::PanicInfo;
use core::fmt::Write;

// Include assembly multiboot header and _start entry point
core::arch::global_asm!(include_str!("boot.S"));

// Entry point called from boot.S
#[no_mangle]
pub extern "C" fn rust_main() {
    // Initialise screen
    vga::clear_screen();
    // Write welcome message via VGA and serial (serial log used for VirtualBox boot test)
    println!("Hello, RustOS!");
    serial::write_str("Hello, RustOS!\n");
    serial::write_str("RustOS booted\n");

    // Initialise hardware
    unsafe {
        core::arch::asm!("cli");
        serial::write_str("Before GDT init\n");
        gdt::init();
        serial::write_str("After GDT init\n");
        pic::remap(0x20, 0x28);
        idt::init();
        serial::write_str("After IDT init\n");
        pit::init();
        keyboard::init();
    }

    // Show current date-time
    rtc::print_datetime();
    // Print hardware / NIC info
    info::print_hardware_info();

    // Initialise in-memory file system
    fs::init();

    serial::write_str("Entering Desktop UI\n");

    // Launch Graphical Desktop Environment
    menu::run();

    // Drop into interactive shell if desktop exits
    shell::run();

    // If shell ever returns, halt CPU
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

// Panic handler prints in red on VGA and logs to serial
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::clear_screen();
    let mut writer = vga::Writer::new(0x04);
    let _ = write!(writer, "{}", info);
    serial::write_str("Kernel Panic: ");
    if let Some(loc) = info.location() {
        let _ = write!(serial::SerialWriter, "at {}:{} ", loc.file(), loc.line());
    }
    serial::write_str("\n");
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
