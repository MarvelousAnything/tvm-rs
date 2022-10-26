use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Row, Table, TableState};
use crate::vm::tvm::Tvm;

struct TvmEvents {
    written_memory: Vec<i32>,
    state: TableState,
}

impl TvmEvents {
    fn new(mem: Vec<i32>) -> TvmEvents{
        TvmEvents {
            written_memory: mem,
            state: TableState::default(),
        }
    }

}

pub fn ui<B: Backend>(f: &mut Frame<B>, tvm: &Tvm) {
    f.render_stateful_widget(to_table(tvm), f.size(), &mut TableState::default());
}

pub fn to_table<'a>(tvm: &Tvm) -> Table<'a> {
    let rows: Vec<Row> = tvm.mem.iter().enumerate().map(|(k, v)| Row::new(vec![k.to_string(), v.to_string()])).collect();
    let table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .header(Row::new(vec!["Address", "Memory"])
            .style(Style::default().fg(Color::Yellow)))
        .block(Block::default().borders(Borders::ALL).title("Memory"))
        .widths(&[Constraint::Length(10), Constraint::Length(10)])
        .column_spacing(1)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .highlight_symbol("> ");
    table
}

