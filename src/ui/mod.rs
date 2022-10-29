use tui::backend::Backend;
use tui::Frame;
use crate::vm::tvm::Tvm;

mod inputs;
mod widget;

pub fn draw_memory<B>(f: &mut Frame<B>, tvm: &Tvm)
    where
        B: Backend, {

}