use crate::frame::Frame;
use crate::instruction::Instruction;
use crate::state::{Stateful, StateResult, TvmState};
use crate::state::StateResult::Continue;

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
            last_result: Some(Continue),
        }
    }
}

impl Tvm {

    pub fn start(&mut self) {
        let builder = Frame::builder();
        let frame = builder
            .id(0)
            .name("main".to_string())
            .instruction(Instruction::get_instruction(1), vec![])
            .primitive(0)
            .callable(-101, vec![])
            .instruction(Instruction::get_instruction(8), vec![])
            .primitive(-109)
            .primitive(10)
            .build();

        self.frame_eval(frame);
    }
}

#[cfg(test)]
mod test {
    use crate::state::Stateful;
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

    #[test]
    fn test_tick_count() {
        let mut tvm = Tvm::default();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();
        tvm.tick();

        assert_eq!(tvm.ticks, 12);
    }
}