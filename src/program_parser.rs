use core::num::dec2flt::parse::parse_number;
use serde_json::Value;
use crate::callable::Callable;
use crate::frame::{Frame, FrameBuilder, FrameData};
use crate::function::{Function, FunctionBuilder};
use crate::instruction::Instruction;
use crate::program::{Program, ProgramBuilder};

#[derive(Debug, Default)]
pub struct ProgramParser {
    input: String,
    root_name: String,
    program_builder: ProgramBuilder,
    current_value: Option<Value>,
    previous_value: Option<Value>,
    current_frame_data: Option<FrameData>,
    previous_frame_data: Option<FrameData>,
}

impl ProgramParser {
    pub fn new(input: String, root_name: String) -> Self {
        Self {
            input,
            root_name,
            program_builder: ProgramBuilder::new(),
            current_value: None,
            previous_value: None,
            current_frame_data: None,
            previous_frame_data: None,
        }
    }

    pub fn parse(&mut self) -> Program {
        let json: Value = serde_json::from_str(&self.input).unwrap();
        self.parse_value(&json);
        self.program_builder.build()
    }

    pub fn parse_function(&mut self, value: &Value) -> Function {
        let mut function_builder = FunctionBuilder::default();
        let name = value[1].as_str().expect("function name").to_string();
        function_builder
            .id(value[0].as_u64().expect("function id") as usize)
            .name(name.clone())
            .args(value[2].as_u64().expect("function args") as usize)
            .locals(value[3].as_u64().expect("function locals") as usize)
            .frame(self.parse_frame(&value[4], name));
        return function_builder.build();
    }

    pub fn parse_frame(&mut self, value: &Value, name: String) -> Frame {
        let mut frame_builder = FrameBuilder::default();
        frame_builder
            .id(value[0].as_u64().expect("frame id") as usize)
            .name(name)
            // .function_id(value[1].as_u64().expect("frame function id") as usize)
            // .pc(value[2].as_u64().expect("frame pc") as usize)
            .data(self.parse_frame_data(&value[3]));
        return frame_builder.build();
    }

    pub fn parse_frame_data(&mut self, value: &Value) -> FrameData {
        let mut frame_data = FrameData::default();
        for (key, value) in value.as_object().unwrap() {
            match key.as_str() {
                "type" => frame_data = frame_data.type_(value.as_str().unwrap().to_string()),
                "value" => self.parse_value(value),
                _ => panic!("Unknown key: {}", key),
            }
        }
        frame_data
    }

    pub fn parse_value(&mut self, value: &Value) -> FrameData {
        self.current_value = Some(value.clone());
        match (value, self.previous_frame_data.clone()) {
            (Value::Number(op), _) => self.parse_number(op.as_i64().expect("number")),
            (Value::Array(val), _) => self.parse_array(val),
            _ => panic!("Invalid value. Expecting a number or an array"),
        }
    }

    pub fn parse_number(&mut self, op: i64) -> FrameData {
        return match (op as i32, self.previous_frame_data.clone()) {
            (n @ 1..=27, None) => {
                FrameData::Instruction(Instruction::get_instruction(n as u32), vec![])
            },
            (n @ -111..=-101, Some(FrameData::Instruction(Instruction::Call { .. }, ..))) => {
                FrameData::Callable(Callable::get_native(n), vec![])
            },
            (n, _) => FrameData::Primitive(n)
        }
    }

    pub fn parse_array(&mut self, array: &Vec<Value>) -> FrameData {
        for value in array {
            self.parse_value(value);
        }

        let mut frame_data = FrameData::default();
        frame_data
    }
}