use crate::callable::Callable;
use crate::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub id: usize,
    pub name: String,
    pub data: Vec<FrameData>,
    pub pc: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameData {
    Frame(Frame),
    Callable(Callable, Vec<i32>), // TODO: Maybe this should be a reference to the callable id?
    Instruction(Instruction, Vec<i32>),
    Primitive(i32),
}