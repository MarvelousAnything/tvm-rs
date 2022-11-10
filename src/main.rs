use crossterm::event::{KeyModifiers};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


use std::{error::Error, io};
use std::cell::RefCell;
use std::rc::Rc;
use cursive::{Cursive, CursiveExt, view, With};
use cursive::event::Key;
use cursive::traits::Nameable;
use cursive::views::{ListView, TextView};

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
mod instruction_tests;
mod native;
mod program;
mod stack;
mod state;
mod state_utils;
mod tvm;
mod ui;

fn main() {

    let tvm = Rc::new(RefCell::new(Tvm::default()));
    let program = Program::from_file("sq.json".to_string());
    tvm.borrow_mut().load(program);
    tvm.borrow_mut().start();

    let mut siv = Cursive::default();
    siv.add_layer(
        cursive::views::Dialog::around(
            ListView::new()
                .with(|list| {
                    tvm.clone().borrow().get_active_memory()
                        .iter()
                        .for_each(|(addr, val)| {
                            list.add_child(format!("{}", addr).as_str(), TextView::new(format!("{}", val)));
                        });
                }).with_name("stack")
        )
            .button("Quit", |s| s.quit()),
    );

    siv.add_global_callback(' ', move |s| {
        s.call_on_name("stack", |view: &mut ListView| {
            view.clear();
            tvm.clone().borrow().get_active_memory()
                .iter()
                .for_each(|(addr, val)| {
                    view.add_child(format!("{}", addr).as_str(), TextView::new(format!("{}", val)));
                });
        });
    });

    siv.add_global_callback(Key::Tab, move |_s| {
        tvm.borrow_mut().tick();
    });

    siv.set_autorefresh(true);
    siv.run();
}