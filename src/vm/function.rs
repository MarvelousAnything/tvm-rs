use crate::vm::builtins::BuiltIn;
use crate::vm::instruction::Instruction;
use serde_json::Value;
use std::fmt::Display;

// A frame is a list of instructions or subframes.
#[derive(Debug, Clone)]
pub struct Function {
    pub id: usize,
    pub name: String,
    pub args: u32,
    pub locals: u32,
    pub frame: Frame,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub frame_id: usize,
    pub frame_data: Vec<FrameData>,
}

impl FromIterator<FrameData> for Frame {
    fn from_iter<T: IntoIterator<Item = FrameData>>(iter: T) -> Self {
        Frame {
            frame_id: 0,
            frame_data: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FrameData {
    Function(Function),
    BuiltIn(BuiltIn),
    Frame(Frame),
    Instruction(Instruction),
    Primitive(i32),
}

impl FrameData {
    pub fn of(json: &Value) -> FrameData {
        match json {
            Value::Number(op) => match op.as_i64().unwrap() as i32 {
                n @ -111..=-101 => FrameData::BuiltIn(BuiltIn::get_builtin(n)),
                n @ 1..=27 => FrameData::Instruction(Instruction::get_instruction(n as u32)),
                n => FrameData::Primitive(n),
            },
            Value::Array(val) => FrameData::Frame(val.iter().map(FrameData::of).collect()),
            _ => panic!("Invalid frame data"),
        }
    }
}

impl Function {
    pub fn from_json(json: &Value) -> Self {
        let id = json[0].as_u64().expect("frame id") as usize;
        let name = json[1].as_str().expect("frame name").to_string();
        let args = json[2].as_u64().unwrap() as u32;
        let locals = json[3].as_u64().unwrap() as u32;
        let frame_data: Vec<FrameData> = json[4]
            .as_array()
            .unwrap()
            .iter()
            .map(FrameData::of)
            .collect();
        Function {
            id,
            name,
            args,
            locals,
            frame: Frame::from_iter(frame_data),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Function {{ id: {}, name: {}, args: {}, locals: {}, frame: {:?} }}",
            self.id, self.name, self.args, self.locals, self.frame
        )
    }
}
