use crate::callable::{Callable, Caller};
use crate::frame::{Frame, FrameEvaluator};
use crate::instruction::Evaluator;
use crate::stack::StackHolder;
use crate::tvm::Tvm;
use enum_dispatch::enum_dispatch;
use std::fmt::{Debug, Display, Formatter};

#[cfg(test)]
use crate::native::NativeFunction;

#[enum_dispatch]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TvmState {
    Waiting(WaitingState),
    Call(CallState),
    Eval(EvalState),
    FrameEval(FrameEvalState),
    Halt(HaltState),
}

impl TvmState {
    pub fn get_previous_state(&self) -> Box<TvmState> {
        match self {
            TvmState::Waiting(state) => state.get_previous_state(),
            TvmState::Call(state) => state.get_previous_state(),
            TvmState::Eval(state) => state.get_previous_state(),
            TvmState::FrameEval(state) => state.get_previous_state(),
            TvmState::Halt(state) => state.get_previous_state(),
        }
    }

    pub fn set_previous_state(&mut self, new_state: Box<TvmState>) {
        match self {
            TvmState::Waiting(_) => {}
            TvmState::Call(state) => state.previous_state = new_state,
            TvmState::Eval(state) => state.previous_state = new_state,
            TvmState::FrameEval(state) => state.previous_state = new_state,
            TvmState::Halt(state) => state.previous_state = new_state,
        }
    }

    pub fn set_result(&mut self, result: StateResult) {
        match self {
            TvmState::Waiting(state) => state.set_result(result),
            TvmState::Call(state) => state.set_result(result),
            TvmState::Eval(state) => state.set_result(result),
            TvmState::FrameEval(state) => state.set_result(result),
            TvmState::Halt(_state) => {}
        }
    }

    pub fn get_depth(&self) -> usize {
        if matches!(self, TvmState::Waiting(_)) {
            1
        } else {
            self.get_previous_state().get_depth() + 1
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            TvmState::Waiting(state) => state.to_string(),
            TvmState::Call(state) => state.to_string(),
            TvmState::Eval(state) => state.to_string(),
            TvmState::FrameEval(state) => state.to_string(),
            TvmState::Halt(state) => state.to_string(),
        }
    }
}

