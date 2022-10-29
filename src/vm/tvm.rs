use std::any::Any;
use crate::vm::builtins::BuiltIn;
use crate::vm::function::{Frame, FrameData, Function};
use crate::vm::instruction::Instruction;
use crate::vm::memory::Memory;
use crate::vm::program::Program;
use rand::Rng;
use std::borrow::Borrow;
use std::cell::Ref;
use crate::vm::state::{CallState, EvalState, EvalFrameState, State, PausedState, Waiting, TvmState};

const SLEEP_TIME: u64 = 1;

#[derive(Debug)]
pub struct Tvm {
    pub memory: Memory,
    pub program: Program,
    pub depth: usize,
    pub frame_id_counter: usize,
    pub stdout: String,
    pub state: TvmState<'static>,
    pub paused: bool,
}

pub enum Callable {
    Function(Function),
    BuiltIn(BuiltIn),
}

impl Callable {
    fn get_name(&self) -> String {
        match self {
            Callable::Function(f) => f.name.clone(),
            Callable::BuiltIn(b) => b.get_name(),
        }
    }
}

impl Tvm {
    pub fn new(program: Program) -> Tvm {
        let mut memory = Memory::default();
        for (location, value) in &program.heap {
            memory[*location] = *value;
        }
        Tvm {
            memory,
            program,
            depth: 0,
            frame_id_counter: 0,
            stdout: String::new(),
            state: Box::new(Waiting),
            paused: false,
        }
    }

    pub fn set_state<T: State<T>>(&mut self, state: Box<dyn State<T>>) {
        self.state = state;
    }

    pub fn start(&mut self) {
        self.call(self.get_callable(self.program.entry_point as i32));
        self.tick();
    }

    pub fn tick(&mut self) {
        self.state.tick(self);
    }

    pub fn pause(&mut self) {
        self.state.pause(self);
    }

    pub fn resume(&mut self) {
        self.state.resume(self);
    }

    fn get_callable(&self, id: i32) -> Callable {
        match id {
            -111..=-101 => Callable::BuiltIn(BuiltIn::get_builtin(id)),
            n if n >= 0 => Callable::Function(self.program.functions[n as usize].clone()),
            _ => panic!("Invalid callable id"),
        }
    }

