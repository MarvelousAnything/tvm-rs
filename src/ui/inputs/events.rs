use std::sync::mpsc::{Receiver, Sender};
use crate::ui::inputs::key::Key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
}

pub struct Events {
    rx: Receiver<InputEvent>,
    tx: Sender<InputEvent>,
}