/*
    A program consists of:
      - a tuple with the frame id for the init function and the size of the heap.
      - a list of tuples with the location on the heap and the value.
      - Frames
 */
use std::fmt::Display;
use std::fs;
use serde_json::Value;
use crate::vm::function::Function;

#[derive(Debug)]
pub struct Program {
    pub entry_point: usize,
    pub heap_size: usize,
    pub heap: Vec<(usize, i32)>,
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new(entry_point: usize, heap_size: usize, heap: Vec<(usize, i32)>, functions: Vec<Function>) -> Program {
        Program {
            entry_point,
            heap_size,
            heap,
            functions,
        }
    }

    pub fn from_file(file: String) -> Program {
        let tape = fs::read_to_string(file).expect("Unable to read file");
        let json: Value = serde_json::from_str(&tape).expect("Unable to parse json");
        let allocation = json[0].as_array().unwrap();
        let heap = json[1].as_array().unwrap();
        let functions: Vec<Function> = json.as_array().unwrap()[2..].iter().map(Function::from_json).collect();
        Program {
            entry_point: allocation[0].as_u64().unwrap() as usize,
            heap_size: allocation[1].as_u64().unwrap() as usize,
            heap: heap.iter().map(|x| (x[0].as_u64().unwrap() as usize, x[1].as_i64().unwrap() as i32)).collect(),
            functions,
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program {{ \nentry_point: {}, \nheap_size: {}, \nfunctions: {:?} }}", self.entry_point, self.heap_size, self.functions)
    }
}