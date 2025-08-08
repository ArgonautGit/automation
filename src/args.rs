use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand, alias = "m")]
    Mouse(MouseCommand),
    #[command(subcommand, alias = "k")]
    Keyboard(KeyboardCommand),
    Script(ScriptArgs),
}

#[derive(Subcommand, Debug)]
pub enum MouseCommand {
    #[command(subcommand)]
    Click(MouseButton),
    #[command(subcommand)]
    Release(MouseButton),
    ClickAndHold {
        #[command(subcommand)]
        button: MouseButton,
        duration: f32,
    },
}

#[derive(Subcommand, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Subcommand, Debug)]
pub enum KeyboardCommand {
    Press { key: char },
    Type { string: String },
}

#[derive(Parser, Debug)]
pub struct ScriptArgs {
    #[arg(short, long)]
    file: PathBuf,
}
