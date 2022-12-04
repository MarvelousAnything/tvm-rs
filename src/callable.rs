use crate::function::Function;
use crate::native::NativeFunction;
use crate::stack::StackHolder;
use crate::state::StateResult::Return;
use crate::state::{StateHolder, StateResult};
use crate::tvm::Tvm;
use rand::Rng;
use std::fmt::{Debug, Display, Formatter};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callable {
    Function(Function),
    Native(NativeFunction),
}

impl Callable {
    pub fn get_id(&self) -> i32 {
        match self {
            Callable::Function(function) => function.id as i32,
            Callable::Native(native) => native.id(),
        }
    }

    pub fn get_native(id: i32) -> Callable {
        match id {
            n @ -111..=-101 => Callable::Native(NativeFunction::get_native(n)),
            _ => panic!("Invalid native function id"),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Callable::Function(function) => function.name.clone(),
            Callable::Native(native) => native.name(),
        }
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::Function(function) => write!(f, "{}", function),
            Callable::Native(native) => write!(f, "{}", native),
        }
    }
}

pub trait Caller: Debug + Clone {
    fn do_call(&mut self, callable: Callable);
    fn get_callable(&self, id: i32) -> Callable;
    fn handle_function_return(&mut self, callable: Callable);
}

impl Caller for Tvm {
    fn do_call(&mut self, callable: Callable) {
        self.state.set_result(StateResult::Continue);
        match callable {
            Callable::Function(function) => {
                // println!("Calling function: {}", function);
                let frame = function.frame;
                // expect that arguments have already been pushed to the stack
                // push zero to the stack for the local data
                for _ in 0..function.locals {
                    self.push(0);
                }
                // push the frame pointer to the stack
                self.push(self.frame_pointer as i32);
                // set the frame pointer to the value of the stack pointer prior to pushing the frame pointer to the stack
                self.frame_pointer = self.stack_pointer + 1;
                // evaluate the function
                self.frame_eval(frame);
                // // TODO: Implement function return behavior
                // // pop the return value from the top of the stack
                // let r = self.pop();
                // // copy the frame pointer to the stack pointer
                // self.stack_pointer = self.frame_pointer;
                // // copy the top of the stack into the frame pointer
                // self.frame_pointer = self.peek() as usize;
                // // Increment the stack pointer by the number of local variables and parameters of the function
                // self.stack_pointer += (function.locals + function.args) as usize;
                // // push the return value to the stack
                // self.push(r);
            }
            Callable::Native(native_function) => match native_function {
                NativeFunction::IPrint { .. } => {
                    let value = self.pop();
                    // println!("stdout: {}", value);
                    self.stdout.push_str(&value.to_string());
                    self.push(0);
                    self.state.set_result(Return);
                }
                NativeFunction::SPrint { .. } => {
                    let addr = self.pop();
                    let s = self.a2s(addr as usize);
                    // println!("stdout: {}", s);
                    self.stdout.push_str(&s);
                    self.push(0);
                    self.state.set_result(Return);
                }
                NativeFunction::IRead { .. } => {
                    let prompt_addr = self.pop();
                    let mut prompt = String::new();
                    if prompt_addr == -1 {
                        prompt = "Integer input: ".to_string();
                    } else {
                        prompt = self.a2s(prompt_addr as usize);
                    }
                    let mut input = String::new();
                    print!("{}", prompt);
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let arg = input.trim().parse::<i32>().expect("Failed to parse input");
                    self.push(arg);
                    self.state.set_result(Return);
                }
                NativeFunction::SRead { .. } => {
                    let prompt_addr = self.pop();
                    let mut prompt = String::new();
                    if prompt_addr == -1 {
                        prompt = "String input: ".to_string();
                    } else {
                        prompt = self.a2s(prompt_addr as usize);
                    }
                    let mut input = String::new();
                    print!("{}", prompt);
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let arg = input.trim().parse::<i32>().expect("Failed to parse input");
                    self.push(arg);
                    self.state.set_result(Return);
                }
                NativeFunction::NL { .. } => {
                    // println!();
                    self.stdout.push('\n');
                    self.push(0);
                    self.state.set_result(Return);
                }
                NativeFunction::Random { .. } => {
                    let n = self.pop();
                    let r = rand::thread_rng().gen_range(0..n);
                    self.push(r);
                    self.state.set_result(Return);
                }
                NativeFunction::Timer { .. } => {
                    let id = self.pop();
                    let time = self.pop();
                    // println!("Timer {} set to {}", id, time);
                    self.push(0);
                    self.state.set_result(Return);
                }
                NativeFunction::StopTimer { .. } => {
                    let id = self.pop();
                    let time = self.pop();
                    // println!("Timer {} stopped at {}", id, time);
                    self.push(0);
                    self.state.set_result(Return);
                }
                NativeFunction::Alloc { .. } => {
                    let size = self.pop();
                    self.push(self.heap_size as i32);
                    self.heap_size += size as usize;
                    // println!("Allocating {} bytes", size);
                    self.state.set_result(Return);
                }
                NativeFunction::Free { .. } => {
                    let addr = self.pop();
                    // println!("Freeing {}", addr);
                    self.push(0);
                    self.state.set_result(Return);
                }
                NativeFunction::I2S { .. } => {
                    let arg = self.pop();
                    let addr = self.pop();
                    self.write_string(addr as usize, arg.to_string());
                    self.push(0);
                    self.state.set_result(Return);
                }
                _ => {
                    // println!("Calling native function: {:?}", native_function);
                },
            },
        }
    }

    fn get_callable(&self, id: i32) -> Callable {
        match id {
            n if n < -111 => panic!("Invalid callable id: {}", n),
            n @ -111..=-101 => Callable::Native(NativeFunction::get_native(n)),
            n if n >= 0 => Callable::Function(self.get_function(n as usize)),
            _ => unreachable!(),
        }
    }

    fn handle_function_return(&mut self, callable: Callable) {
        match callable {
            Callable::Function(function) => {
                // println!("Returning from function: {}", function);
                let r = self.pop();
                self.stack_pointer = self.frame_pointer;
                self.frame_pointer = self.peek() as usize;
                self.stack_pointer += (function.locals + function.args) as usize;
                self.push(r);
            }
            Callable::Native(_) => {}
        };
    }
}
