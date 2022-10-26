pub enum BuiltIn {
    IPrint,
    SPrint,
    IRead,
    SRead,
    NL,
    Random,
    Timer,
    STOPTIMER,
    ALLOC,
    FREE,
    I2S,
    UNKNOWN
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
            -108 => BuiltIn::STOPTIMER,
            -109 => BuiltIn::ALLOC,
            -110 => BuiltIn::FREE,
            -111 => BuiltIn::I2S,
            _ => BuiltIn::UNKNOWN
        }
    }
}