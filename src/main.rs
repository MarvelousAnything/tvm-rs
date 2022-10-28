use tvm_rs::vm::program::Program;
use tvm_rs::vm::tvm::Tvm;

fn main() {
    let program = Program::from_file("sq.json".to_string());
    let tvm = Tvm::new(program);
    println!("{:?}", tvm);

}
