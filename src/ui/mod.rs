use std::{
    sync::{Arc, atomic::AtomicBool},
    thread,
    time::Duration,
};

use eframe::{egui, glow::REPLACE};
use egui::Button;

mod menu;

use crate::automation::{self, mouse::CursorPosition};

use super::automation::mouse;

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((300.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Test App",
        options,
        Box::new(|_cc| Ok(Box::<AutoGui>::default())),
    )
}

#[derive(Default)]
struct AutoGui {
    escape_macro: Arc<AtomicBool>,
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
