extern crate core;

use crossterm::event::KeyModifiers;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{error::Error, io};

use tui::layout::Direction;
use tui::widgets::{List, ListState, Paragraph};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};

use crate::program::Program;
use crate::state::StateHolder;
use crate::tvm::Tvm;

mod callable;
mod frame;
mod function;
mod heap;
mod instruction;
mod native;
mod program;
mod stack;
mod state;
mod state_utils;
mod tvm;
mod ui;
mod program_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let program = Program::from_file("sq.json".to_string());
    let mut tvm = Tvm::default();
    tvm.load(program);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_tvm(&mut terminal, &mut tvm);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }
    Ok(())
}

fn run_tvm<B: Backend>(terminal: &mut Terminal<B>, tvm: &mut Tvm) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, tvm))?;
        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(()),
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => tvm.update_table_state(),
                (KeyCode::Char('t'), KeyModifiers::NONE) => {
                    tvm.tick();
                    tvm.update_table_state();
                }
                (KeyCode::Char('s'), KeyModifiers::NONE) => {
                    tvm.start();
                    tvm.update_table_state();
                }
                (KeyCode::Char('r'), KeyModifiers::NONE) => {
                    tvm.reset();
                    tvm.update_table_state();
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, tvm: &mut Tvm) {
    let main_layout = Layout::default()
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(45),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .margin(3)
        .direction(Direction::Horizontal)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Address", "Value"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let stack = tvm.get_stack_vec();
    let rows = stack.iter().map(|(k, v)| {
        let cells = vec![Cell::from(k.to_string()), Cell::from(v.to_string())];
        Row::new(cells)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Stack"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    let state_layout = Layout::default()
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .margin(0)
        .direction(Direction::Vertical)
        .split(main_layout[1]);

    let state = Paragraph::new(tvm.state.get_name()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Current State"),
    );

    let state_history = List::new(Tvm::state_history_to_list_items(&tvm.state_history))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("State History"),
        )
        .highlight_style(selected_style)
        .highlight_symbol(">> ");

    let mut state_history_state = ListState::default();
    if !tvm.state_history.is_empty() {
        state_history_state.select(Some(tvm.state_history.len() - 1));
    }

    f.render_widget(state, state_layout[0]);
    f.render_stateful_widget(state_history, state_layout[1], &mut state_history_state);

    let output_layout = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(0)
        .direction(Direction::Vertical)
        .split(main_layout[2]);
    let stdout = Paragraph::new(tvm.stdout.as_ref())
        .block(Block::default().borders(Borders::ALL).title("Stdout"));
    let log_items = Tvm::log_to_list_items(&tvm.log);
    let log = List::new(log_items.clone())
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ");
    if !log_items.is_empty() {
        tvm.log_state.select(Some(log_items.len() - 1));
    }
    f.render_stateful_widget(t, main_layout[0], &mut tvm.table_state);
    f.render_widget(stdout, output_layout[0]);
    f.render_stateful_widget(log, output_layout[1], &mut tvm.log_state);
}
