use tui::widgets::{ListItem, ListState, TableState};
use crate::state::TvmState;
use crate::tvm::Tvm;

pub struct TvmUI {
    pub stack_state: TableState,
    pub heap_state: TableState,
    pub log_state: ListState,
    pub history_state: ListState,
}

impl Tvm {
    pub fn selected_to_active(&self) -> usize {
        self.get_active_memory()
            .binary_search_by(|(k, _)| k.cmp(&self.table_state.selected().unwrap_or(0)))
            .unwrap_or(0)
    }

    pub fn active_to_all(&self, active: usize) -> usize {
        self.get_active_memory()[active].0
    }

    pub fn all_to_selected(&self, all: usize) -> usize {
        self.get_active_memory()
            .binary_search_by(|(k, _)| k.cmp(&all))
            .unwrap_or(0)
    }

    pub fn update_table_state(&mut self) {
        self.table_state.select(Some(self.all_to_selected(self.stack_pointer)));
    }

    pub fn log_to_list_items(log: &str) -> Vec<ListItem> {
        log
            .lines()
            .map(|l| ListItem::new(l.to_string()))
            .collect()
    }

    fn get_tabs(state: &TvmState) -> String {
        let mut out = String::new();
        for _ in 0..state.get_depth() {
            out.push(' ');
        }
        out
    }

    pub fn state_history_to_list_items(state_history: &[TvmState]) -> Vec<ListItem> {
        state_history
            .iter()
            .map(|l| ListItem::new(format!("{}{}", Self::get_tabs(l), l.get_name())))
            .collect()
    }
}
