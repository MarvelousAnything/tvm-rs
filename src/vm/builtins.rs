use crate::vm::instruction::Instruction;

pub enum BuiltIn {
    IPrint,
    SPrint,
    IRead,
    SRead,
    NL,
    Random,
    Timer,
    StopTimer,
    Alloc,
    Free,
    I2S,
    Instruction(Instruction),
    Unknown(i32),
}

impl BuiltIn {
    pub fn get_builtin(op: i32) -> BuiltIn {
        match op {
            -101 => BuiltIn::IPrint,
            -102 => BuiltIn::SPrint,
            -103 => BuiltIn::IRead,
            -104 => BuiltIn::SRead,
            -105 => BuiltIn::NL,
            -106 => BuiltIn::Random,
            -107 => BuiltIn::Timer,
            -108 => BuiltIn::StopTimer,
            -109 => BuiltIn::Alloc,
            -110 => BuiltIn::Free,
            -111 => BuiltIn::I2S,
            n => BuiltIn::Instruction(Instruction::get_instruction(n)),
        }
    }
}