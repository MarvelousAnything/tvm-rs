use std::fmt::Debug;
use std::rc::Rc;
use crate::callable::Callable;
use crate::frame::Frame;
use crate::tvm::Tvm;

pub trait State : Debug {
    fn tick(&mut self, tvm: &mut Tvm) -> StateResult;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateResult {
    Return(i32),
    Break,
    Continue,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TvmState {
    Waiting,
    Paused,
    Call(Callable),
    Eval(Frame),
    FrameEval(Frame),
    Halted,
}

impl TvmState {
    pub fn to_state(&self) -> Box<dyn State> {
        match self {
            TvmState::Waiting => Box::new(states::Waiting),
            TvmState::Paused => Box::new(states::Paused),
            TvmState::Call(callable) => Box::new(states::Call { callable: callable.clone() }),
            TvmState::Eval(frame) => Box::new(states::Eval { frame: frame.clone(), pc: 0 }),
            TvmState::FrameEval(frame) => Box::new(states::FrameEval { frame: frame.clone() }),
            TvmState::Halted => Box::new(states::Halted),
        }
    }

    pub fn is_waiting(&self) -> bool {
        matches!(self, TvmState::Waiting)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, TvmState::Paused)
    }

    pub fn is_call(&self) -> bool {
        matches!(self, TvmState::Call(_))
    }

    pub fn is_eval(&self) -> bool {
        matches!(self, TvmState::Eval(_))
    }

    pub fn is_frame_eval(&self) -> bool {
        matches!(self, TvmState::FrameEval(_))
    }

    pub fn is_halted(&self) -> bool {
        matches!(self, TvmState::Halted)
    }
}

pub trait Stateful : Debug {
    fn get_state(&self) -> TvmState;
    fn set_state(&mut self, state: TvmState);
    fn get_ticks(&self) -> usize;
    fn increment_ticks(&mut self);
    fn previous_state(&self) -> Option<TvmState>;
    fn pause(&mut self);
    fn resume(&mut self);
    fn tick(&mut self);
    fn get_last_result(&self) -> Option<StateResult>;
    fn is_paused(&self) -> bool;
    fn handle_result(&mut self, result: StateResult);

    fn call(&mut self, callable: Callable) {
        self.set_state(TvmState::Call(callable));
    }

    fn eval(&mut self, frame: Frame) {
        self.set_state(TvmState::Eval(frame));
    }

    fn frame_eval(&mut self, frame: Frame) {
        self.set_state(TvmState::FrameEval(frame));
    }

    fn should_continue(&self) -> bool {
        !self.get_state().is_halted()
            || matches!(self.get_last_result(), Some(StateResult::Exit))
            || matches!(self.get_last_result(), Some(StateResult::Break))
            || matches!(self.get_last_result(), Some(StateResult::Return(_)))
    }
}

impl Stateful for Tvm {
    fn get_state(&self) -> TvmState {
        self.state.clone()
    }

    fn set_state(&mut self, state: TvmState) {
        println!("Setting state to: {:?}", state);
        self.previous_state = Some(self.state.clone());
        self.state = state;
    }

    fn get_ticks(&self) -> usize {
        self.ticks
    }

    fn increment_ticks(&mut self) {
        self.ticks += 1;
    }

    fn previous_state(&self) -> Option<TvmState> {
        self.previous_state.clone()
    }

    fn pause(&mut self) {
        if !self.is_paused() {
            self.set_state(TvmState::Paused);
        }
    }

    fn resume(&mut self) {
        if self.is_paused() {
            self.set_state(self.previous_state().unwrap());
        }
    }

    fn tick(&mut self) {
        // Do nothing if the tvm is paused
        if !self.is_paused() {
            self.increment_ticks();
            self.state.to_state().tick(self);
            let result = self.get_last_result();
            self.handle_result(result.unwrap());
        }

        println!("Tvm state: {:?}", self.state);
    }

    fn get_last_result(&self) -> Option<StateResult> {
        self.last_result.clone()
    }

    fn is_paused(&self) -> bool {
        matches!(self.state, TvmState::Paused)
    }

    fn handle_result(&mut self, result: StateResult) {
        println!("Handling result: {:?}", result);
    }
}

pub mod states {
    use crate::callable::Caller;
    use crate::frame::FrameEvaluator;
    use crate::instruction::Evaluator;
    use crate::state::StateResult::{Continue, Exit};
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Waiting;
    #[derive(Debug, Clone)]
    pub struct Paused;
    #[derive(Debug, Clone)]
    pub struct Call {
        pub callable: Callable,
    }
    #[derive(Debug, Clone)]
    pub struct Eval {
        pub frame: Frame,
        pub pc: usize,
    }
    #[derive(Debug, Clone)]
    pub struct FrameEval {
        pub frame: Frame,
    }
    #[derive(Debug, Clone)]
    pub struct Halted;

    impl State for Waiting {
        // Tick should do nothing.
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            Continue
        }
    }

    impl State for Paused {
        // Tick should do nothing.
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            Continue
        }
    }

    impl State for Call {
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            tvm.do_call(self.callable.clone());
            Continue
        }
    }

    impl State for Eval {
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            if tvm.should_continue() {
                tvm.do_eval(&mut self.frame);
                self.pc = self.frame.pc;
            }
            Exit
        }
    }

    impl State for FrameEval {
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            if tvm.should_continue() {
                tvm.do_frame_eval(&self.frame);
            }
            Exit
        }
    }

    impl State for Halted {
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            Continue
        }
    }
}
