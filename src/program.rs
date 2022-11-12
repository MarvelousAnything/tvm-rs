use crate::callable::Callable;

use crate::frame::{Frame, FrameData};
use crate::function::Function;
use crate::instruction::Instruction;
use serde_json::Value;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub entry_point: usize,
    pub heap_size: usize,
    pub heap: Vec<(usize, i32)>,
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new(
        entry_point: usize,
        heap_size: usize,
        heap: Vec<(usize, i32)>,
        functions: Vec<Function>,
    ) -> Program {
        Program {
            entry_point,
            heap_size,
            heap,
            functions,
        }
    }

    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::new()
    }

    pub fn framedata_from_json(json: &Value) -> FrameData {
        match json {
            Value::Number(op) => match op.as_i64().unwrap() as i32 {
                n @ -111..=-101 => FrameData::Callable(Callable::get_native(n), vec![]),
                n @ 1..=27 => {
                    FrameData::Instruction(Instruction::get_instruction(n as u32), vec![])
                }
                n => FrameData::Primitive(n),
            },
            Value::Array(val) => {
                FrameData::Frame(val.iter().map(Program::framedata_from_json).collect())
            }
            _ => panic!("Invalid frame data"),
        }
    }

    pub fn function_from_json(json: &Value) -> Function {
        let id = json[0].as_u64().expect("frame id") as usize;
        let name = json[1].as_str().expect("frame name").to_string();
        let args = json[2].as_u64().unwrap() as usize;
        let locals = json[3].as_u64().unwrap() as usize;
        let frame_data: Vec<FrameData> = json[4]
            .as_array()
            .unwrap()
            .iter()
            .map(Self::framedata_from_json)
            .collect();
        let mut frame_name = name.clone();
        frame_name.push_str("-frame");
        Function {
            id,
            name,
            args,
            locals,
            frame: Frame {
                id,
                name: frame_name,
                data: frame_data,
                pc: 0,
            },
        }
    }

    pub fn from_file(file: String) -> Program {
        let tape = fs::read_to_string(file).expect("Unable to read file");
        let json: Value = serde_json::from_str(&tape).expect("Unable to parse json");
        let allocation = json[0].as_array().unwrap();
        let heap = json[1].as_array().unwrap();
        let functions: Vec<Function> = json.as_array().unwrap()[2..]
            .iter()
            .map(Self::function_from_json)
            .collect();
        Program {
            entry_point: allocation[0].as_u64().unwrap() as usize,
            heap_size: allocation[1].as_u64().unwrap() as usize,
            heap: heap
                .iter()
                .map(|x| {
                    (
                        x[0].as_u64().unwrap() as usize,
                        x[1].as_i64().unwrap() as i32,
                    )
                })
                .collect(),
            functions,
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program {{")?;
        writeln!(f, "\tentry_point: {},", self.entry_point)?;
        writeln!(f, "\theap_size: {},", self.heap_size)?;
        writeln!(f, "\theap: [")?;
        for (i, (addr, val)) in self.heap.iter().enumerate() {
            write!(f, "\t\t({}, {})", addr, val)?;
            if i != self.heap.len() - 1 {
                writeln!(f, ",")?;
            }
        }
        writeln!(f, "\n\tfunctions: [")?;
        for (i, func) in self.functions.iter().enumerate() {
            write!(f, "\t\t{}", func)?;
            if i != self.functions.len() - 1 {
                writeln!(f, ",")?;
            }
        }
        write!(f, "\n\t],\n")
    }
}

#[derive(Default)]
pub struct ProgramBuilder {
    entry_point: usize,
    heap: Vec<(usize, i32)>,
    functions: Vec<Function>,
}

impl ProgramBuilder {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder::default()
    }

    pub fn entry_point(mut self, entry_point: usize) -> ProgramBuilder {
        self.entry_point = entry_point;
        self
    }

    pub fn heap(mut self, heap: Vec<(usize, i32)>) -> ProgramBuilder {
        self.heap = heap;
        self
    }

    pub fn function(mut self, function: Function) -> ProgramBuilder {
        self.functions.push(function);
        self
    }

    pub fn build(self) -> Program {
        Program::new(self.entry_point, self.heap.len(), self.heap, self.functions)
    }
}
