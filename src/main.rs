use tvm_rs::vm::program::Program;
use tvm_rs::vm::tvm::Tvm;

fn main() {
    let program = Program::from_file("sq.json".to_string());
    // println!("{}", program);
    let mut tvm = Tvm::new(program);
    tvm.start();
    println!("{}", tvm.stdout);
}
