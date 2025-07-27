use eframe::egui;
use egui::Pos2;

mod menu;

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
    // Put state here.
}

impl eframe::App for AutoGui {
    // Runs every frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AutoGui");
        });
    }
}
