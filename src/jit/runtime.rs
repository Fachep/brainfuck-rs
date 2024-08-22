use std::arch::asm;
use std::io::{Read, Write};
use std::mem::{ManuallyDrop, MaybeUninit};
use crate::jit::executable_memory::{EXECUTABLE_ALLOCATOR, ExecutableAllocator};

pub struct Runtime {
    codes: Option<Vec<u8, ExecutableAllocator>>,
    __rt_input: unsafe fn(*mut u8) -> *mut u8,
    __rt_output: unsafe fn(*const u8) -> *const u8,
    __rt_mem_extend: unsafe fn(usize) -> usize,
}

impl Default for Runtime {
    fn default() -> Self {
        unsafe fn __rt_mem_extend(current: usize) -> usize {
            let mut start;
            let mut end;

            asm!("mov {}, rbx", out(reg) start);
            asm!("mov {}, rdx", out(reg) end);

            if current > end {
                let mut memory = Vec::from_raw_parts(start as *mut u8, end - start, end - start);
                let size = current - start;
                memory.resize(size, 0);
                memory.resize(memory.capacity(), 0);
                assert_eq!(memory.len(), memory.capacity());
                let (start, len) = {
                    let mut m = ManuallyDrop::new(memory);
                    (m.as_ptr(), m.len())
                };
                let start = start as usize;
                let end = start + len;

                asm!("mov rbx, {}", in(reg) start);
                asm!("mov rdx, {}", in(reg) end);

                start + size
            } else if current < start {
                let size = end - current;
                let mut new = Vec::with_capacity(size);
                new.resize(new.capacity(), 0u8);
                (start as *const u8).copy_to((new.as_mut_ptr() as usize + new.len() - size) as _, size);
                let (start, len) = {
                    let mut m = ManuallyDrop::new(new);
                    (m.as_ptr(), m.len())
                };
                let start = start as usize;
                let end = start + len;

                asm!("mov rbx, {}", in(reg) start);
                asm!("mov rdx, {}", in(reg) end);

                end - size
            } else {
                current
            }
        }
        unsafe fn __rt_input(x: *mut u8) -> *mut u8 {
            let mut start: usize;
            let mut end: usize;
            asm!(
                "mov {}, rbx",
                "mov {}, rdx",
                out(reg) start,
                out(reg) end
            );
            {

                std::io::stdin().lock().read_exact(std::slice::from_raw_parts_mut(x, 1)).unwrap();
            }

            asm!(
                "mov rbx, {}",
                "mov rdx, {}",
                in(reg) start,
                in(reg) end,
            );
            x
        }
        unsafe fn __rt_output(x: *const u8) ->  *const u8{
            let mut start: usize;
            let mut end: usize;
            asm!(
                "mov {}, rbx",
                "mov {}, rdx",
                out(reg) start,
                out(reg) end
            );
            {
                let mut stdout = std::io::stdout().lock();
                stdout.write_all(std::slice::from_raw_parts(x, 1)).unwrap();
            }

            asm!(
                "mov rbx, {}",
                "mov rdx, {}",
                in(reg) start,
                in(reg) end,
            );
            x
        }
        Self {
            codes: None,
            __rt_input,
            __rt_output,
            __rt_mem_extend,
        }
    }
}

impl Runtime {
    pub fn set_codes(&mut self, codes: Vec<u8>) {
        self.codes = Some(codes.to_vec_in(EXECUTABLE_ALLOCATOR));
    }

    pub fn codes(&self) -> &Option<Vec<u8, ExecutableAllocator>> {
        &self.codes
    }

    pub fn run(&self) {
        let Some(ref codes) = self.codes else { return; };
        let (start, len) = {
            let mut m = ManuallyDrop::new(Vec::with_capacity(64));
            let cap = m.capacity();
            m.resize(cap, 0u8);
            (m.as_ptr(), cap)
        };
        let start = start as usize;
        let end = start + len;
        unsafe {
            let f: fn(usize) = std::mem::transmute(codes.as_ptr());
            asm!("mov rbx, {}", in(reg) start);
            asm!("mov rdx, {}", in(reg) end);
            f(start+(len>>1));
        };
        let r = unsafe { Vec::from_raw_parts(start as *mut u8, end - start, end - start) };
    }

    pub fn rt_input(&self) -> usize {
        self.__rt_input as _
    }

    pub fn rt_output(&self) -> usize {
        self.__rt_output as _
    }

    pub fn rt_mem_extend(&self) -> usize {
        self.__rt_mem_extend as _
    }
}

#[test]
fn t() {
    let mut rt = Runtime::default();
    let mut i = 0;
    unsafe { (rt.__rt_input)(&mut i) };
    println!("{}", i as char);
}

#[test]
fn b() {
    use std::alloc::{alloc, Layout};
    let arr: Box<[i32;10]> = unsafe { Box::from_raw(alloc(Layout::new::<[i32;10]>()).cast()) };
    println!("{:?}", arr);
}