impl Display for TvmState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TvmState::Waiting(state) => write!(f, "{}", state),
            TvmState::Call(state) => write!(f, "{} <- {}", state, state.get_previous_state()),
            TvmState::Eval(state) => write!(f, "{} <- {}", state, state.get_previous_state()),
            TvmState::FrameEval(state) => write!(f, "{} <- {}", state, state.get_previous_state()),
            TvmState::Halt(state) => write!(f, "{} <- {}", state, state.get_previous_state()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateResult {
    None,
    Return,
    Break,
    Exit,
    Continue,
    Halt,
}

#[enum_dispatch(TvmState)]
pub trait State: Debug + Display {
    fn tick(&mut self, tvm: &mut Tvm);
    fn get_previous_state(&self) -> Box<TvmState>;
    fn get_result(&self) -> StateResult;
    fn set_result(&mut self, result: StateResult);
}

pub trait StateHolder {
    fn get_state(&self) -> TvmState;
    fn set_state(&mut self, state: TvmState);
    fn get_previous_state(&self) -> Box<TvmState>;
    fn get_result(&self) -> StateResult;
    fn handle_result(&mut self, result: StateResult);
    fn tick(&mut self);
    fn call(&mut self, callable: Callable);
    fn frame_eval(&mut self, frame: Frame);
    fn eval(&mut self, frame: Frame);
    fn should_continue(&self) -> bool;
}

impl StateHolder for Tvm {
    fn get_state(&self) -> TvmState {
        self.state.clone()
    }

    fn set_state(&mut self, state: TvmState) {
        // println!("{} -> {}", self.state, state);
        let temp = self.state.clone();
        self.state = state;
        self.state.set_previous_state(Box::new(temp));
    }

    fn get_previous_state(&self) -> Box<TvmState> {
        self.state.get_previous_state()
    }

    fn get_result(&self) -> StateResult {
        self.state.get_result()
    }

    fn handle_result(&mut self, result: StateResult) {
        let previous_state = *self.get_previous_state();
        // println!("Handling result: {:#?} for {}", result, previous_state);
        // println!("Current state: {}", self.state);
        match result {
            StateResult::None => {}
            StateResult::Return => {
                if let TvmState::Call(CallState {
                    callable: Callable::Function(function),
                    ..
                }) = &self.state.clone()
                {
                    // Handle the return of a function.
                    let r = self.pop();
                    self.stack_pointer = self.frame_pointer;
                    self.frame_pointer = self.memory[self.stack_pointer] as usize;
                    self.stack_pointer += function.args + function.locals;
                    self.push(r);
                }

                // If there is a nested frame. For instance a return statement within a loop, this will not work.
                // For now, my rational is that the return call (within the execution of an eval state) will be within a frame eval state
                // and that frame eval state will have a previous state of a call state. This is the call for the function that is being returned from.
                // That call state will have a previous state of eval state. This is where we want to return to.
                // This is only for non-native functions.
                self.state = self.state.get_return_state().into();
            }
            StateResult::Break => {
                // Assume break is in a loop somewhere.
                let loop_state = self.state.get_loop_frame_eval_state();
                if let Some(state) = loop_state {
                    let mut temp = state;
                    if let TvmState::Eval(EvalState { frame, .. }) = &mut temp {
                        if frame.pc < frame.data.len() {
                            frame.pc += 2;
                        }
                    }
                    self.state = temp;
                }
            }
            StateResult::Continue => {}
            StateResult::Halt => {
                self.state = TvmState::Halt(HaltState {
                    previous_state: Box::new(previous_state),
                });
            }
            StateResult::Exit => {
                // Exit should exit the frame not the program.
                self.log.push_str(format!("current state: {}\n", self.state.get_name()).as_str());
                if let TvmState::Eval(EvalState { frame, .. }) = &self.state {
                    self.log.push_str(format!("current frame: {}, pc: {}\n", frame.name, frame.pc).as_str());
                    // frame.pc += 1;
                }
                // get enclosing frame.
                let mut enclosing_state = *previous_state.get_previous_state();
                self.log.push_str(format!("enclosing state: {}\n", enclosing_state.get_name()).as_str());
                if let TvmState::Eval(EvalState { frame, .. }) = &mut enclosing_state {
                    self.log.push_str(format!("enclosing frame: {}, pc: {}\n", frame.name, frame.pc).as_str());
                    frame.pc += 1;
                } else {
                    self.state = TvmState::Halt(HaltState {
                        previous_state: Box::new(previous_state),
                    });
                    return;
                }
                self.state = enclosing_state;
            }
        }
    }

    fn tick(&mut self) {
        self.log.push_str(format!("Tick {}: {}\n", self.ticks, self.state.get_name()).as_str());
        self.state_history.push(self.state.clone());
        let mut temp_state = self.state.clone();
        // println!("Ticking: {}", temp_state);
        temp_state.tick(self); // This is so the PC can persist. Hopefully.
        temp_state.set_result(self.get_result()); // Update temp state with result.
        if matches!(temp_state, TvmState::Eval(_)) && matches!(self.state, TvmState::Eval(_)) {
            self.state = temp_state;
        } else if matches!(self.state, TvmState::Call(_))
            && !matches!(temp_state, TvmState::Call(_))
        {
            // println!("Setting state to: {}", temp_state);
            // Assuming that temp_state is an EvalState, when we evaluate call state, we want to go back to the temp_state with incremented PC.
            self.state.set_previous_state(Box::new(temp_state)); // This is so the PC can persist. Hopefully. After the call, we want to go back to the EvalState.
        }
        self.handle_result(self.get_result());
        self.ticks += 1;
    }

    fn call(&mut self, callable: Callable) {
        self.set_state(
            CallState {
                callable,
                previous_state: Box::new(self.get_state()),
                result: StateResult::Continue,
            }
            .into(),
        )
    }

    fn frame_eval(&mut self, frame: Frame) {
        self.set_state(
            FrameEvalState {
                frame,
                previous_state: Box::new(self.get_state()),
                result: StateResult::Continue,
            }
            .into(),
        )
    }

    fn eval(&mut self, frame: Frame) {
        self.set_state(
            EvalState {
                frame,
                previous_state: Box::new(self.get_state()),
                result: StateResult::Continue,
            }
            .into(),
        )
    }

    fn should_continue(&self) -> bool {
        !matches!(self.state, TvmState::Halt(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitingState;

impl State for WaitingState {
    fn tick(&mut self, tvm: &mut Tvm) {
        tvm.state = self.clone().into();
    }

    fn get_previous_state(&self) -> Box<TvmState> {
        Box::new(TvmState::Waiting(Self))
    }

    fn get_result(&self) -> StateResult {
        StateResult::None
    }

    fn set_result(&mut self, _result: StateResult) {}
}

#[cfg(test)]
impl Default for WaitingState {
    fn default() -> Self {
        Self
    }
}

impl Display for WaitingState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitingState")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallState {
    pub callable: Callable,
    pub previous_state: Box<TvmState>,
    pub result: StateResult,
}

impl State for CallState {
    fn tick(&mut self, tvm: &mut Tvm) {
        tvm.state = self.clone().into();
        tvm.do_call(self.callable.clone());
    }

    fn get_previous_state(&self) -> Box<TvmState> {
        self.previous_state.clone()
    }

    fn get_result(&self) -> StateResult {
        self.result.clone()
    }

    fn set_result(&mut self, result: StateResult) {
        self.result = result;
    }
}

#[cfg(test)]
impl Default for CallState {
    fn default() -> Self {
        Self {
            callable: Callable::Native(NativeFunction::Unknown(-999)),
            previous_state: Box::new(WaitingState::default().into()),
            result: StateResult::Continue,
        }
    }
}

impl Display for CallState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call {}", self.callable)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalState {
    pub frame: Frame,
    pub previous_state: Box<TvmState>,
    pub result: StateResult,
}

impl State for EvalState {
    fn tick(&mut self, tvm: &mut Tvm) {
        tvm.state = self.clone().into();
        tvm.do_eval(&mut self.frame);
    }

    fn get_previous_state(&self) -> Box<TvmState> {
        self.previous_state.clone()
    }

    fn get_result(&self) -> StateResult {
        self.result.clone()
    }

    fn set_result(&mut self, result: StateResult) {
        self.result = result;
    }
}

#[cfg(test)]
impl Default for EvalState {
    fn default() -> Self {
        Self {
            frame: Frame::default(),
            previous_state: Box::new(WaitingState::default().into()),
            result: StateResult::Continue,
        }
    }
}

impl Display for EvalState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.frame.pc < self.frame.data.len() {
            write!(f, "EvalState {} {} - {}", self.frame.name, self.frame.pc, self.frame.data[self.frame.pc])
        } else {
            write!(f, "EvalState {} {}", self.frame.name, self.frame.pc)
        }
    }
}

// The only real purpose of this state is to call the eval state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameEvalState {
    pub frame: Frame,
    pub previous_state: Box<TvmState>,
    pub result: StateResult,
}

