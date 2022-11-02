use std::fmt::Debug;
use enum_dispatch::enum_dispatch;
use crate::callable::Callable;
use crate::frame::Frame;
use crate::stack::StackHolder;
use crate::state::states::{Call, Eval, FrameEval, Halted, Paused, Waiting};
use crate::tvm::Tvm;

#[enum_dispatch(TvmState)]
pub trait State : Debug {
    fn tick(&mut self, tvm: &mut Tvm) -> StateResult;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateResult {
    Return(i32),
    Break,
    Continue(i32),
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[enum_dispatch]
pub enum TvmState {
    Waiting(Waiting),
    Paused(Paused),
    Call(Call),
    Eval(Eval),
    FrameEval(FrameEval),
    Halted(Halted),
}

impl TvmState {

    pub fn is_waiting(&self) -> bool {
        matches!(self, TvmState::Waiting(_))
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, TvmState::Paused(_))
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
        matches!(self, TvmState::Halted(_))
    }

    pub fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
        match self {
            TvmState::Waiting(ref mut state) => state.tick(tvm),
            TvmState::Paused(ref mut state) => state.tick(tvm),
            TvmState::Call(ref mut state) => state.tick(tvm),
            TvmState::Eval(ref mut state) => state.tick(tvm),
            TvmState::FrameEval(ref mut state) => state.tick(tvm),
            TvmState::Halted(ref mut state) => state.tick(tvm),
        }
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
        self.set_state(Call { callable }.into());
    }

    fn eval(&mut self, frame: Frame) {
        let pc = &frame.pc.clone();
        self.set_state(Eval { frame, pc: *pc }.into());
    }

    fn frame_eval(&mut self, frame: Frame) {
        self.set_state(FrameEval { frame }.into());
    }

    fn should_continue(&self) -> bool {
        !(self.get_state().is_halted()
            || matches!(self.get_last_result(), Some(StateResult::Exit))
            || matches!(self.get_last_result(), Some(StateResult::Break))
            || matches!(self.get_last_result(), Some(StateResult::Return(_))))
    }

    fn get_next_state(&mut self) -> Option<TvmState>;
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
        if let Some(TvmState::Eval(frame)) = &self.previous_state {
            Some(TvmState::Eval(frame.clone()))
        } else {
            None
        }
    }

    fn pause(&mut self) {
        if !self.is_paused() {
            self.set_state(Paused {}.into());
        }
    }

    fn resume(&mut self) {
        if self.is_paused() {
            self.set_state(self.previous_state().unwrap());
        }
    }

    fn tick(&mut self) {
        // Do nothing if the tvm is paused
        if !self.is_paused() && self.should_continue() {
            self.increment_ticks();
            let mut temp_state = self.get_next_state().unwrap();
            temp_state.tick(self);
            let result = self.get_last_result();
            self.handle_result(result.unwrap());
            self.previous_state = Some(temp_state);
        }

        println!("Tvm state: {:?}", self.state);
    }

    fn get_last_result(&self) -> Option<StateResult> {
        self.last_result.clone()
    }

    fn is_paused(&self) -> bool {
        matches!(self.state, TvmState::Paused(_))
    }

    fn handle_result(&mut self, result: StateResult) {
        println!("Handling result: {:?}", result);
        if let StateResult::Return(res) = result {
            self.set_state(self.previous_state().unwrap());
            self.push(res) // I have no clue if this is correct.
        }
        if let StateResult::Break = result {
            self.set_state(self.previous_state().unwrap());
        }
        if let StateResult::Exit = result {
            self.set_state(Halted {}.into());
        }
    }

    // Hacky way of passing the program counter for a frame.
    fn get_next_state(&mut self) -> Option<TvmState> {
        match (&self.state, &self.previous_state) {
            (TvmState::Eval(Eval { frame, pc: _pc }), Some(TvmState::Eval(Eval { frame: prev_frame, pc: prev_pc}))) => {
                let mut next_frame = frame.clone();
                next_frame.pc = prev_frame.pc;
                Some(Eval { frame: next_frame, pc: *prev_pc }.into())
            }
            (state, _) => {
                println!("No arms match.");
                Some(state.clone())
            }
        }
    }
}

pub mod states {
    use crate::callable::Caller;
    use crate::frame::FrameEvaluator;
    use crate::instruction::Evaluator;
    use crate::state::StateResult::{Continue, Exit};
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Waiting;
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Paused;
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Call {
        pub callable: Callable,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Eval {
        pub frame: Frame,
        pub pc: usize,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FrameEval {
        pub frame: Frame,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Halted;

    impl State for Waiting {
        // Tick should do nothing.
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            Continue(0)
        }
    }

    impl State for Paused {
        // Tick should do nothing.
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            Continue(0)
        }
    }

    impl State for Call {
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            tvm.do_call(self.callable.clone());
            tvm.set_state(tvm.previous_state.clone().unwrap()); // Maybe replace this with a calling frame of some kind?
            Continue(0)
        }
    }

    impl State for Eval {
        fn tick(&mut self, tvm: &mut Tvm) -> StateResult {
            if tvm.should_continue() {
                tvm.do_eval(&mut self.frame, 0);
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
            Continue(0)
        }
    }
}
