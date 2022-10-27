use std::ops;
use crate::vm::memory::Memory;

pub enum Pointer<'a> {
    StackPointer(&'a mut usize),
    FramePointer(&'a mut usize),
    ReferencePointer(&'a mut usize),
}

impl ops::Shl<i32> for &Pointer<'_> {
    type Output = ();

    fn shl(mut self, rhs: i32) -> Self::Output {
        match self {
            Pointer::StackPointer(sp) => **sp = rhs as usize,
            Pointer::FramePointer(fp) => **fp = rhs as usize,
            Pointer::ReferencePointer(rp) => **rp = rhs as usize,
        };
    }
}

impl ops::Mul<&mut Memory<'_>> for &Pointer<'_> {
    type Output = i32;

    fn mul(self, rhs: &mut Memory<'_>) -> Self::Output {
        match self {
            Pointer::StackPointer(sp) => rhs.mem[**sp],
            Pointer::FramePointer(fp) => rhs.mem[**fp],
            Pointer::ReferencePointer(rp) => rhs.mem[**rp],
        }
    }
}

impl From<usize> for Pointer<'_> {
    fn from(mut ptr: usize) -> Self {
        Pointer::ReferencePointer(&mut ptr)
    }
}