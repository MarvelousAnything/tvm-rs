use crate::callable::Callable;
use crate::instruction::Instruction;
use crate::state::{StateResult, Stateful};
use crate::tvm::Tvm;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frame {
    pub id: usize,
    pub name: String,
    pub data: Vec<FrameData>,
    pub pc: usize,
    pub parent_frame: Option<Box<Frame>>,
    pub result: Option<StateResult>,
}

impl FromIterator<FrameData> for Frame {
    fn from_iter<T: IntoIterator<Item = FrameData>>(iter: T) -> Self {
        Frame {
            id: 0,
            name: "".to_string(),
            data: iter.into_iter().collect(),
            pc: 0,
            parent_frame: None,
            result: None,
        }
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // format list of framedata as a string
        let mut data = String::new();
        for d in &self.data {
            data.push_str(&format!("\n\t{},", d));
        }
        let mut previous_frame = String::new();
        if let Some(frame) = &self.parent_frame {
            previous_frame = format!("{}", frame);
        }
        let mut result = String::new();
        if let Some(state_result) = &self.result {
            result = format!("{}", state_result);
        }
        write!(
            f,
            "Frame {{ id: {}, name: {}, data: {}, pc: {}, previous_frame: {}, result: {}, current instruction: {} }}",
            self.id, self.name, data, self.pc, previous_frame, result, self.data[self.pc]
        )
    }
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

impl Display for FrameData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameData::Frame(frame) => write!(f, "frame {} {}", frame.name, frame.pc),
            FrameData::Callable(callable, _args) => write!(f, "callable {}", callable.name()),
            FrameData::Instruction(instruction, _args) => {
                write!(f, "instruction {}", instruction.name())
            }
            FrameData::Primitive(primitive) => write!(f, "primitive {}", primitive),
        }
    }
}

impl FrameData {
    pub fn get_id(&self) -> i32 {
        match self {
            FrameData::Frame(frame) => frame.id as i32,
            FrameData::Callable(callable, _) => callable.get_id(),
            FrameData::Instruction(instruction, _) => instruction.op() as i32,
            FrameData::Primitive(value) => *value,
        }
    }
}

pub trait FrameEvaluator {
    fn do_frame_eval(&mut self, frame: &Frame);
}

impl FrameEvaluator for Tvm {
    fn do_frame_eval(&mut self, frame: &Frame) {
        println!("Evaluating frame: {}", frame.name);
        let mut frame_dup = frame.clone();
        frame_dup.parent_frame = self.get_current_frame().map(Box::new);
        if self.should_continue() {
            self.eval(frame_dup);
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
        self.data
            .push(FrameData::Callable(Callable::get_native(callable), args));
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
            parent_frame: None,
            result: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;
    use crate::state::TvmState;

    #[test]
    fn test_do_frame_eval() {
        let mut tvm = Tvm::default();
        let frame = Frame::builder()
            .name("test".to_string())
            .primitive(1)
            .primitive(2)
            .primitive(3)
            .build();
        tvm.do_frame_eval(&frame);
        assert!(
            matches!(tvm.state, TvmState::Eval(state::states::Eval { frame, .. }) if frame.name == "test")
        );
    }

    #[test]
    fn test_builder() {
        let frame = Frame::builder()
            .id(1)
            .name("test".to_string())
            .frame(Frame::builder().id(2).name("test2".to_string()).build())
            .callable(-101, vec![1, 2, 3])
            .instruction(Instruction::get_instruction(1), vec![1, 2, 3])
            .primitive(1)
            .build();
        assert_eq!(frame.id, 1);
        assert_eq!(frame.name, "test");
        assert_eq!(frame.data.len(), 4);
        assert!(
            matches!(&frame.data[0], FrameData::Frame(frame) if frame.name == "test2" && frame.id == 2)
        );
        assert!(
            matches!(&frame.data[1], FrameData::Callable(Callable::Native(native), _) if native.id() == -101)
        );
        assert!(
            matches!(&frame.data[2], FrameData::Instruction(instruction, _) if instruction.op() == 1)
        );
        assert!(matches!(&frame.data[3], FrameData::Primitive(n) if *n == 1));
    }
}
