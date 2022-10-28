#[derive(Debug, Clone)]
pub enum BuiltIn {
    IPrint{id: i32, name: String, args: u32},
    SPrint{id: i32, name: String, args: u32},
    IRead{id: i32, name: String, args: u32},
    SRead{id: i32, name: String, args: u32},
    NL{id: i32, name: String, args: u32},
    Random{id: i32, name: String, args: u32},
    Timer{id: i32, name: String, args: u32},
    StopTimer{id: i32, name: String, args: u32},
    Alloc{id: i32, name: String, args: u32},
    Free{id: i32, name: String, args: u32},
    I2S{id: i32, name: String, args: u32},
    Unknown(i32),
}

impl BuiltIn {
    pub fn get_builtin(id: i32) -> BuiltIn {
        match id {
            -101 => BuiltIn::IPrint { id, name: "iprint".to_string(), args: 1 },
            -102 => BuiltIn::SPrint { id, name: "sprint".to_string(), args: 1 },
            -103 => BuiltIn::IRead { id, name: "iread".to_string(), args: 1 },
            -104 => BuiltIn::SRead { id, name: "sread".to_string(), args: 2 },
            -105 => BuiltIn::NL { id, name: "nl".to_string(), args: 0 },
            -106 => BuiltIn::Random { id, name: "random".to_string(), args: 1 },
            -107 => BuiltIn::Timer { id, name: "timer".to_string(), args: 2 },
            -108 => BuiltIn::StopTimer { id, name: "stoptimer".to_string(), args: 1 },
            -109 => BuiltIn::Alloc { id, name: "alloc".to_string(), args: 1 },
            -110 => BuiltIn::Free { id, name: "free".to_string(), args: 1 },
            -111 => BuiltIn::I2S { id, name: "i2s".to_string(), args: 1 },
            n => BuiltIn::Unknown(n)
        }
    }

    pub fn get_id(&self) -> i32 {
        match self {
            BuiltIn::IPrint { id, .. } => *id,
            BuiltIn::SPrint { id, .. } => *id,
            BuiltIn::IRead { id, .. } => *id,
            BuiltIn::SRead { id, .. } => *id,
            BuiltIn::NL { id, .. } => *id,
            BuiltIn::Random { id, .. } => *id,
            BuiltIn::Timer { id, .. } => *id,
            BuiltIn::StopTimer { id, .. } => *id,
            BuiltIn::Alloc { id, .. } => *id,
            BuiltIn::Free { id, .. } => *id,
            BuiltIn::I2S { id, .. } => *id,
            BuiltIn::Unknown(n) => *n,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            BuiltIn::IPrint { name, .. } => name.clone(),
            BuiltIn::SPrint { name, .. } => name.clone(),
            BuiltIn::IRead { name, .. } => name.clone(),
            BuiltIn::SRead { name, .. } => name.clone(),
            BuiltIn::NL { name, .. } => name.clone(),
            BuiltIn::Random { name, .. } => name.clone(),
            BuiltIn::Timer { name, .. } => name.clone(),
            BuiltIn::StopTimer { name, .. } => name.clone(),
            BuiltIn::Alloc { name, .. } => name.clone(),
            BuiltIn::Free { name, .. } => name.clone(),
            BuiltIn::I2S { name, .. } => name.clone(),
            BuiltIn::Unknown(n) => n.to_string(),
        }
    }
}