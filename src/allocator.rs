// src/allocator.rs
//! Minimal heap allocator implementing GlobalAlloc.

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB heap
static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub struct BumpAllocator {
    offset: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            offset: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let mut cur = self.offset.load(Ordering::Relaxed);
        loop {
            let aligned = (cur + align - 1) & !(align - 1);
            let next = aligned + size;
            if next > HEAP_SIZE {
                return core::ptr::null_mut();
            }
            match self.offset.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => {
                    let ptr = unsafe { HEAP_MEM.as_mut_ptr().add(aligned) };
                    return ptr;
                }
                Err(actual) => cur = actual,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not free individual items
    }
}

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();

