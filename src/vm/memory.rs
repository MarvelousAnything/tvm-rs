use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Memory {
    pub mem: [i32; 65536],
    pub stack_pointer: usize,
    pub frame_pointer: usize,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 65536],
            stack_pointer: 65535,
            frame_pointer: 65535
        }
    }

    pub fn push(&mut self, value: i32) {
        self.mem[self.stack_pointer] = value;
        self.stack_pointer -= 1;
        // println!("Pushed {} to stack", value);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack_pointer += 1;
        // println!("Popped {} from stack", self.mem[self.stack_pointer]);
        self.mem[self.stack_pointer]
    }

    pub fn peek(&self) -> i32 {
        self.mem[self.stack_pointer]
    }
}

impl Index<usize> for Memory {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mem[index]
    }
}