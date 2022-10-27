use serde_json::Value;
use crate::vm::frame::Frame;
use crate::vm::memory::Memory;
use crate::vm::program::Program;

type SerdeTape = Value;

pub struct Tvm<'a> {
    pub memory: Memory<'a>,
    program: Program,
    edata : i32
}

impl Tvm<'_> {
    pub fn new<'a>(program: Program) -> Tvm<'a> {
        let mut memory = Memory::new();
        for (location, value) in program.heap {
            memory[location] = value;
        }
        Tvm {
            memory,
            program,
            edata: 0
        }
    }
    
    pub fn eval_frame(&mut self, frame: &Frame) {
    }
}