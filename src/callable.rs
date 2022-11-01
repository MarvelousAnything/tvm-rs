use std::fmt::Debug;
use crate::function::Function;
use crate::native::NativeFunction;
use crate::stack::StackHolder;
use crate::tvm::Tvm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callable {
    Function(Function),
    Native(NativeFunction),
}

impl Callable {
    pub fn get_callable(id: i32) -> Self {
        match id {
            n if n < -111 => panic!("Invalid callable id: {}", n),
            n @ -111..=-101 => Callable::Native(NativeFunction::get_native(n)),
            n if n >= 0 => Callable::Function(Function::get_function(n as usize)),
            _ => unreachable!(),
        }
    }

    pub fn get_id(&self) -> i32 {
        match self {
            Callable::Function(function) => function.id as i32,
            Callable::Native(native) => native.get_id(),
        }
    }
}

pub trait Caller: Debug + Clone {
    fn do_call(&mut self, callable: Callable);
}

impl Caller for Tvm {
    fn do_call(&mut self, callable: Callable) {
        match callable {
            Callable::Function(function) => {
                println!("Calling function: {:?}", function);
                self.push(0);
            },
            Callable::Native(native_function) => {

                match native_function {
                    NativeFunction::IPrint { .. } => {
                        let value = self.pop();
                        println!("stdout: {}", value);
                        self.push(0);
                    },
                    NativeFunction::Alloc { .. } => {
                        let size = self.pop();
                        self.push(self.heap_size as i32);
                        self.heap_size += size as usize;
                        println!("Allocating {} bytes", size);
                    },
                    _ => println!("Calling native function: {:?}", native_function),
                }
            },
        }
    }
}