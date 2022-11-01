use crate::callable::Callable;
use crate::instruction::Instruction;
use crate::state::{Stateful, StateResult};
use crate::state::StateResult::Continue;
use crate::tvm::Tvm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub id: usize,
    pub name: String,
    pub data: Vec<FrameData>,
    pub pc: usize,
    pub previous_frame: Option<Box<Frame>>,
    pub result: Option<StateResult>,
}

impl Frame {
    pub fn builder() -> FrameBuilder {
        FrameBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameData {
    Frame(Frame),
    Callable(Callable, Vec<i32>), // TODO: Maybe this should be a reference to the callable id?
    Instruction(Instruction, Vec<i32>),
    Primitive(i32),
}

impl FrameData {
    pub fn get_id(&self) -> i32 {
        match self {
            FrameData::Frame(frame) => frame.id as i32,
            FrameData::Callable(callable, _) => callable.get_id(),
            FrameData::Instruction(instruction, _) => instruction.get_op() as i32,
            FrameData::Primitive(value) => *value,
        }
    }
}

pub trait FrameEvaluator {
    fn do_frame_eval(&mut self, frame: &Frame);
}

impl FrameEvaluator for Tvm {
    fn do_frame_eval(&mut self, frame: &Frame) {
        println!("Evaluating frame: {:?}", frame);
        if self.should_continue() {
            self.eval(frame.clone());
        }
    }
}

#[derive(Default)]
pub struct FrameBuilder {
    id: usize,
    name: String,
    data: Vec<FrameData>,
}

impl FrameBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn frame(mut self, frame: Frame) -> Self {
        self.data.push(FrameData::Frame(frame));
        self
    }

    pub fn callable(mut self, callable: i32, args: Vec<i32>) -> Self {
        self.data.push(FrameData::Callable(Callable::get_callable(callable), args));
        self
    }

    pub fn instruction(mut self, instruction: Instruction, args: Vec<i32>) -> Self {
        self.data.push(FrameData::Instruction(instruction, args));
        self
    }

    pub fn primitive(mut self, primitive: i32) -> Self {
        self.data.push(FrameData::Primitive(primitive));
        self
    }

    pub fn build(self) -> Frame {
        Frame {
            id: self.id,
            name: self.name,
            data: self.data,
            pc: 0,
            previous_frame: None,
            result: None
        }
    }
}