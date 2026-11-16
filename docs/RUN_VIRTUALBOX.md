# Running RustOS in VirtualBox

This guide explains how to build the ISO, launch the VM, and interact with the OS.

## Prerequisites
- **VirtualBox** installed and added to the system `PATH` (so `VBoxManage` is callable from PowerShell).
- **Docker** (used by the build scripts) – the Docker image `rustos-builder` must be available.
- **PowerShell** (default on Windows) for the provided helper scripts.

## Build the ISO
```powershell
# From the repository root
.\scripts\build.ps1
```
The script uses Docker to compile the kernel and then creates `build\rustos.iso` with GRUB.

## Launch the VM
```powershell
# Starts the VM head‑less (no UI). The VM name is "RustOS" as defined in the script.
VBoxManage startvm RustOS --type headless
```
The VM will boot, GRUB will load the kernel, and you will see the desktop menu.

## Interact
- The VM window is not displayed in head‑less mode. To view the console you can:
  - Open the VirtualBox GUI and attach a **VNC** or **RDP** display, or
  - Run the VM in `--type gui` instead of `headless`.
- All kernel output (including boot messages) is written to `build\serial.log` via the COM1 serial port.

## Stop the VM
```powershell
VBoxManage controlvm RustOS poweroff
```
Alternatively, you can shut down from within the OS by selecting **Exit** from the desktop menu.

---
*The `boot_test.ps1` script automates the start‑wait‑stop‑verify process for CI.*