    fn get_next_frame<'a>(&mut self, data: &'a FrameData) -> &'a Frame {
        match data {
            FrameData::Frame(f) => f,
            FrameData::Function(_) => panic!("Cannot jump to a function"),
            FrameData::BuiltIn(_) => panic!("Cannot jump to a builtin"),
            FrameData::Instruction(_) => panic!("Cannot jump to an instruction"),
            FrameData::Primitive(_) => panic!("Cannot jump to a primitive"),
        }
    }

    fn a2s(&self, a: i32) -> String {
        let mut s = String::new();
        let mut i = a;
        while self.memory[i as usize] != 0 {
            s.push(self.memory[i as usize] as u8 as char);
            i += 1;
        }
        s
    }

    pub fn write_str(&mut self, addr: usize, s: String) {
        let mut x = addr;
        for c in s.chars() {
            self.memory[x] = c as i32;
            x += 1;
        }
        self.memory[x] = 0;
    }

    pub fn call(&mut self, callable: Callable) {
        self.set_state(Box::new(CallState { callable }));
    }

    pub fn do_call(&mut self, callable: Callable) {
        match callable {
            Callable::Function(mut function) => {
                function.frame.frame_id = self.frame_id_counter;
                self.frame_id_counter += 1;
                let frame = function.frame.borrow();
                // expect that arguments have already been pushed to the stack
                // push zero to the stack for the local data
                for _ in 0..function.locals {
                    self.memory.push(0);
                }
                // push the frame pointer to the stack
                self.memory.push(self.memory.frame_pointer as i32);
                // set the frame pointer to the value of the stack pointer prior to pushing the frame pointer to the stack
                self.memory.frame_pointer = self.memory.stack_pointer + 1;
                // evaluate the function
                self.eval_frame(frame.to_owned());
                // pop the return value from the top of the stack
                let r = self.memory.pop();
                // copy the frame pointer to the stack pointer
                self.memory.stack_pointer = self.memory.frame_pointer;
                self.memory.update_state();
                // copy the top of the stack into the frame pointer
                self.memory.frame_pointer = self.memory.peek() as usize;
                // Increment the stack pointer by the number of local variables and parameters of the function
                self.memory.stack_pointer += (function.locals + function.args) as usize;
                self.memory.update_state();
                // push the return value to the stack
                self.memory.push(r);
            }
            Callable::BuiltIn(builtin) => {
                match builtin {
                    BuiltIn::IPrint { .. } => {
                        let arg = self.memory.pop();
                        self.stdout += &*arg.to_string();
                        self.memory.push(0);
                    }
                    BuiltIn::SPrint { .. } => {
                        let arg = self.memory.pop();
                        self.stdout += &*self.a2s(arg);
                        self.memory.push(0);
                    }
                    BuiltIn::IRead { .. } => {
                        let prompt_addr = self.memory.pop();
                        let mut prompt = String::new();
                        if prompt_addr == -1 {
                            prompt = "Integer input: ".to_string();
                        } else {
                            prompt = self.a2s(prompt_addr);
                        }
                        // print!("{}", prompt);
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        let arg = input.trim().parse::<i32>().expect("Failed to parse input");
                        self.memory.push(arg);
                    }
                    BuiltIn::SRead { .. } => {
                        let addr = self.memory.pop();
                        let prompt_addr = self.memory.pop();
                        let mut prompt = String::new();
                        if prompt_addr == -1 {
                            prompt = "String input: ".to_string();
                        } else {
                            prompt = self.a2s(prompt_addr);
                        }
                        // print!("{}", prompt);
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        let arg = input.trim().to_string();
                        self.write_str(addr as usize, arg);
                        self.memory.push(0);
                    }
                    BuiltIn::NL { .. } => {
                        self.stdout += "\n";
                        self.memory.push(0);
                    }
                    BuiltIn::Random { .. } => {
                        let arg = self.memory.pop();
                        let r = rand::thread_rng().gen_range(0..arg);
                        self.memory.push(r);
                    }
                    BuiltIn::Timer { .. } => {
                        // TODO: implement timer
                        let _ = self.memory.pop();
                        self.memory.push(0);
                    }
                    BuiltIn::StopTimer { .. } => {
                        // TODO: implement stop timer
                        let _ = self.memory.pop();
                        self.memory.push(0);
                    }
                    BuiltIn::Alloc { .. } => {
                        let arg = self.memory.pop();
                        self.memory.push(self.program.heap_size as i32);
                        self.program.heap_size += arg as usize;
                    }
                    BuiltIn::Free { .. } => {
                        let _ = self.memory.pop();
                        self.memory.push(0);
                    }
                    BuiltIn::I2S { .. } => {
                        let arg = self.memory.pop();
                        let addr = self.memory.pop();
                        self.write_str(addr as usize, arg.to_string());
                        self.memory.push(0);
                    }
                    BuiltIn::Unknown(n) => panic!("Unknown builtin {}", n),
                }
            }
        }
    }

    pub fn do_eval_frame(&mut self, frame: &Frame) -> i32 {
        let mut pc = 0;
        self.depth += 1;
        while pc >= 0 {
            pc = self.do_eval(frame, pc);
        }
        self.depth -= 1;
        // // println!("Frame returned");
        pc
    }

    pub fn eval_frame(&mut self, frame: Frame) {
        self.set_state(Box::new(EvalFrameState { frame }));
    }

    pub fn do_eval(&mut self, frame: &Frame, mut pc: i32) -> i32 {
        if pc >= frame.frame_data.len() as i32 {
            return -1;
        }
        // print!("Frame {}{}{}: ", self.depth, "\t".repeat(self.depth), pc);
        let data = &frame.frame_data[pc as usize];
        pc += 1;
        match data {
            FrameData::Instruction(instruction) => {
                match instruction {
                    Instruction::Push { .. } => {
                        let fd = &frame.frame_data[pc as usize];
                        let to_push = match fd {
                            FrameData::Function(Function { id, .. }) => *id as i32,
                            FrameData::BuiltIn(builtin) => builtin.get_id(),
                            FrameData::Instruction(instruction) => instruction.get_op() as i32,
                            FrameData::Primitive(n) => *n,
                            FrameData::Frame(_) => panic!("Cannot push a frame"),
                        };
                        self.memory.push(to_push);
                        // println!("PUSH {}", to_push);
                        pc += 1;
                    }
                    Instruction::Fetch { .. } => {
                        let index = self.memory.pop();
                        self.memory.push(self.memory[index as usize]);
                        // println!("FETCH {} = {}", index, self.memory[index as usize]);
                    }
                    Instruction::Store { .. } => {
                        let value = self.memory.pop();
                        let index = self.memory.pop() as usize;
                        self.memory[index] = value;
                        // println!("STORE {} {}", index, value);
                    }
                    Instruction::IF { .. } => {
                        let arg = self.memory.pop();
                        // println!("IF {}", arg);
                        let fd = &frame.frame_data[pc as usize];
                        let r: i32;
                        let next_frame = self.get_next_frame(fd);
                        if arg != 0 {
                            r = self.do_eval_frame(next_frame);
                            pc += 2;
                        } else {
                            pc += 1;
                            r = self.do_eval_frame(next_frame);
                            pc += 1;
                        }

                        if r <= -2 {
                            return r;
                        }
                    }
                    Instruction::Loop { .. } => {
                        // println!("LOOP");
                        loop {
                            let fd = &frame.frame_data[pc as usize];
                            let next_frame = self.get_next_frame(fd);
                            let r = self.do_eval_frame(next_frame);
                            if r == -2 {
                                break;
                            } else if r == -3 {
                                return -3;
                            }
                        }
                        pc += 1;
                    }
                    Instruction::Break { .. } => {
                        let x = self.memory.pop();
                        // println!("BREAK {}", x);
                        if x != 0 {
                            return -2;
                        }
                    }
                    Instruction::Return { .. } => {
                        // println!("RETURN");
                        return -3;
                    }
                    Instruction::Call { .. } => {
                        let fd = &frame.frame_data[pc as usize];
                        let callable = match fd {
                            FrameData::Function(function) => Callable::Function(function.clone()),
                            FrameData::BuiltIn(builtin) => Callable::BuiltIn(builtin.clone()),
                            FrameData::Primitive(n) => {
                                if (*n >= self.program.heap_size as i32) | (*n < 0) {
                                    panic!("Cannot call primitive {}", n);
                                }
                                Callable::Function(self.program.functions[*n as usize].clone())
                            }
                            FrameData::Instruction(_) => panic!("Cannot call an instruction"),
                            FrameData::Frame(_) => panic!("Cannot call a frame"),
                        };
                        // println!("CALL {}", callable.get_name());
                        self.call(callable);
                        pc += 1;
                    }
                    Instruction::FPPlus { .. } => {
                        let x = self.memory.pop();
                        self.memory.push(self.memory.frame_pointer as i32 + x);
                        // println!("FP+ {} + {}", self.memory.frame_pointer, x);
                    }
                    Instruction::Add { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x + y);
                        // println!("ADD {} {}", x, y);
                    }
                    Instruction::Sub { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x - y);
                        // println!("SUB {} {}", x, y);
                    }
                    Instruction::Mul { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x * y);
                        // println!("MUL {} {}", x, y);
                    }
                    Instruction::Div { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x / y);
                        // println!("DIV {} {}", x, y);
                    }
                    Instruction::Mod { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x % y);
                        // println!("MOD {} {}", x, y);
                    }
                    Instruction::Not { .. } => {
                        let x = self.memory.pop();
                        self.memory.push(!x);
                        // println!("NOT {}", x);
                    }
                    Instruction::And { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x & y);
                        // println!("AND {} {}", x, y);
                    }
                    Instruction::OR { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x | y);
                        // println!("OR {} {}", x, y);
                    }
                    Instruction::Xor { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x ^ y);
                        // println!("XOR {} {}", x, y);
                    }
                    Instruction::EQ { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push((x == y) as i32);
                        // println!("EQ {} {}", x, y);
                    }
                    Instruction::Neq { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push((x != y) as i32);
                        // println!("NEQ {} {}", x, y);
                    }
                    Instruction::LT { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push((x < y) as i32);
                        // println!("LT {} {}", x, y);
                    }
                    Instruction::Leq { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push((x <= y) as i32);
                        // println!("LEQ {} {}", x, y);
                    }
                    Instruction::GT { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push((x > y) as i32);
                        // println!("GT {} {}", x, y);
                    }
                    Instruction::Geq { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push((x >= y) as i32);
                        // println!("GEQ {} {}", x, y);
                    }
                    Instruction::Pop { .. } => {
                        self.memory.pop();
                        // println!("POP");
                    }
                    Instruction::LShift { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x << y);
                        // println!("LSHIFT {} {}", x, y);
                    }
                    Instruction::RShift { .. } => {
                        let y = self.memory.pop();
                        let x = self.memory.pop();
                        self.memory.push(x >> y);
                        // println!("RSHIFT {} {}", x, y);
                    }
                    Instruction::Unknown(op) => panic!("Unknown opcode {}", op),
                }
                pc
            }
            FrameData::Frame(_) => panic!("Cannot evaluate a frame"),
            FrameData::Function(_) | FrameData::BuiltIn(_) => panic!("Cannot evaluate a function"),
            FrameData::Primitive(_) => panic!("Cannot evaluate a primitive"),
        }
    }

    pub fn eval(&mut self, frame: Ref<Frame>, pc: i32) {
        self.set_state(Box::new(EvalState { frame: frame.to_owned(), pc }));
    }
}