impl State for FrameEvalState {
    fn tick(&mut self, tvm: &mut Tvm) {
        tvm.state = self.clone().into();
        tvm.do_frame_eval(self.frame.clone());
    }

    fn get_previous_state(&self) -> Box<TvmState> {
        self.previous_state.clone()
    }

    fn get_result(&self) -> StateResult {
        self.result.clone()
    }

    fn set_result(&mut self, result: StateResult) {
        self.result = result;
    }
}

#[cfg(test)]
impl Default for FrameEvalState {
    fn default() -> Self {
        Self {
            frame: Frame::default(),
            previous_state: Box::new(WaitingState::default().into()),
            result: StateResult::Continue,
        }
    }
}

impl Display for FrameEvalState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FrameEvalState {}", self.frame.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaltState {
    pub previous_state: Box<TvmState>,
}

impl State for HaltState {
    fn tick(&mut self, tvm: &mut Tvm) {
        tvm.state = self.clone().into();
        // println!("HaltState");
    }

    fn get_previous_state(&self) -> Box<TvmState> {
        self.previous_state.clone()
    }

    fn get_result(&self) -> StateResult {
        StateResult::Halt
    }

    fn set_result(&mut self, _result: StateResult) {}
}

#[cfg(test)]
impl Default for HaltState {
    fn default() -> Self {
        Self {
            previous_state: Box::new(WaitingState::default().into()),
        }
    }
}

impl Display for HaltState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HaltState")
    }
}
