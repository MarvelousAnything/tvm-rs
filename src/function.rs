use crate::frame::Frame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub id: usize,
    pub name: String,
    pub args: usize,
    pub locals: usize,
    pub frame: Frame,
}

impl Function {
    pub(crate) fn get_function(id: usize) -> Function {
        Function {
            id,
            name: "test-function".to_string(),
            args: 0,
            locals: 0,
            frame: Frame {
                id: 0,
                name: "test-frame".to_string(),
                data: vec![],
                pc: 0,
                previous_frame: None,
                result: None,
            }
        }
    }
}