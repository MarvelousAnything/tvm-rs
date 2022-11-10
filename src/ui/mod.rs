use std::sync::mpsc;
use cursive::Cursive;

pub enum UiMessage {
    UpdateOutput(String),
}

pub enum ControllerMessage {
    UpdatedInputAvailable(String),
}

pub struct TvmUI {
    cursive: Cursive,
    ui_rx: mpsc::Receiver<UiMessage>,
    ui_tx: mpsc::Sender<UiMessage>,
    controller_tx: mpsc::Sender<ControllerMessage>,
}

pub struct Controller {
    rx: mpsc::Receiver<ControllerMessage>,
    ui: TvmUI,
}