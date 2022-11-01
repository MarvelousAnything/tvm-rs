use std::fmt::Debug;
use crate::function::Function;
use crate::native::NativeFunction;
use crate::tvm::Tvm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callable {
    Function(Function),
    NativeFunction(NativeFunction),
}

pub trait Caller: Debug + Clone {
    fn call(callable: Callable);
}

impl Caller for Tvm {
    fn call(callable: Callable) {
        match callable {
            Callable::Function(function) => {
                println!("Calling function: {:?}", function);
            },
            Callable::NativeFunction(native_function) => {
                println!("Calling native function: {:?}", native_function);
            },
        }
    }
}