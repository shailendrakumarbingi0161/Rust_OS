// src/info.rs
// Simple hardware‑info helper – prints a few key values using the VGA console.

#![allow(dead_code)]

use core::str;

// Conditional import for CPUID depending on target architecture
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid as cpuid;
#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid as cpuid;

pub fn print_hardware_info() {
    // ----- CPU vendor string -----
    let cpuid0 = unsafe { cpuid(0) };
    let mut vendor = [0u8; 12];
    vendor[0..4].copy_from_slice(&cpuid0.ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&cpuid0.edx.to_le_bytes());
    vendor[8..12].copy_from_slice(&cpuid0.ecx.to_le_bytes());
    let vendor_str = str::from_utf8(&vendor).unwrap_or("??");
    println!("CPU Vendor: {}", vendor_str.trim());

    // ----- NIC detection (PCI scan) -----
    const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
    const PCI_CONFIG_DATA: u16 = 0xCFC;

    unsafe fn outl(port: u16, val: u32) {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") val, options(nostack, preserves_flags));
    }
    unsafe fn inl(port: u16) -> u32 {
        let mut val: u32;
        core::arch::asm!("in eax, dx", out("eax") val, in("dx") port, options(nostack, preserves_flags));
        val
    }
    unsafe fn pci_read_dword(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = (1 << 31)
            | ((bus as u32) << 16)
            | ((dev as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xFC);
        outl(PCI_CONFIG_ADDRESS, address);
        inl(PCI_CONFIG_DATA)
    }

    let mut nic_found = false;
    for dev in 0..32u8 {
        let vendor_device = unsafe { pci_read_dword(0, dev, 0, 0) };
        if vendor_device == 0xFFFFFFFF { continue; }
        let vendor_id = (vendor_device & 0xFFFF) as u16;
        let device_id = (vendor_device >> 16) as u16;
        if vendor_id == 0x1022 && device_id == 0x2000 {
            println!("NIC: PCnet‑II detected (bus 0, device {:#02x})", dev);
            nic_found = true;
            break;
        }
    }
    if !nic_found {
        println!("NIC: not detected");
    }

    // ----- Network placeholder -----
    println!("Network: (no stack implemented – add smoltcp for DHCP/IP)");
}
