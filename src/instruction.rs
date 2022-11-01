use std::fmt::Debug;
use crate::tvm::Tvm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Push { op: u32, name: String, num_operands: u32 },
    Fetch { op: u32, name: String, num_operands: u32 },
    Store { op: u32, name: String, num_operands: u32 },
    IF { op: u32, name: String, num_operands: u32 },
    Loop { op: u32, name: String, num_operands: u32 },
    Break { op: u32, name: String, num_operands: u32 },
    Return { op: u32, name: String, num_operands: u32 },
    Call { op: u32, name: String, num_operands: u32 },
    FPPlus { op: u32, name: String, num_operands: u32 },
    Add { op: u32, name: String, num_operands: u32 },
    Sub { op: u32, name: String, num_operands: u32 },
    Mul { op: u32, name: String, num_operands: u32 },
    Div { op: u32, name: String, num_operands: u32 },
    Mod { op: u32, name: String, num_operands: u32 },
    Not { op: u32, name: String, num_operands: u32 },
    And { op: u32, name: String, num_operands: u32 },
    OR { op: u32, name: String, num_operands: u32 },
    Xor { op: u32, name: String, num_operands: u32 },
    EQ { op: u32, name: String, num_operands: u32 },
    Neq { op: u32, name: String, num_operands: u32 },
    LT { op: u32, name: String, num_operands: u32 },
    Leq { op: u32, name: String, num_operands: u32 },
    GT { op: u32, name: String, num_operands: u32 },
    Geq { op: u32, name: String, num_operands: u32 },
    Pop { op: u32, name: String, num_operands: u32 },
    LShift { op: u32, name: String, num_operands: u32 },
    RShift { op: u32, name: String, num_operands: u32 },
    Unknown(u32),
}

pub trait Evaluator: Debug + Clone {
    fn eval(&mut self, instruction: Instruction);
}

impl Evaluator for Tvm {
    fn eval(&mut self, instruction: Instruction) {
        println!("Evaluating instruction: {:?}", instruction);
    }
}