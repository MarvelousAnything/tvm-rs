use std::{io};

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, event, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{Event, KeyCode, KeyEvent};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use tvm_rs::ui::ui;
use tvm_rs::vm::tvm::Tvm;

pub fn start_ui(tvm: &Tvm) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            f.render_widget(ui::to_table(tvm), f.size());
        })?;
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            if let KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE, ..
                } = event { break }
        };
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() {
    let mut tvm = Tvm::new("sq.json".to_string());
    tvm.bootstrap();
    start_ui(&tvm).unwrap();
}
