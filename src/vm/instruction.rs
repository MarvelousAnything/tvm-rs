#[derive(Debug, Clone)]
pub enum Instruction {
    Push { op: u32, name: String },
    Fetch { op: u32, name: String },
    Store { op: u32, name: String },
    IF { op: u32, name: String },
    Loop { op: u32, name: String },
    Break { op: u32, name: String },
    Return { op: u32, name: String },
    Call { op: u32, name: String },
    FPPlus { op: u32, name: String },
    Add { op: u32, name: String },
    Sub { op: u32, name: String },
    Mul { op: u32, name: String },
    Div { op: u32, name: String },
    Mod { op: u32, name: String },
    Not { op: u32, name: String },
    And { op: u32, name: String },
    OR { op: u32, name: String },
    Xor { op: u32, name: String },
    EQ { op: u32, name: String },
    Neq { op: u32, name: String },
    LT { op: u32, name: String },
    Leq { op: u32, name: String },
    GT { op: u32, name: String },
    Geq { op: u32, name: String },
    Pop { op: u32, name: String },
    LShift { op: u32, name: String },
    RShift { op: u32, name: String },
    Unknown(u32),
}

impl Instruction {
    pub fn get_instruction(op_code: u32) -> Instruction {
        match op_code {
            1 => Instruction::Push {
                op: 1,
                name: "push".to_string(),
            },
            2 => Instruction::Fetch {
                op: 2,
                name: "fetch".to_string(),
            },
            3 => Instruction::Store {
                op: 3,
                name: "store".to_string(),
            },
            4 => Instruction::IF {
                op: 4,
                name: "if".to_string(),
            },
            5 => Instruction::Loop {
                op: 5,
                name: "loop".to_string(),
            },
            6 => Instruction::Break {
                op: 6,
                name: "break".to_string(),
            },
            7 => Instruction::Return {
                op: 7,
                name: "return".to_string(),
            },
            8 => Instruction::Call {
                op: 8,
                name: "call".to_string(),
            },
            9 => Instruction::FPPlus {
                op: 9,
                name: "fp+".to_string(),
            },
            10 => Instruction::Add {
                op: 10,
                name: "+".to_string(),
            },
            11 => Instruction::Sub {
                op: 11,
                name: "-".to_string(),
            },
            12 => Instruction::Mul {
                op: 12,
                name: "*".to_string(),
            },
            13 => Instruction::Div {
                op: 13,
                name: "/".to_string(),
            },
            14 => Instruction::Mod {
                op: 14,
                name: "%".to_string(),
            },
            15 => Instruction::Not {
                op: 15,
                name: "!".to_string(),
            },
            16 => Instruction::And {
                op: 16,
                name: "&".to_string(),
            },
            17 => Instruction::OR {
                op: 17,
                name: "|".to_string(),
            },
            18 => Instruction::Xor {
                op: 18,
                name: "^".to_string(),
            },
            19 => Instruction::EQ {
                op: 19,
                name: "==".to_string(),
            },
            20 => Instruction::Neq {
                op: 20,
                name: "!=".to_string(),
            },
            21 => Instruction::LT {
                op: 21,
                name: "<".to_string(),
            },
            22 => Instruction::Leq {
                op: 22,
                name: "<=".to_string(),
            },
            23 => Instruction::GT {
                op: 23,
                name: ">".to_string(),
            },
            24 => Instruction::Geq {
                op: 24,
                name: ">=".to_string(),
            },
            25 => Instruction::Pop {
                op: 25,
                name: "pop".to_string(),
            },
            26 => Instruction::LShift {
                op: 26,
                name: "<<".to_string(),
            },
            27 => Instruction::RShift {
                op: 27,
                name: ">>".to_string(),
            },
            _ => Instruction::Unknown(op_code),
        }
    }

    pub fn get_op(&self) -> u32 {
        match self {
            Instruction::Push { op, .. } => *op,
            Instruction::Fetch { op, .. } => *op,
            Instruction::Store { op, .. } => *op,
            Instruction::IF { op, .. } => *op,
            Instruction::Loop { op, .. } => *op,
            Instruction::Break { op, .. } => *op,
            Instruction::Return { op, .. } => *op,
            Instruction::Call { op, .. } => *op,
            Instruction::FPPlus { op, .. } => *op,
            Instruction::Add { op, .. } => *op,
            Instruction::Sub { op, .. } => *op,
            Instruction::Mul { op, .. } => *op,
            Instruction::Div { op, .. } => *op,
            Instruction::Mod { op, .. } => *op,
            Instruction::Not { op, .. } => *op,
            Instruction::And { op, .. } => *op,
            Instruction::OR { op, .. } => *op,
            Instruction::Xor { op, .. } => *op,
            Instruction::EQ { op, .. } => *op,
            Instruction::Neq { op, .. } => *op,
            Instruction::LT { op, .. } => *op,
            Instruction::Leq { op, .. } => *op,
            Instruction::GT { op, .. } => *op,
            Instruction::Geq { op, .. } => *op,
            Instruction::Pop { op, .. } => *op,
            Instruction::LShift { op, .. } => *op,
            Instruction::RShift { op, .. } => *op,
            Instruction::Unknown(op) => *op,
        }
    }
}
