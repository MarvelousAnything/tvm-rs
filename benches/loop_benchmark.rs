use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tvm::frame::Frame;
use tvm::function::Function;
use tvm::instruction::Instruction;
use tvm::program::Program;
use tvm::state::StateHolder;
use tvm::tvm::Tvm;

fn get_program(n: i32) -> Program {
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
                                .primitive(n)
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

fn run_program(tvm: &mut Tvm) {
    while !tvm.is_halted() {
        tvm.tick();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_benchmark");
    for n in [5, 10, 25, 50, 100, 200, 500, 1000].iter() {
        let mut tvm = Tvm::default();
        let program = get_program(*n);
        tvm.load(program);
        tvm.start();
        group.throughput(Throughput::Bytes(*n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| run_program(&mut tvm));
        });
    }
    group.finish();
}

fn benchmark_sq(c: &mut Criterion) {
    let mut tvm = Tvm::default();
    let program = Program::from_file("sq.json".to_string());
    tvm.load(program);
    tvm.start();
    c.bench_function("sq", |b| b.iter(|| run_program(&mut tvm)));
}

criterion_group!(benches, criterion_benchmark, benchmark_sq);
criterion_main!(benches);
