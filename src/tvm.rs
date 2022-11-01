use crate::frame::Frame;
use crate::state::{StateResult, TvmState};

#[derive(Debug, Clone)]
pub struct Tvm {
    pub memory: [i32; 65536],
    pub stack_pointer: usize,
    pub frame_pointer: usize,
    pub heap_size: usize,
    pub state: TvmState,
    pub ticks: usize,
    pub previous_state: Option<TvmState>,
    pub last_result: Option<StateResult>,
}

impl Default for Tvm {
    fn default() -> Self {
        Tvm {
            memory: [0; 65536],
            stack_pointer: 65535,
            frame_pointer: 65535,
            heap_size: 0,
            state: TvmState::Waiting,
            ticks: 0,
            previous_state: None,
            last_result: None,
        }
    }
}

impl Tvm {
    pub fn frame_eval(&mut self, frame: Frame) {
        self.state = TvmState::FrameEval(frame);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let tvm = Tvm::default();
        assert_eq!(tvm.memory.len(), 65536);
        assert_eq!(tvm.stack_pointer, 65535);
        assert_eq!(tvm.frame_pointer, 65535);
        assert_eq!(tvm.heap_size, 0);
        assert_eq!(tvm.state, TvmState::Waiting);
        assert_eq!(tvm.ticks, 0);
        assert_eq!(tvm.previous_state, None);
    }
}