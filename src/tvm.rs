use crate::callable::Caller;
use crate::function::Function;
use crate::program::Program;
use crate::state::{StateHolder, TvmState, WaitingState};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tvm {
    pub memory: [i32; 65536],
    pub stack_pointer: usize,
    pub frame_pointer: usize,
    pub heap_size: usize,
    pub state: TvmState,
    pub ticks: usize,
    pub stdout: String,
    pub program: Program,
    pub log: String,
    pub state_history: Vec<TvmState>,
}

impl Default for Tvm {
    fn default() -> Self {
        Tvm {
            memory: [0; 65536],
            stack_pointer: 65535,
            frame_pointer: 65535,
            heap_size: 0,
            state: TvmState::Waiting(WaitingState),
            ticks: 0,
            stdout: String::new(),
            program: Program::default(),
            log: String::new(),
            state_history: Vec::new(),
        }
    }
}

impl Display for Tvm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tvm:")?;
        // write active memory
        writeln!(f, "  Memory: ")?;
        writeln!(f, "    Heap: ")?;
        for i in 0..self.heap_size {
            writeln!(f, "    {}\t{}", i, self.memory[i])?;
        }
        writeln!(f, "    Stack: ")?;
        for i in self.stack_pointer..65536 {
            writeln!(f, "    {}\t{}", i, self.memory[i])?;
        }
        writeln!(f, "  Stack Pointer: {}", self.stack_pointer)?;
        writeln!(f, "  Frame Pointer: {}", self.frame_pointer)?;
        writeln!(f, "  Heap Size: {}", self.heap_size)?;
        writeln!(f, "  State: {}", self.state)?;
        writeln!(f, "  Ticks: {}", self.ticks)?;
        Ok(())
    }
}

impl Tvm {
    pub fn start(&mut self) {
        self.call(self.get_callable(self.program.entry_point as i32));
    }

    pub fn load(&mut self, program: Program) {
        self.program = program;
        self.heap_size = self.program.heap_size as usize;
        for (location, value) in &self.program.heap {
            self.memory[*location] = *value;
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; 65536];
        self.stack_pointer = 65535;
        self.frame_pointer = 65535;
        self.heap_size = 0;
        self.state = TvmState::Waiting(WaitingState);
        self.ticks = 0;
        self.stdout = String::new();
        self.state_history = Vec::new();
        self.log.push_str("Reset\n");
        self.load(self.program.clone());
    }

    pub fn a2s(&mut self, address: usize) -> String {
        let mut s = String::new();
        let mut i = address;
        while self.memory[i] != 0 {
            s.push(self.memory[i] as u8 as char);
            i += 1;
        }
        s
    }

    pub fn write_string(&mut self, address: usize, s: String) {
        let mut i = address;
        for c in s.chars() {
            self.memory[i] = c as i32;
            i += 1;
        }
        self.memory[i] = 0;
    }

    pub fn get_active_memory(&self) -> Vec<(usize, i32)> {
        let mut memory = Vec::new();
        // for i in 0..self.heap_size {
        //     memory.push((i, self.memory[i]));
        // }
        for i in self.stack_pointer..65536 {
            memory.push((i, self.memory[i].clone()));
        }
        memory.sort_by(|(a, _), (b, _)| a.cmp(b));
        memory
    }

    pub fn get_active_memory_string(&self) -> String {
        let mut s = String::new();
        for (i, value) in self.get_active_memory() {
            s.push_str(&format!("{}: {}\n", i, value));
        }
        s
    }

    pub fn get_function(&self, id: usize) -> Function {
        self.program.functions[id].clone()
    }

    pub fn is_halted(&self) -> bool {
        matches!(self.state, TvmState::Halt(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::callable;
    use crate::state::CallState;

    fn get_test_program() -> Program {
        Program::builder()
            .entry_point(0)
            .heap(vec![
                (0, 'h' as i32),
                (1, 'e' as i32),
                (2, 'l' as i32),
                (3, 'l' as i32),
                (4, 'o' as i32),
                (5, 0),
            ])
            .function(Function::builder().id(0).name("init".to_string()).build())
            .build()
    }

    #[test]
    fn test_default() {
        let tvm = Tvm::default();
        assert_eq!(tvm.memory.len(), 65536);
        assert_eq!(tvm.stack_pointer, 65535);
        assert_eq!(tvm.frame_pointer, 65535);
        assert_eq!(tvm.heap_size, 0);
        assert_eq!(tvm.state, TvmState::Waiting(WaitingState));
        assert_eq!(tvm.ticks, 0);
        assert_eq!(
            tvm.get_previous_state(),
            Box::new(TvmState::Waiting(WaitingState))
        );
    }

    #[test]
    fn test_load() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        assert_eq!(tvm.heap_size, 6);
        assert_eq!(tvm.memory[0], 'h' as i32);
        assert_eq!(tvm.memory[1], 'e' as i32);
        assert_eq!(tvm.memory[2], 'l' as i32);
        assert_eq!(tvm.memory[3], 'l' as i32);
        assert_eq!(tvm.memory[4], 'o' as i32);
        assert_eq!(tvm.memory[5], 0);
    }

    #[test]
    fn test_start() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        tvm.start();
        let state = tvm.state;
        assert!(matches!(state, TvmState::Call(_)));
        assert!(
            matches!(state, TvmState::Call(CallState { callable: callable::Callable::Function(function), .. }) if function.id == 0 && function.name == "init")
        );
    }

    #[test]
    fn test_a2s() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        assert_eq!(tvm.a2s(0), "hello");
    }

    #[test]
    fn test_write_string() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        assert_eq!(tvm.a2s(0), "hello");
        tvm.write_string(0, "world".to_string());
        assert_eq!(tvm.a2s(0), "world");
    }

    #[test]
    fn test_get_active_memory() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        assert_eq!(
            tvm.get_active_memory(),
            vec![
                (0, 'h' as i32),
                (1, 'e' as i32),
                (2, 'l' as i32),
                (3, 'l' as i32),
                (4, 'o' as i32),
                (5, 0),
                (65535, 0)
            ]
        );
    }

    #[test]
    fn test_get_function() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        let function = tvm.get_function(0);
        assert_eq!(function.id, 0);
        assert_eq!(function.name, "init");
    }

    #[test]
    fn test_get_active_memory_string() {
        let mut tvm = Tvm::default();
        let program = get_test_program();
        tvm.load(program);
        assert_eq!(
            tvm.get_active_memory_string(),
            "0: 104\n1: 101\n2: 108\n3: 108\n4: 111\n5: 0\n65535: 0\n"
        );
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
