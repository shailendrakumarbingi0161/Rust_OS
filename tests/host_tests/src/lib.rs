//! Host‑Side Test Suite for RustOS
//!
//! This crate contains lightweight pure‑Rust unit tests that verify the core logic used by the kernel.
//! The tests run on the host with the standard library, so they can be executed quickly via `cargo test`.

use std::collections::HashMap;

/// Simple in‑memory filesystem used only for testing.
#[derive(Default)]
struct InMemoryFs {
    files: HashMap<String, Vec<u8>>, // name → contents
}

impl InMemoryFs {
    fn new() -> Self {
        Self::default()
    }

    fn write(&mut self, name: &str, data: &[u8]) {
        self.files.insert(name.to_string(), data.to_vec());
    }

    fn read(&self, name: &str) -> Option<&[u8]> {
        self.files.get(name).map(|v| v.as_slice())
    }

    fn delete(&mut self, name: &str) -> bool {
        self.files.remove(name).is_some()
    }
}

/// Very small Banker's‑algorithm style deadlock detector.
///
/// * `allocation` – resources currently allocated to the process.
/// * `maximum` – maximum resources the process may request.
/// * `available` – resources currently free in the system.
///
/// Returns `true` if the system is **unsafe** (i.e., a deadlock could occur).
fn is_deadlock(allocation: &[u32], maximum: &[u32], available: &[u32]) -> bool {
    // Compute need = maximum - allocation
    let need: Vec<u32> = maximum.iter().zip(allocation).map(|(m, a)| m - a).collect();
    // If for every resource type need <= available, the system is safe.
    // Otherwise it is unsafe.
    need.iter().zip(available).any(|(n, av)| n > av)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(2 + 2, 4);
        assert_eq!(10 - 3, 7);
    }

    #[test]
    fn test_in_memory_fs() {
        let mut fs = InMemoryFs::new();
        fs.write("hello.txt", b"Hello, world!");
        assert_eq!(fs.read("hello.txt"), Some(b"Hello, world!" as &[u8]));
        assert!(fs.delete("hello.txt"));
        assert!(fs.read("hello.txt").is_none());
    }

    #[test]
    fn test_deadlock_detection_safe() {
        // One process, 2 resource types.
        // Allocation: [1, 0]
        // Maximum:    [3, 2]
        // Available:  [2, 2]
        let allocation = [1, 0];
        let maximum = [3, 2];
        let available = [2, 2];
        assert!(!is_deadlock(&allocation, &maximum, &available)); // safe
    }

    #[test]
    fn test_deadlock_detection_unsafe() {
        // Allocation: [2, 2]
        // Maximum:    [3, 3]
        // Available:  [0, 0]
        let allocation = [2, 2];
        let maximum = [3, 3];
        let available = [0, 0];
        assert!(is_deadlock(&allocation, &maximum, &available)); // unsafe
    }
}
