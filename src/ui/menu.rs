use std::time::Duration;

use egui::Button;
use inputbot::KeybdKey;

use crate::{
    automation::mouse,
    ui::{AutoGui, LogError},
};

pub fn build(state: &mut AutoGui, ui: &mut egui::Ui) {
    let record_and_replay_button =
        ui.vertical_centered(|ui| ui.add(Button::new("Record and Replay"))).inner;
    // Record and replay button.
    if record_and_replay_button.clicked() {
        log::info!("Replay button clicked");

        let (send, recv) = smol::channel::bounded(1);
        state.escape_macro = Some(send); // Initialize sender.
        smol::spawn(async move {
            // Create recording thread and watcher for user cancellation.
            let recording_task =
                mouse::MouseRecording::new(Duration::from_secs(3), Duration::from_millis(1));
            let cancel_task = async {
                recv.recv().await.log_error();
                Err(mouse::Error::MacroCancel)
            };

            // Finish or stop if cancelled early.
            let recording = match smol::future::or(recording_task, cancel_task).await {
                Ok(recording) => {
                    log::info!("Recording finished");
                    recording
                }
                Err(mouse::Error::MacroCancel) => {
                    log::info!("Macro cancelled");
                    return;
                }
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            // Create replay thread and watcher for cancellation.
            let cancel_task = async {
                recv.recv().await.log_error();
                Err(mouse::Error::MacroCancel)
            };
            let replay_task = recording.replay();
            // Finish or stop if cancelled early.
            match smol::future::or(replay_task, cancel_task).await {
                Err(mouse::Error::MacroCancel) => log::info!("Replay cancelled"),
                Err(e) => log::error!("{e}"),
                Ok(_) => log::info!("Replay finished"),
            };
        })
        .detach();
    }

    // Cancel macro button.
    let cancel_macro_button = ui.vertical_centered(|ui| ui.button("Cancel Macro")).inner;
    if cancel_macro_button.clicked() {
        log::info!("Cancel macro button clicked");
        if let Some(sender) = state.escape_macro.take() {
            sender.send_blocking(true).log_error();

            // What it actually does.
            if let Err(e) = sender.send_blocking(true) {
                log::error!("{e}")
            }
        }
    }

    let mut key_input = match state.cancel_key.clone() {
        Some(key_input) => key_input,
        None => char::default().to_string(),
    };
    let cancel_key_input = ui.vertical_centered(|ui| ui.text_edit_singleline(&mut key_input)).inner;
    if cancel_key_input.clicked() {
        log::info!("Pressed");
        let key = KeybdKey::from(u64::from('A'));
        key.bind(|| log::info!("Keybind pressed!"));
        inputbot::handle_input_events();
    }
}
