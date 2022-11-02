use crate::state::{Stateful, TvmState};
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
    use std::io::{ stdin ,stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    let mut tvm = Tvm::default();
    tvm.start();
    while !tvm.state.is_halted() {
        // stdin().read_line(&mut s).expect("Did not enter a correct string");
        // if !s.is_empty() {
        //     print!("Tick {}:\t\t", tvm.ticks);
        //
        // }
        tvm.tick();
    }
}
