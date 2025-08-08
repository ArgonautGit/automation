#![allow(dead_code)]

use std::time::Duration;

use rustautogui::{MouseClick, RustAutoGui};

use crate::error::AutoError;

pub struct Mouse(rustautogui::RustAutoGui);
impl Mouse {
    /// Constructor
    pub fn new() -> Result<Self, AutoError> {
        Ok(Self(RustAutoGui::new(true)?))
    }
    
    // Mouse movement \\
    pub fn move_to(
        &self, position: (Option<u32>, Option<u32>), duration: Option<Duration>,
    ) -> Result<(), AutoError> {
        match duration {
            Some(duration) => {
                self.0.move_mouse_to(position.0, position.1, duration.as_secs_f32())?
            }
            None => self.0.move_mouse_to(position.0, position.1, 0.0)?,
        }
        Ok(())
    }
    pub fn move_by(&self, distance: (i32, i32), duration: Option<Duration>) -> Result<(), AutoError> {
        match duration {
            Some(duration) => self.0.move_mouse(distance.0, distance.1, duration.as_secs_f32())?,
            None => self.0.move_mouse(distance.0, distance.1, 0.0)?,
        }
        Ok(())
    }

    // Mouse buttons \\
    pub fn click(&self, button: MouseButton) -> Result<(), AutoError> {
        self.0.click(button.into())?;
        Ok(())
    }
    pub fn hold(&self, button: MouseButton) -> Result<(), AutoError> {
        self.0.click_down(button.into())?;
        Ok(())
    }
    pub fn release(&self, button: MouseButton) -> Result<(), AutoError> {
        self.0.click_up(button.into())?;
        Ok(())
    }
    pub fn scroll(&self, distance: i32) -> Result<(), AutoError> {
        if distance > 0 {
            self.0.scroll_up(distance as u32)?;
        } else {
            self.0.scroll_down(distance as u32)?;
        }
        Ok(())
    }
    pub fn click_and_drag(
        &self, to: (Option<u32>, Option<u32>), duration: Option<Duration>, button: MouseButton,
    ) -> Result<(), AutoError> {
        self.hold(button)?;
        self.move_to(to, duration)?;
        self.release(button)?;
        Ok(())
    }
    pub fn click_and_drag_by(
        &self, to: (i32, i32), duration: Option<Duration>, button: MouseButton,
    ) -> Result<(), AutoError> {
        self.hold(button)?;
        self.move_by(to, duration)?;
        self.release(button)?;
        Ok(())
    }
    pub fn click_for(&self, duration: Duration, button: MouseButton) -> Result<(), AutoError> {
        self.hold(button)?;
        std::thread::sleep(duration);
        self.release(button)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
impl From<MouseButton> for MouseClick {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => MouseClick::LEFT,
            MouseButton::Right => MouseClick::RIGHT,
            MouseButton::Middle => MouseClick::MIDDLE,
        }
    }
}
