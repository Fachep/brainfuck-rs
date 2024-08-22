#![cfg(all(windows, target_arch = "x86_64"))]

mod executable_memory;
pub mod runtime;
mod asm;
pub mod compile;

macro_rules! executable {
    ($x:expr) => {
        <[_]>::into_vec(Box::new_in(x, executable_memory::EXECUTABLE_ALLOCATOR));
    };
}

pub(crate) use executable;
