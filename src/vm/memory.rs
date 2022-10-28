use std::ops;
use crate::vm::pointer::Pointer;

#[derive(Debug)]
pub struct Memory {
    pub mem: [i32; 65536],
    pub stack_pointer: Pointer,
    pub frame_pointer: Pointer,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 65536],
            stack_pointer: Pointer::StackPointer(65535),
            frame_pointer: Pointer::FramePointer(65535),
        }
    }

    pub fn push(&mut self, value: i32) {
        self.mem[self.stack_pointer.value()] = value;
        self.stack_pointer = &self.stack_pointer << 1;
    }

}

impl ops::Index<&Pointer> for Memory {
    type Output = i32;

    fn index(&self, ptr: &Pointer) -> &Self::Output {
        match ptr {
            Pointer::StackPointer(sp) => &self.mem[*sp],
            Pointer::FramePointer(fp) => &self.mem[*fp],
            Pointer::ReferencePointer(rp) => &self.mem[*rp],
        }
    }
}

impl ops::IndexMut<&Pointer> for Memory {
    fn index_mut(&mut self, ptr: &Pointer) -> &mut Self::Output {
        match ptr {
            Pointer::StackPointer(sp) => &mut self.mem[*sp],
            Pointer::FramePointer(fp) => &mut self.mem[*fp],
            Pointer::ReferencePointer(rp) => &mut self.mem[*rp],
        }
    }
}

impl ops::Index<usize> for Memory {
    type Output = i32;

    fn index(&self, ptr: usize) -> &Self::Output {
        &self[&Pointer::from(ptr)]
    }
}

impl ops::IndexMut<usize> for Memory {
    fn index_mut(&mut self, ptr: usize) -> &mut Self::Output {
        &mut self[&Pointer::from(ptr)]
    }
}

impl ops::Shl<i32> for Memory {
    type Output = ();

    fn shl(mut self, rhs: i32) -> Self::Output {
        self.push(rhs);
    }
}

