#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use log::LevelFilter;

mod automation;
mod ui;

fn main() -> eframe::Result {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Off)
        .with_module_level("automation", LevelFilter::Debug)
        .init()
        .expect("Failed to start logger");

    ui::run()
}
