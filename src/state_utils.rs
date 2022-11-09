use crate::state::{EvalState, TvmState};
use crate::frame::{FrameData};
use crate::instruction::Instruction;

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

    // Finds out if a previous frame is evaluating a loop.
    pub fn check_in_loop(&self) -> bool {
        let mut state = self.clone();
        // This will have optimal performance every time...
        loop {
            if let TvmState::Waiting(_) = state {
                return false;
            }
            if let TvmState::Eval(EvalState { frame, .. }) = state.clone() {
                let mut pc = frame.pc;
                if pc >= frame.data.len() {
                    pc = frame.data.len() - 1;
                }
                if matches!(frame.data[pc], FrameData::Instruction(Instruction::Loop { .. }, _)) {
                    return true;
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
                if matches!(frame.data[pc], FrameData::Instruction(Instruction::Loop { .. }, _)) {
                    return Some(state);
                }
            }
            state = *state.get_previous_state();
        }
    }
}