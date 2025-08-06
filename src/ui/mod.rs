use std::fmt::Display;

use eframe::egui;
use smol::channel::Sender;

mod menu;

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((300.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native("Test App", options, Box::new(|_cc| Ok(Box::<AutoGui>::default())))
}

#[derive(Default)]
struct AutoGui {
    escape_macro: Option<Sender<bool>>,
    cancel_key: Option<String>,
}

impl eframe::App for AutoGui {
    // Runs every frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AutoGui");
            menu::build(self, ui);
        });
    }
}

/// Extension trait for lazy logging of error results.
trait LogError {
    /// Discard result and log if error.
    fn log_error(&self);
}

impl<T, E: Display> LogError for Result<T, E> {
    fn log_error(&self) {
        if let Err(error) = self {
            log::error!("{error}");
        }
    }
}
