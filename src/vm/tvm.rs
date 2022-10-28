use crate::vm::builtins::BuiltIn;
use crate::vm::frame::{Frame, FrameData};
use crate::vm::instruction::Instruction;
use crate::vm::memory::Memory;
use crate::vm::program::Program;

#[derive(Debug)]
pub struct Tvm {
    pub memory: Memory,
    program: Program,
}

impl Tvm {
    pub fn new(program: Program) -> Tvm {
        let mut memory = Memory::new();
        for (location, value) in &program.heap {
            memory[*location] = *value;
        }
        Tvm {
            memory,
            program,
        }
    }
    
    pub fn eval_frame(&mut self, frame: &FrameData) {
        match frame {
            FrameData::Instruction(instruction) => println!("{:?}", instruction),
            FrameData::Frame(frame) => {
                self.eval(frame);
            },
            FrameData::SubFrame(frame) => {
                frame.iter().for_each(|frame| self.eval_frame(frame))
            }
        }
    }

    pub fn eval(&mut self, frame: &Frame) {
        frame.frame_data.iter().for_each(|frame| self.eval_frame(frame))
    }

    pub fn call(&mut self, built_in: &BuiltIn) {
        let frame = built_in.frame();
        self.eval(&frame);
    }
}
