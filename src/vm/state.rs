use std::fmt::Debug;
use crate::vm::function::{Frame};
use crate::vm::tvm::{Callable, Tvm};

pub trait State<T: State<T>> {
    fn pause(self: Box<Self>, tvm: &mut Tvm);
    fn resume(self: Box<Self>, tvm: &mut Tvm);
    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32>;
    fn get_tvm_state(&self) -> TvmState;
}

impl <T: State<T>> Debug for dyn State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State")
    }
}

pub struct Waiting;

pub enum PausedState {
    Waiting(Box<Waiting>),
    CallState(Box<CallState>),
    EvalState(Box<EvalState>),
    EvalFrameState(Box<EvalFrameState>),
    ErrorState(Box<ErrorState>),
}

pub struct CallState {
    pub(crate) callable: Callable
}

pub struct EvalState {
    pub frame: Frame,
    pub pc: i32
}

pub struct EvalFrameState {
    pub frame: Frame
}

pub struct ErrorState {
    pub error: String
}

impl State<Self> for ErrorState {
    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        panic!("Error: {}", self.error);
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        panic!("Error: {}", self.error);
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32> {
        panic!("Error: {}", self.error);
    }

    fn get_tvm_state(&self) -> TvmState {
        TvmState::ErrorState(self)
    }
}

impl State<Self> for Waiting {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {

    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {

    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32> {
        None
    }

    fn get_tvm_state(&self) -> TvmState {
        TvmState::Waiting(self)
    }
}

impl State<Self> for PausedState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        // Do nothing
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        tvm.paused = false;
        State::resume(self.previous_state.unwrap(), tvm);
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32> {
        if !tvm.paused {
            State::resume(self.previous_state.unwrap(), tvm);
        }
        None
    }

    fn get_tvm_state(&self) -> TvmState {
        TvmState::PausedState(self)
    }
}

impl PausedState {
    pub fn previous_state(&self) -> Option<Box<dyn State<PausedState>>> {
        *match self {
            PausedState::Waiting(_) => None,
            PausedState::CallState(state) => Some(state),
            PausedState::EvalState(state) => Some(state),
            PausedState::EvalFrameState(state) => Some(state),
            PausedState::ErrorState(state) => Some(state),
        }
    }
}

impl State<Self> for CallState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        tvm.paused = true;
        tvm.set_state(Box::new(PausedState::CallState(self)));
    }
    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        if tvm.paused {
            tvm.paused = false;
        }
        tvm.set_state(self);
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32> {
        if !tvm.paused {
            tvm.do_call(self.callable);
        } else {
            tvm.set_state(Box::new(PausedState::CallState(self)));
        }
        None
    }

    fn get_tvm_state(&self) -> TvmState {
        TvmState::CallState(self)
    }
}

impl State<Self> for EvalState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        tvm.paused = true;
        tvm.set_state(Box::new(PausedState::EvalState(self)));
    }
    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        if tvm.paused {
            tvm.paused = false;
        }
        tvm.set_state(self);
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32> {
        if !tvm.paused {
            return Some(tvm.do_eval(&self.frame, self.pc));
        } else {
            tvm.set_state(Box::new(PausedState::EvalState(self)));
        }
        None
    }

    fn get_tvm_state(&self) -> TvmState {
        TvmState::EvalState(self)
    }
}

impl State<Self> for EvalFrameState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        tvm.paused = true;
        tvm.set_state(Box::new(PausedState::EvalFrameState(self)));
    }
    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        if tvm.paused {
            tvm.paused = false;
        }
        tvm.set_state(self);
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) -> Option<i32> {
        if !tvm.paused {
            return Some(tvm.do_eval_frame(&self.frame));
        } else {
            tvm.set_state(Box::new(PausedState::EvalFrameState(self)));
        }
        None
    }

    fn get_tvm_state(&self) -> TvmState {
        TvmState::EvalFrameState(self)
    }
}

pub enum TvmState<'a> {
    Waiting(&'a Waiting),
    PausedState(&'a PausedState),
    CallState(&'a CallState),
    EvalState(&'a EvalState),
    EvalFrameState(&'a EvalFrameState),
    ErrorState(&'a ErrorState)
}

impl <'a> TvmState<'a> {
    pub fn new<T: State<T>>(state: Box<dyn State<T>>) -> TvmState<'a> {
        state.get_tvm_state()
    }

    pub fn get_state<T: State<T>>(&self) -> &dyn State<T> {
        *match self {
            TvmState::Waiting(state) => Box::new(state),
            TvmState::PausedState(state) => Box::new(state),
            TvmState::CallState(state) => state,
            TvmState::EvalState(state) => state,
            TvmState::EvalFrameState(state) => state,
            TvmState::ErrorState(state) => state
        }
    }
}