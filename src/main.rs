use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::{KeyModifiers, ModifierKeyCode};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};
use tui::layout::Direction;
use tui::widgets::Paragraph;
use tvm_rs::vm::program::Program;
use tvm_rs::vm::tvm::Tvm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let program = Program::from_file("sq.json".to_string());
    // println!("{}", program);
    let mut tvm = Tvm::new(program);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_tvm(&mut terminal, &mut tvm);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
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
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => tvm.memory.update_state(),
                (KeyCode::Down, _) => tvm.memory.next(),
                (KeyCode::Up, _) => tvm.memory.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, tvm: &mut Tvm) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
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
    let active = tvm.memory.get_active_mem();
    let rows = active.iter().map(|(k, v)| {
        let cells = vec![Cell::from(k.to_string()), Cell::from(v.to_string())];
        Row::new(cells)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    let b = Paragraph::new(tvm.stdout.as_ref()).block(Block::default().borders(Borders::ALL).title("Output"));
    f.render_stateful_widget(t, rects[0], &mut tvm.memory.state);
    f.render_widget(b, rects[1]);
}
