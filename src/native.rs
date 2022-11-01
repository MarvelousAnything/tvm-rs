#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFunction {
    pub id: i32,
    pub name: String,
    pub args: usize,
}