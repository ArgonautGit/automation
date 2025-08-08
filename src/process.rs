use std::time::Duration;

use automation::{keyboard, mouse};
use clap::Parser;

use crate::args::{self, KeyboardCommand, MouseCommand, ScriptArgs};

pub fn entry() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let result = match args.command {
        args::Command::Mouse(cmd) => handle_mouse_command(cmd),
        args::Command::Keyboard(cmd) => handle_keyboard_command(cmd),
        args::Command::Script(cmd) => handle_script_command(cmd),
    };

    // All application error handling happens here.
    return result;
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
    let keyboard = keyboard::Keyboard::new()?;

    use KeyboardCommand::*;
    match command {
        Press { key } => keyboard.press(key)?,
        Type { ref string } => keyboard.type_string(string)?,
    }

    Ok(())
}
fn handle_script_command(_command: ScriptArgs) -> anyhow::Result<()> {
    unimplemented!()
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
