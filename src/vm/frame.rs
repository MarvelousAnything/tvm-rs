use serde_json::Value;
use crate::vm::instruction::Instruction;

#[derive(Debug)]
pub struct Frame {
    pub id: usize,
    pub name: String,
    pub args: u32,
    pub locals: u32,
    pub frame_data: Vec<FrameData>,
}

#[derive(Debug)]
pub struct SubFrame {
    pub pc: usize,
    pub frame_data: Vec<FrameData>,
}

impl FromIterator<FrameData> for SubFrame {
    fn from_iter<T: IntoIterator<Item=FrameData>>(iter: T) -> Self {
        SubFrame {
            frame_data: iter.into_iter().collect(),
            pc: 0,
        }
    }
}

impl SubFrame {
    pub fn next(&mut self) -> Option<&FrameData> {
        let next = self.frame_data.get(self.pc);
        self.pc += 1;
        next
    }

    pub fn get_instruction(&self) -> Option<&Instruction> {
        match self.frame_data.get(self.pc) {
            Some(FrameData::Instruction(instruction)) => Some(instruction),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum FrameData {
    Frame(Frame),
    SubFrame(SubFrame),
    Instruction(Instruction),
}

impl FrameData {
    pub fn of(json: &Value) -> FrameData {
        match json {
            Value::Number(op) => FrameData::Instruction(Instruction::get_instruction(op.as_i64().unwrap() as i32)),
            Value::Array(val) => FrameData::SubFrame(val.iter().map(FrameData::of).collect()),
            _ => panic!("Invalid frame data"),
        }
    }

    pub fn eval(&self) {
        match self {
            FrameData::Instruction(instruction) => println!("{:?}", instruction),
            FrameData::Frame(frame) => {
                frame.eval();
            },
            FrameData::SubFrame(frame) => {
                frame.frame_data.iter().for_each(|frame| frame.eval())
            }
        }
    }
}

impl Frame {
    pub fn from_json(json: &Value) -> Self {
        let id = json[0].as_u64().expect("frame id") as usize;
        let name = json[1].as_str().expect("frame name").to_string();
        let args = json[2].as_u64().unwrap() as u32;
        let locals = json[3].as_u64().unwrap() as u32;
        let frame_data: Vec<FrameData> = json[4].as_array().unwrap().iter().map(FrameData::of).collect();
        Frame {
            id,
            name,
            args,
            locals,
            frame_data
        }
    }
}