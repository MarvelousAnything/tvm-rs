use std::ops::{Index, IndexMut};
use tui::widgets::TableState;

#[derive(Debug)]
pub struct Memory {
    pub mem: [i32; 65536],
    pub stack_pointer: usize,
    pub frame_pointer: usize,
    pub state: TableState,
}

impl Memory {
    pub fn default() -> Memory {
        let mut m = Memory {
            mem: [0; 65536],
            stack_pointer: 65535,
            frame_pointer: 65535,
            state: TableState::default(),
        };
        m.state.select(Some(m.all_to_selected(m.stack_pointer)));
        m
    }

    pub fn push(&mut self, value: i32) {
        self.mem[self.stack_pointer] = value;
        self.stack_pointer -= 1;
        self.previous();
        // println!("Pushed {} to stack", value);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack_pointer += 1;
        // println!("Popped {} from stack", self.mem[self.stack_pointer]);
        self.next();
        self.mem[self.stack_pointer]
    }

    pub fn get_active_mem(&self) -> Vec<(usize, i32)> {
        self.mem
            .iter()
            .enumerate()
            .filter(|(k, v)| (**v != 0) | (*k == self.stack_pointer) | (*k == self.frame_pointer))
            .map(|(k, v)| (k, *v))
            .collect()
    }

    pub fn selected_to_active(&self) -> usize {
        self.get_active_mem()
            .binary_search_by(|(k, _)| k.cmp(&self.state.selected().unwrap_or(0)))
            .unwrap_or(0)
    }

    pub fn active_to_all(&self, active: usize) -> usize {
        self.get_active_mem()[active].0
    }

    pub fn all_to_selected(&self, all: usize) -> usize {
        self.get_active_mem()
            .binary_search_by(|(k, _)| k.cmp(&all))
            .unwrap_or(0)
    }

    pub fn update_state(&mut self) {
        self.state
            .select(Some(self.all_to_selected(self.stack_pointer)));
    }

    pub fn peek(&self) -> i32 {
        self.mem[self.stack_pointer]
    }

    pub fn iter(&self) -> std::slice::Iter<i32> {
        self.mem.iter()
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.get_active_mem().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.get_active_mem().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Index<usize> for Memory {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mem[index]
    }
}
