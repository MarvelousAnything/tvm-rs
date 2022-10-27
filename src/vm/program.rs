/*
    A program consists of:
      - a tuple with the frame id for the init function and the size of the heap.
      - a list of tuples with the location on the heap and the value.
      - Frames
 */
use crate::vm::frame::Frame;

pub struct Program {
    pub entry_point: usize,
    pub heap_size: usize,
    pub heap: Vec<(usize, i32)>,
    pub frames: Vec<Frame>,
}

impl Program {
    pub fn new(entry_point: usize, heap_size: usize, heap: Vec<(usize, i32)>, frames: Vec<Frame>) -> Program {
        Program {
            entry_point,
            heap_size,
            heap,
            frames,
        }
    }
}