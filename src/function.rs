use crate::frame::Frame;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub id: usize,
    pub name: String,
    pub args: usize,
    pub locals: usize,
    pub frame: Frame,
}

impl Function {
    pub fn builder() -> FunctionBuilder {
        FunctionBuilder::new()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default)]
pub struct FunctionBuilder {
    id: usize,
    name: String,
    args: usize,
    locals: usize,
    frame: Frame,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn args(mut self, args: usize) -> Self {
        self.args = args;
        self
    }

    pub fn locals(mut self, locals: usize) -> Self {
        self.locals = locals;
        self
    }

    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = frame;
        self
    }

    pub fn build(self) -> Function {
        Function {
            id: self.id,
            name: self.name,
            args: self.args,
            locals: self.locals,
            frame: self.frame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_builder() {
        let function = Function::builder()
            .id(1)
            .name("test".to_string())
            .args(2)
            .locals(3)
            .frame(Frame::builder().build())
            .build();
        assert_eq!(function.id, 1);
        assert_eq!(function.name, "test");
        assert_eq!(function.args, 2);
        assert_eq!(function.locals, 3);
        assert_eq!(function.frame, Frame::builder().build());
    }
}
