#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser;

mod args;
mod process;

fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Trace).expect("failed to start logger");
    args::Args::parse();

    return process::entry()
}
