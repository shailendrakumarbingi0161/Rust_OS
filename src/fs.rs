// In-memory file system used by the educational OS.
// Supports up to 16 files, each with a maximum size of 2048 bytes.
// The API is deliberately simple – just enough for the shell commands.

use core::fmt::Write;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use crate::vga::Writer;

const MAX_FILES: usize = 16;
const MAX_NAME: usize = 24;
const MAX_SIZE: usize = 2048;

#[derive(Clone, Copy)]
struct FileEntry {
    name: [u8; MAX_NAME],      // zero-padded UTF-8 name
    len: usize,                // actual length of data
    data: [u8; MAX_SIZE],
    writable: bool,            // true = read-write, false = read-only
}

impl FileEntry {
    fn name_str(&self) -> &str {
        // Find first 0 byte termination.
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(MAX_NAME);
        core::str::from_utf8(&self.name[..end]).unwrap_or("")
    }
}

// The file table lives in a static mutable array.
static mut FILE_TABLE: [Option<FileEntry>; MAX_FILES] = [None; MAX_FILES];

/// Helper: write a file name into a fixed-size buffer (zero-padded).
fn write_name(buf: &mut [u8; MAX_NAME], name: &str) {
    let bytes = name.as_bytes();
    let len = core::cmp::min(bytes.len(), MAX_NAME - 1);
    for i in 0..len {
        buf[i] = bytes[i];
    }
    buf[len] = 0; // null-terminate
    for i in len + 1..MAX_NAME {
        buf[i] = 0;
    }
}

/// Initialise the file system with the four starter files.
pub fn init() {
    let starters: [(&str, &[u8], bool); 4] = [
        ("readme.txt", b"RustOS - educational 32-bit OS.\nSee docs/README.md for details.\n", true),
        ("notes.txt", b"", true),
        ("todo.txt", b"- Implement shell commands\n- Add games\n", true),
        ("about.txt", b"RustOS version 0.1 - built for teaching OS fundamentals.\n", true),
    ];
    for &(name, data, writable) in starters.iter() {
        let _ = create_file(name, data, writable);
    }
}

/// Find a file index by name.
pub fn find_index(name: &str) -> Option<usize> {
    unsafe {
        for (i, entry) in FILE_TABLE.iter().enumerate() {
            if let Some(e) = entry {
                if e.name_str() == name {
                    return Some(i);
                }
            }
        }
    }
    None
}

/// Create a new file. Returns `Ok(())` on success or an error string.
pub fn create_file(name: &str, data: &[u8], writable: bool) -> Result<(), &'static str> {
    if name.len() >= MAX_NAME {
        return Err("name too long");
    }
    if data.len() > MAX_SIZE {
        return Err("data too large");
    }
    unsafe {
        if find_index(name).is_some() {
            return Err("file exists");
        }
        // Find a free slot.
        for slot in FILE_TABLE.iter_mut() {
            if slot.is_none() {
                let mut entry = FileEntry {
                    name: [0; MAX_NAME],
                    len: data.len(),
                    data: [0; MAX_SIZE],
                    writable,
                };
                write_name(&mut entry.name, name);
                entry.data[..data.len()].copy_from_slice(data);
                *slot = Some(entry);
                return Ok(());
            }
        }
    }
    Err("no free slot")
}

/// Read a file's contents. Returns a slice of the stored data.
pub fn read_file(name: &str) -> Result<&'static [u8], &'static str> {
    unsafe {
        if let Some(idx) = find_index(name) {
            if let Some(ref e) = FILE_TABLE[idx] {
                return Ok(&e.data[..e.len]);
            }
        }
    }
    Err("not found")
}

/// Write data to an existing file (must be writable).
pub fn write_file(name: &str, data: &[u8]) -> Result<(), &'static str> {
    if data.len() > MAX_SIZE {
        return Err("data too large");
    }
    unsafe {
        if let Some(idx) = find_index(name) {
            if let Some(ref mut e) = FILE_TABLE[idx] {
                if !e.writable {
                    return Err("read-only");
                }
                e.len = data.len();
                e.data[..data.len()].copy_from_slice(data);
                return Ok(());
            }
        }
    }
    Err("not found")
}

/// Delete a file (if it exists and is writable). "read-only" files cannot be removed.
pub fn delete_file(name: &str) -> Result<(), &'static str> {
    unsafe {
        if let Some(idx) = find_index(name) {
            if let Some(ref e) = FILE_TABLE[idx] {
                if !e.writable {
                    return Err("read-only");
                }
            }
            FILE_TABLE[idx] = None;
            return Ok(());
        }
    }
    Err("not found")
}

/// Change permission – 6 = read-write, 4 = read-only.
pub fn chmod(name: &str, mode: u8) -> Result<(), &'static str> {
    unsafe {
        if let Some(idx) = find_index(name) {
            if let Some(ref mut e) = FILE_TABLE[idx] {
                e.writable = mode == 6;
                return Ok(());
            }
        }
    }
    Err("not found")
}

/// List files – returns a `Vec<(String, usize, bool)>`.
pub fn list_files() -> Vec<(String, usize, bool)> {
    let mut list = Vec::new();
    unsafe {
        for entry in FILE_TABLE.iter() {
            if let Some(ref e) = entry {
                let name = e.name_str().to_string();
                list.push((name, e.len, e.writable));
            }
        }
    }
    list
}

/// Print the file list to the VGA screen.
pub fn print_ls() {
    let mut writer = Writer::new(0x07);
    let _ = writeln!(writer, "Perm Size Name");
    for (name, size, writable) in list_files() {
        let perms = if writable { "rw-" } else { "r--" };
        let _ = writeln!(writer, "{} {:>5} {}", perms, size, name);
    }
}

/// Helper to print an error in red (used by the shell).
pub fn print_error(msg: &str) {
    let mut writer = Writer::new(0x04);
    let _ = writer.write_str("Error: ");
    let _ = writer.write_str(msg);
    let _ = writer.write_str("\n");
}
