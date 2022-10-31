use std::cell::RefCell;
use std::rc::Rc;
use crate::vm::function::Frame;
use crate::vm::tvm::{Callable, Tvm};

pub type StateBox = Box<dyn State>;
type StateRef = Rc<RefCell<StateBox>>;

pub trait State {
    fn pause(self: Box<Self>, tvm: &mut Tvm);
    fn resume(self: Box<Self>, tvm: &mut Tvm);
    fn tick(self: Box<Self>, tvm: &mut Tvm);
    fn get_previous_state(self: Box<Self>) -> Option<StateBox>;
    fn set_previous_state(&mut self, state: StateBox);
}

impl dyn State {
    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        tvm.pause();
    }
    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        tvm.resume();
    }
    fn tick(self: Box<Self>, tvm: &mut Tvm) {
        tvm.tick();
    }
    fn get_previous_state(&self) -> Option<StateBox> {
        None
    }
}

pub struct WaitingState;

impl State for WaitingState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn get_previous_state(self: Box<Self>) -> Option<StateBox> {
        None
    }

    fn set_previous_state(&mut self, state: StateBox) {

    }
}

pub struct CallState {
    pub callable: Callable,
    pub previous_state: Option<StateBox>,
}

impl State for CallState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn get_previous_state(self: Box<Self>) -> Option<StateBox> {
        self.previous_state
    }

    fn set_previous_state(&mut self, state: StateBox) {
        self.previous_state = Some(state);
    }
}

pub struct EvalFrameState {
    pub frame: Frame,
    pub previous_state: Option<StateBox>,
}

impl State for EvalFrameState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn get_previous_state(self: Box<Self>) -> Option<StateBox> {
        self.previous_state
    }

    fn set_previous_state(&mut self, state: StateBox) {
        self.previous_state = Some(state);
    }
}

pub struct EvalState {
    pub frame: Frame,
    pub pc: i32,
    pub previous_state: Option<StateBox>,
}

impl State for EvalState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn get_previous_state(self: Box<Self>) -> Option<StateBox> {
        self.previous_state
    }

    fn set_previous_state(&mut self, state: StateBox) {
        self.previous_state = Some(state);
    }
}

pub struct PauseState {
    pub previous_state: Option<StateBox>,
}

impl State for PauseState {

    fn pause(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn resume(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn tick(self: Box<Self>, tvm: &mut Tvm) {
        todo!()
    }

    fn get_previous_state(self: Box<Self>) -> Option<StateBox> {
        self.previous_state
    }

    fn set_previous_state(&mut self, state: StateBox) {
        self.previous_state = Some(state);
    }
}
