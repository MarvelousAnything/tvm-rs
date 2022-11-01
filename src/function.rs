use crate::frame::Frame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub id: usize,
    pub name: String,
    pub args: usize,
    pub locals: usize,
    pub frame: Frame,
}