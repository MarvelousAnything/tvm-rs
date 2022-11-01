use crate::tvm::Tvm;

mod tvm;
mod state;
mod stack;
mod program;
mod heap;
mod callable;
mod frame;
mod native;
mod function;
mod instruction;

fn main() {
    let tvm = Tvm::default();
    println!("{:?}", tvm);
}
