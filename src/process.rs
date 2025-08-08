use std::time::Duration;

use automation::{keyboard, mouse};
use clap::Parser;

use crate::args::{self, KeyboardCommand, MouseCommand, ScriptArgs};

pub fn entry() {
    let args = args::Args::parse();

    let result = match args.command {
        args::Command::Mouse(cmd) => handle_mouse_command(cmd),
        args::Command::Keyboard(cmd) => handle_keyboard_command(cmd),
        args::Command::Script(cmd) => handle_script_command(cmd),
    };

    // All application error handling happens here.
    match result {
        Ok(()) => {}
        Err(e) => {
            log::error!("Unexepectedly exited with error: {e}");
            eprintln!("Unexpectedly exited with error: {e}");
        }
    }
}

fn handle_mouse_command(command: MouseCommand) -> anyhow::Result<()> {
    let mouse = mouse::Mouse::new()?;

    use MouseCommand::*;
    match command {
        Click(button) => mouse.click(button.into())?,
        Release(button) => mouse.release(button.into())?,
        ClickAndHold { button, duration } => {
            mouse.click_for(Duration::from_secs_f32(duration), button.into())?
        }
    }

    Ok(())
}
fn handle_keyboard_command(command: KeyboardCommand) -> anyhow::Result<()> {
    let keyboard = keyboard::Keyboard::new()?;sdfd

    Ok(())
}
fn handle_script_command(command: ScriptArgs) -> anyhow::Result<()> {
    todo!()
}

impl From<args::MouseButton> for mouse::MouseButton {
    fn from(value: args::MouseButton) -> Self {
        use args::MouseButton as MB2;
        use mouse::MouseButton as MB1;
        match value {
            MB2::Left => MB1::Left,
            MB2::Middle => MB1::Middle,
            MB2::Right => MB1::Right,
        }
    }
}

#[test]
fn test() {}
