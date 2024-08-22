use std::io::{Result, Write};
use crate::jit::asm::register::*;

pub trait Command {
    fn compile(self, dest: &mut impl Write) -> Result<()>;
}

pub enum Immediate {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}
