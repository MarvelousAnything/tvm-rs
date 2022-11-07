use crate::program::Program;
use crate::state::{StateHolder, TvmState};
use crate::tvm::Tvm;

mod callable;
mod frame;
mod function;
mod heap;
mod instruction;
mod instruction_tests;
mod native;
mod program;
mod stack;
mod state;
mod tvm;

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    let mut tvm = Tvm::default();
    let program = Program::from_file("sq.json".to_string());
    println!("{}", program);
    tvm.load(program);
    println!("{}", tvm);
    tvm.start();
    while !tvm.is_halted() {
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if s.trim() == "? state" {
            println!("{:#?}", tvm.state);
        } else if s.trim() == "state" {
            println!("{}", tvm.state);
        } else if s.trim() == "? memory" {
            println!("{}", tvm.get_active_memory_string());
        } else if s.trim() == "? program" {
            println!("{}", tvm.program);
        } else if s.trim() == "? stdout" {
            println!("{}", tvm.stdout);
        } else if s.trim() == "? all" {
            println!("{}", tvm);
        } else if !s.is_empty() {
            print!("Tick {}:\t", tvm.ticks);
            tvm.tick();
        }
        s.clear();
    }
}
