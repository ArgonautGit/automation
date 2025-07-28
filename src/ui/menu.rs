use std::{sync::atomic::Ordering, time::Duration};

use egui::Button;

use crate::{automation::mouse, ui::AutoGui};

pub fn build(state: &mut AutoGui, ui: &mut egui::Ui) {
    let record_and_replay_button = ui
        .vertical_centered(|ui| ui.add(Button::new("Record and Replay")))
        .inner;

    let should_escape = state.escape_macro.clone();
    if record_and_replay_button.clicked() {
        log::debug!("Replay button clicked");
        smol::spawn(async move {
            let recording = mouse::MouseRecording::new(
                Duration::from_secs(3),
                Duration::from_millis(1),
                should_escape,
            )
            .await
            .unwrap();

            recording.replay().await.unwrap();
        })
        .detach();
    }

    let cancel_macro_button = ui.vertical_centered(|ui| ui.button("Cancel Macro")).inner;
    if cancel_macro_button.clicked() {
        state.escape_macro.store(true, Ordering::Relaxed);
    }
}

mod components {}
