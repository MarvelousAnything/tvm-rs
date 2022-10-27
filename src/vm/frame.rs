use serde_json::Value;
use crate::vm::instruction::Instruction;

pub struct Frame {
    pub id: usize,
    pub name: String,
    pub args: u32,
    pub locals: u32,
    pub frame_data: Vec<FrameData>,
}

enum FrameData {
    Frame(Frame),
    Instruction(Instruction),
}

impl FrameData {
    pub fn of()
}

impl Frame {
    pub fn from_json(json: &Value) -> Self {
        let id = json[0].as_u64().unwrap() as usize;
        let name = json[1].as_str().unwrap().to_string();
        let args = json[2].as_u64().unwrap() as u32;
        let locals = json[3].as_u64().unwrap() as u32;
        let instructions_raw = json[4].as_array().unwrap();
        Frame {
            id,
            name,
            args,
            locals,
            frame_data: Vec::new(),
        }
    }

    pub fn read_instructions(instructions: &Vec<Value>) -> FrameData {
        for instruction in instructions {
            match instruction {
                Value::Array(ref op) => {
                    Frame::from_json(instruction);
                }
                _ => panic!("Invalid instruction"),
            }
        }
    }
}