use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeFunction {
    IPrint { id: i32, name: String, args: u32 },
    SPrint { id: i32, name: String, args: u32 },
    IRead { id: i32, name: String, args: u32 },
    SRead { id: i32, name: String, args: u32 },
    NL { id: i32, name: String, args: u32 },
    Random { id: i32, name: String, args: u32 },
    Timer { id: i32, name: String, args: u32 },
    StopTimer { id: i32, name: String, args: u32 },
    Alloc { id: i32, name: String, args: u32 },
    Free { id: i32, name: String, args: u32 },
    I2S { id: i32, name: String, args: u32 },
    Unknown(i32),
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl NativeFunction {
    pub fn get_native(id: i32) -> Self {
        match id {
            -101 => NativeFunction::IPrint {
                id,
                name: "iprint".to_string(),
                args: 1,
            },
            -102 => NativeFunction::SPrint {
                id,
                name: "sprint".to_string(),
                args: 1,
            },
            -103 => NativeFunction::IRead {
                id,
                name: "iread".to_string(),
                args: 1,
            },
            -104 => NativeFunction::SRead {
                id,
                name: "sread".to_string(),
                args: 2,
            },
            -105 => NativeFunction::NL {
                id,
                name: "nl".to_string(),
                args: 0,
            },
            -106 => NativeFunction::Random {
                id,
                name: "random".to_string(),
                args: 1,
            },
            -107 => NativeFunction::Timer {
                id,
                name: "timer".to_string(),
                args: 2,
            },
            -108 => NativeFunction::StopTimer {
                id,
                name: "stoptimer".to_string(),
                args: 1,
            },
            -109 => NativeFunction::Alloc {
                id,
                name: "alloc".to_string(),
                args: 1,
            },
            -110 => NativeFunction::Free {
                id,
                name: "free".to_string(),
                args: 1,
            },
            -111 => NativeFunction::I2S {
                id,
                name: "i2s".to_string(),
                args: 1,
            },
            n => NativeFunction::Unknown(n),
        }
    }

    pub fn id(&self) -> i32 {
        match self {
            NativeFunction::IPrint { id, .. } => *id,
            NativeFunction::SPrint { id, .. } => *id,
            NativeFunction::IRead { id, .. } => *id,
            NativeFunction::SRead { id, .. } => *id,
            NativeFunction::NL { id, .. } => *id,
            NativeFunction::Random { id, .. } => *id,
            NativeFunction::Timer { id, .. } => *id,
            NativeFunction::StopTimer { id, .. } => *id,
            NativeFunction::Alloc { id, .. } => *id,
            NativeFunction::Free { id, .. } => *id,
            NativeFunction::I2S { id, .. } => *id,
            NativeFunction::Unknown(n) => *n,
        }
    }

    pub fn name(&self) -> String {
        match self {
            NativeFunction::IPrint { name, .. } => name.clone(),
            NativeFunction::SPrint { name, .. } => name.clone(),
            NativeFunction::IRead { name, .. } => name.clone(),
            NativeFunction::SRead { name, .. } => name.clone(),
            NativeFunction::NL { name, .. } => name.clone(),
            NativeFunction::Random { name, .. } => name.clone(),
            NativeFunction::Timer { name, .. } => name.clone(),
            NativeFunction::StopTimer { name, .. } => name.clone(),
            NativeFunction::Alloc { name, .. } => name.clone(),
            NativeFunction::Free { name, .. } => name.clone(),
            NativeFunction::I2S { name, .. } => name.clone(),
            NativeFunction::Unknown(n) => format!("Unknown({})", n),
        }
    }
}
