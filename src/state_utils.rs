use crate::state::{EvalState, TvmState};

impl TvmState {
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
}