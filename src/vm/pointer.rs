use std::ops;
use crate::vm::memory::Memory;

#[derive(Debug)]
pub enum Pointer {
    StackPointer(usize),
    FramePointer(usize),
    ReferencePointer(usize),
}

impl Pointer {
    pub fn value(&self) -> usize {
        match self {
            Pointer::StackPointer(sp) => *sp,
            Pointer::FramePointer(fp) => *fp,
            Pointer::ReferencePointer(rp) => *rp,
        }
    }

    fn set(&self, value: usize) -> Pointer {
        match self {
            Pointer::StackPointer(_) => Pointer::StackPointer(value),
            Pointer::FramePointer(_) => Pointer::FramePointer(value),
            Pointer::ReferencePointer(_) => Pointer::ReferencePointer(value),
        }
    }
}

impl ops::Shl<i32> for &Pointer {
    type Output = Pointer;

    fn shl(self, rhs: i32) -> Self::Output {
        self.set(rhs as usize)
    }
}

impl ops::Mul<&mut Memory> for &Pointer {
    type Output = i32;

    fn mul(self, rhs: &mut Memory) -> Self::Output {
        match self {
            Pointer::StackPointer(sp) => rhs.mem[*sp],
            Pointer::FramePointer(fp) => rhs.mem[*fp],
            Pointer::ReferencePointer(rp) => rhs.mem[*rp],
        }
    }
}

impl From<usize> for Pointer {
    fn from(mut ptr: usize) -> Self {
        Pointer::ReferencePointer(ptr)
    }
}