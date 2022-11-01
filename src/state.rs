use std::fmt::Debug;
use crate::callable::Callable;
use crate::frame::Frame;
use crate::tvm::Tvm;

pub trait State : Debug + Clone {
    fn pause(&mut self);
    fn resume(&mut self);
    fn tick(&mut self) -> StateResult;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateResult {
    Return(i32),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TvmState {
    Waiting,
    Paused,
    Call(Callable),
    Eval(Frame, usize),
    FrameEval(Frame),
    Halted,
}

pub trait Stateful : Debug {
    fn get_state(&self) -> TvmState;
    fn set_state(&mut self, state: TvmState);
    fn get_ticks(&self) -> usize;
    fn increment_ticks(&mut self);
    fn previous_state(&self) -> Option<TvmState>;
    fn is_paused(&self) -> bool;
}

impl Stateful for Tvm {
    fn get_state(&self) -> TvmState {
        self.state.clone()
    }

    fn set_state(&mut self, state: TvmState) {
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

    fn is_paused(&self) -> bool {
        self.state == TvmState::Paused
    }
}

