use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::SeqCst};
pub struct Strallocator;

static ALLOCATED: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for Strallocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(l.size() as u64, SeqCst);
        System.alloc(l)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
        ALLOCATED.fetch_sub(l.size() as u64, SeqCst);
        System.dealloc(ptr, l);
    }
}

impl Strallocator {
    pub fn reset(&self) {
        ALLOCATED.store(0, SeqCst);
    }

    pub fn get(&self) -> u64 {
        ALLOCATED.load(SeqCst)
    }
}