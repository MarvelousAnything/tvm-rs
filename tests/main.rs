#[cfg(test)]
use std::println; // Workaround to use prinltn! for logs.
use tvm::frame::Frame;
use tvm::function::Function;
use tvm::instruction::Instruction;
use tvm::program::Program;
use tvm::stack::StackHolder;
use tvm::state::StateHolder;
use tvm::state::TvmState;
use tvm::tvm::Tvm;

fn get_test_program() -> Program {
    Program::builder()
        .entry_point(0)
        .function(
            Function::builder()
                .id(0)
                .name("init".to_string())
                .locals(1)
                .frame(
                    Frame::builder()
                        .id(0)
                        .name("init".to_string())
                        .instruction(Instruction::Push)
                        .primitive(1)
                        .instruction(Instruction::FPPlus)
                        .instruction(Instruction::Push)
                        .primitive(1)
                        .instruction(Instruction::Store)
                        .instruction(Instruction::Loop)
                        .frame(
                            Frame::builder()
                                .id(1)
                                .instruction(Instruction::Push)
                                .primitive(1)
                                .instruction(Instruction::FPPlus)
                                .instruction(Instruction::Fetch)
                                .instruction(Instruction::Call)
                                .primitive(-101)
                                .instruction(Instruction::Pop)
                                .instruction(Instruction::Push)
                                .primitive(1)
                                .instruction(Instruction::FPPlus)
                                .instruction(Instruction::Fetch)
                                .instruction(Instruction::Push)
                                .primitive(10)
                                .instruction(Instruction::Geq)
                                .instruction(Instruction::Break)
                                .instruction(Instruction::Push)
                                .primitive(1)
                                .instruction(Instruction::FPPlus)
                                .instruction(Instruction::Push)
                                .primitive(1)
                                .instruction(Instruction::FPPlus)
                                .instruction(Instruction::Fetch)
                                .instruction(Instruction::Push)
                                .primitive(1)
                                .instruction(Instruction::Add)
                                .instruction(Instruction::Store)
                                .build(),
                        )
                        .instruction(Instruction::Push)
                        .primitive(0)
                        .build(),
                )
                .build(),
        )
        .build()
}

fn get_tabs(state: &TvmState) -> String {
    let mut out = String::new();
    for _ in 0..state.get_depth() {
        out.push('\t');
    }
    out
}

#[test]
fn test_loops() {
    let mut tvm = Tvm::default();
    tvm.load(get_test_program());
    tvm.start();
    while !tvm.is_halted() {
        tvm.tick();
        if tvm.ticks > 500 {
            panic!("Tvm is stuck in an infinite loop");
        }
    }

    println!("Tvm ticks: {}", tvm.ticks);
    println!("Tvm stdout: {}", tvm.stdout);
    println!("Tvm stack: {:?}", tvm.get_stack());
    // pretty print tvm state history
    for state in &tvm.state_history {
        println!("{}{}", get_tabs(state), state.get_name());
    }

    assert!(tvm.is_halted());
    assert!(matches!(tvm.state, TvmState::Halt(_)));
    assert_eq!(tvm.stdout, "12345678910");
}

#[test]
fn test_sq() {
    let mut tvm = Tvm::default();
    let program = Program::from_file("sq.json".to_string());
    tvm.load(program);
    tvm.start();
    while !tvm.is_halted() {
        tvm.tick();
        if tvm.ticks > 10000 {
            panic!("Tvm is stuck in an infinite loop");
        }
    }

    println!("Tvm ticks: {}", tvm.ticks);
    println!("Tvm stdout: {}", tvm.stdout);
    println!("Tvm stack: {:?}", tvm.get_stack());

    assert!(tvm.is_halted());
    assert!(matches!(tvm.state, TvmState::Halt(_)));
}
