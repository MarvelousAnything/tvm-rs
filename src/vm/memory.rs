use std::ops;
use crate::vm::register::Pointer;

pub struct Memory<'a> {
    pub mem: [i32; 65536],
    pub stack_pointer: Pointer<'a>,
    pub frame_pointer: Pointer<'a>,
}

impl Memory<'_> {
    pub fn new<'a>() -> Memory<'a> {
        Memory {
            mem: [0; 65536],
            stack_pointer: Pointer::StackPointer(&mut 65535),
            frame_pointer: Pointer::FramePointer(&mut 65535),
        }
    }

    pub fn push(&mut self, value: i32) {
        self[&self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> i32 {
        self.stack_pointer -= 1;
        self.mem[self.stack_pointer]
    }


}

impl ops::Index<&Pointer<'_>> for Memory<'_> {
    type Output = i32;

    fn index(&self, ptr: &Pointer) -> &Self::Output {
        match ptr {
            Pointer::StackPointer(sp) => &self.mem[**sp],
            Pointer::FramePointer(fp) => &self.mem[**fp],
            Pointer::ReferencePointer(rp) => &self.mem[**rp],
        }
    }
}

impl ops::IndexMut<&Pointer<'_>> for Memory<'_> {
    fn index_mut(&mut self, ptr: &Pointer) -> &mut Self::Output {
        match ptr {
            Pointer::StackPointer(sp) => &mut self.mem[**sp],
            Pointer::FramePointer(fp) => &mut self.mem[**fp],
            Pointer::ReferencePointer(rp) => &mut self.mem[**rp],
        }
    }
}

impl ops::Index<usize> for Memory<'_> {
    type Output = i32;

    fn index(&self, ptr: usize) -> &Self::Output {
        &self[&Pointer::from(ptr)]
    }
}

impl ops::IndexMut<usize> for Memory<'_> {
    fn index_mut(&mut self, ptr: usize) -> &mut Self::Output {
        &mut self[&Pointer::from(ptr)]
    }
}

impl ops::Shl<i32> for &Memory<'_> {
    type Output = ();

    fn shl(mut self, rhs: i32) -> Self::Output {
        self.push(rhs);
    }
}

