use serde_json::Value;

pub enum Instruction {
    Push(i32),
    Fetch(i32),
    Store,
    IF(Vec<Value>),
    Loop(Vec<Value>),
    Break,
    Return,
    Call(i32),
    FPPlus,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    And,
    OR,
    Xor,
    EQ,
    Neq,
    LT,
    Leq,
    GT,
    Geq,
    Pop,
    LShift,
    RShift,
    Unknown
}

impl Instruction {
    pub fn get_instruction(op_code: i32, addr: i32, frame: &[Value]) -> Instruction {
        match op_code {
            1 => Instruction::Push(frame[addr as usize].as_i64().unwrap() as i32),
            2 => Instruction::Fetch(frame[addr as usize].as_i64().unwrap() as i32),
            3 => Instruction::Store,
            4 => Instruction::IF(frame[addr as usize].as_array().unwrap().to_vec()),
            5 => Instruction::Loop(frame[addr as usize].as_array().unwrap().to_vec()),
            6 => Instruction::Break,
            7 => Instruction::Return,
            8 => Instruction::Call(frame[addr as usize].as_i64().unwrap() as i32),
            9 => Instruction::FPPlus,
            10 => Instruction::Add,
            11 => Instruction::Sub,
            12 => Instruction::Mul,
            13 => Instruction::Div,
            14 => Instruction::Mod,
            15 => Instruction::Not,
            16 => Instruction::And,
            17 => Instruction::OR,
            18 => Instruction::Xor,
            19 => Instruction::EQ,
            20 => Instruction::Neq,
            21 => Instruction::LT,
            22 => Instruction::Leq,
            23 => Instruction::GT,
            24 => Instruction::Geq,
            25 => Instruction::Pop,
            26 => Instruction::LShift,
            27 => Instruction::RShift,
            _ => Instruction::Unknown
        }
    }
}