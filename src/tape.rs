pub struct Tape {
    pub bootstrapper: (i32, i32),
    pub tape: Vec<(i32, i32)>,
}

pub struct TapeFunction {
    pub id: i32,
    pub name: String,
    pub arguments: u32,
    pub main: bool
}