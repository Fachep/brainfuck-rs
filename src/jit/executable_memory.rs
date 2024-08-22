
use std::ptr::NonNull;
use std::alloc::{Allocator, AllocError, Layout};

#[link(name = "kernel32")]
extern "C" {
    fn VirtualAlloc(lp_address: usize, dw_size: usize, fl_allocation_type: u32, fl_protect: u32) -> Option<NonNull<u8>>;
    fn VirtualFree(lp_address: NonNull<u8>, dw_size: usize, dw_free_type: u32) -> bool;
    fn GetLastError() -> i32;
}

pub struct ExecutableAllocator;
unsafe impl Allocator for ExecutableAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe {
            VirtualAlloc(0, layout.size(), 0x1000 | 0x2000, 0x40)
        };
        ptr.map_or_else(
            || Err(AllocError),
            |data| Ok(NonNull::slice_from_raw_parts(data, layout.size()))
        )
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        VirtualFree(ptr, layout.size(), 0x8000);
    }
}

pub const EXECUTABLE_ALLOCATOR: ExecutableAllocator = ExecutableAllocator;

#[test]
fn test() {
    fn a(x:&u8) {
        println!("{}", x);
    }
    let mut v = vec![254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 72, 184];
    v.extend(&(a as *const () as u64).to_ne_bytes());
    v.extend([0xFF, 0xD0, 0xC3]);
    let v = v.to_vec_in(EXECUTABLE_ALLOCATOR);
    println!("{:?}", v);
    let mut r = 0u8;
    let f: fn(usize) = unsafe { std::mem::transmute(v.as_ptr()) };
    f(&mut r as *mut u8 as usize);
    println!("{}", r);
}
