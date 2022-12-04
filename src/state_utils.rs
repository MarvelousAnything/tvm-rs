use crate::frame::FrameData;
use crate::instruction::Instruction;
use crate::state::{EvalState, TvmState};

impl TvmState {
    // Gets
    pub fn get_return_state(&self) -> EvalState {
        let mut state = self.clone();
        while !matches!(state, TvmState::Call(_)) {
            if let TvmState::Waiting(_) = state {
                panic!("Waiting state has no return state");
            }
            state = *state.get_previous_state();
        }
        if let TvmState::Eval(state) = *state.get_previous_state() {
            state
        } else {
            panic!("Expected EvalState. Found {}", *state.get_previous_state());
        }
    }

    // Finds out if a previous frame is evaluating a loop.
    pub fn check_in_loop(&self) -> bool {
        let mut state = self.clone();
        // This will have optimal performance every time...
        loop {
            if let TvmState::Waiting(_) = state {
                return false;
            }
            if let TvmState::Eval(EvalState { frame, .. }) = &mut state {
                if !frame.data.is_empty() {
                    let mut pc = frame.pc;
                    if pc >= frame.data.len() {
                        pc = frame.data.len() - 1;
                    }
                    if matches!(
                    frame.data[pc],
                    FrameData::Instruction(Instruction::Loop { .. }, _)
                ) {
                        return true;
                    }
                }
            }
            state = *state.get_previous_state();
        }
    }

    pub fn get_loop_frame_eval_state(&self) -> Option<TvmState> {
        let mut state = self.clone();
        // This will have optimal performance every time...
        loop {
            if let TvmState::Waiting(_) = state {
                return None;
            }
            if let TvmState::Eval(EvalState { frame, .. }) = state.clone() {
                let mut pc = frame.pc;
                if pc >= frame.data.len() {
                    pc = frame.data.len() - 1;
                }
                if matches!(
                    frame.data[pc],
                    FrameData::Instruction(Instruction::Loop { .. }, _)
                ) {
                    return Some(state);
                }
            }
            state = *state.get_previous_state();
        }
    }

    pub fn get_root_state(self) -> TvmState {
        if self.is_root_state() {
            return self;
        }
        self.get_previous_state().get_root_state()
    }

    // Just check if the previous state is waiting. If it is, then we can assume that it is the root state.
    pub fn is_root_state(&self) -> bool {
        matches!(*self.get_previous_state(), TvmState::Waiting(_))
    }
}

#[cfg(test)]
mod state_builder {
    use crate::state::{CallState, EvalState, FrameEvalState, HaltState, TvmState, WaitingState};

    pub struct StateBuilder {
        state: TvmState
    }

    impl StateBuilder {
        pub fn new() -> Self {
            Self {
                state: TvmState::Waiting(WaitingState)
            }
        }

        fn add_state(&mut self, state: TvmState) {
            let mut temp = state;
            temp.set_previous_state(Box::new(self.state.clone()));
            self.state = temp;
        }

        pub fn waiting(mut self) -> Self {
            self.add_state(TvmState::Waiting(WaitingState));
            self
        }

        pub fn call(mut self) -> Self {
            self.add_state(TvmState::Call(CallState::default()));
            self
        }

        pub fn eval(mut self) -> Self {
            self.add_state(EvalState::default().into());
            self
        }

        pub fn frame_eval(mut self) -> Self {
            self.add_state(FrameEvalState::default().into());
            self
        }

        pub fn halt(mut self) -> Self {
            self.add_state(HaltState::default().into());
            self
        }

        pub fn build(self) -> TvmState {
            self.state
        }
    }

    impl From<TvmState> for StateBuilder {
        fn from(state: TvmState) -> Self {
            Self {
                state
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{EvalState, TvmState};
    use super::state_builder::*;

    #[test]
    fn test_get_return_state() {
        let state = StateBuilder::new()
            .call()
            .frame_eval()
            .eval()
            .eval()
            .eval()
            .build();

        assert!(matches!(state.get_return_state(), EvalState { .. }));
    }
}