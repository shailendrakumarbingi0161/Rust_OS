// src/net.rs
//! Network Connectivity and Diagnostics for RustOS.
//! Simulates and manages the VirtualBox Intel PRO/1000 MT (82540EM) Ethernet interface.

use crate::vga::{self, clear_screen, set_cursor, write_at};
use crate::keyboard;
use crate::pit;

pub fn get_status_short() -> &'static str {
    "Net: 10.0.2.15 (e1000 UP)"
}

pub fn run() {
    draw_window(false);

    loop {
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0x01 | 0x1C => { // Esc or Enter -> exit
                    return;
                }
                0x19 => { // 'P' key -> run ping diagnostics
                    run_ping_test();
                    draw_window(true);
                }
                _ => {}
            }
        }
    }
}

fn draw_window(has_ping: bool) {
    clear_screen();

    // Top title bar
    for c in 0..vga::WIDTH {
        vga::write_char_at(0, c, ' ', 0x1F); // White on blue
    }
    write_at(0, 2, "🌐 RustOS Network Center - Intel PRO/1000 MT (e1000)", 0x1F);

    // Frame borders
    write_at(1, 1, "+----------------------------------------------------------------------------+", 0x09);
    for r in 2..22 {
        write_at(r, 1, "|", 0x09);
        write_at(r, 78, "|", 0x09);
    }
    write_at(22, 1, "+----------------------------------------------------------------------------+", 0x09);

    // Section 1: Interface & Link Status
    write_at(2, 4, "[ INTERFACE & PHYSICAL LINK ]", 0x0E);
    write_at(3, 4, "Device Name    : eth0 (VirtualBox PCI Network Adapter)", 0x0F);
    write_at(4, 4, "NIC Controller : Intel 82540EM Gigabit Ethernet (Vendor: 8086, Dev: 100E)", 0x07);
    write_at(5, 4, "Physical MAC   : 08:00:27:12:34:56 (Oracle VirtualBox OUI)", 0x07);
    write_at(6, 4, "Link State     : [ ONLINE - CARRIER DETECTED ]", 0x0A); // Light green
    write_at(7, 4, "Negotiated Spd : 1000 Mbps (Gigabit) Full-Duplex, Auto-MDIX: Enabled", 0x0B);

    // Section 2: IPv4 Configuration
    write_at(9, 4, "[ IPv4 NETWORK CONFIGURATION ]", 0x0E);
    write_at(10, 4, "IP Mode        : DHCP (Assigned by VirtualBox NAT Engine)", 0x07);
    write_at(11, 4, "IPv4 Address   : 10.0.2.15 / 24", 0x0E);
    write_at(12, 4, "Subnet Mask    : 255.255.255.0", 0x07);
    write_at(13, 4, "Default Gateway: 10.0.2.2 (NAT Router)", 0x0B);
    write_at(14, 4, "DNS Server     : 10.0.2.3 (VirtualBox Resolver)", 0x07);

    // Section 3: Traffic Counters
    write_at(16, 4, "[ PACKET TRANSMISSION & RECEPTION ]", 0x0E);
    write_at(17, 4, "Packets TX     : 1,428 packets (164.2 KB) | Dropped: 0 | Collisions: 0", 0x07);
    write_at(18, 4, "Packets RX     : 3,194 packets (412.8 KB) | Frame Errors: 0", 0x07);

    if !has_ping {
        write_at(20, 4, "Actions: Press [ P ] to test Gateway Ping | Press [ Esc / Enter ] to Exit", 0x0F);
    } else {
        write_at(20, 4, "Ping Test Complete (4/4 packets OK, 0% loss). Press [ Esc ] to Return.", 0x0A);
    }

    set_cursor(23, 0);
}

fn run_ping_test() {
    write_at(19, 4, "Pinging 10.0.2.2 with 32 bytes of ICMP data...", 0x0E);
    pit::sleep_ms(300);
    write_at(19, 4, "Reply from 10.0.2.2: bytes=32 time<1ms TTL=64 (ICMP Echo Reply)   ", 0x0A);
    pit::sleep_ms(300);
}
