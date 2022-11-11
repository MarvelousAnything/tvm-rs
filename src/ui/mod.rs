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

impl TvmUI {
    pub fn new (controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();
        let mut ui = TvmUI {
            cursive: Cursive::new(),
            ui_tx,
            ui_rx,
            controller_tx
        };

        ui
    }
}

pub struct Controller {
    rx: mpsc::Receiver<ControllerMessage>,
    ui: TvmUI,
}

impl Controller {
    pub fn new() -> Result<Controller, String> {
        let (tx, rx) = mpsc::channel::<ControllerMessage>();
        Ok(Controller {
            rx,
            ui: TvmUI::new(tx.clone()),
        })
    }
}