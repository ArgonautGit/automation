#![allow(dead_code)]

use inputbot::KeybdKey;
use rustautogui::{RustAutoGui, errors::AutoGuiError};
use std::{thread, time::Duration};

use crate::error::AutoError;

pub struct Keyboard(RustAutoGui);
impl Keyboard {
    /// Constructor
    pub fn new() -> Result<Self, AutoError> {
        Ok(Self(RustAutoGui::new(true)?))
    }

    /// Instantly press and release.
    pub fn press(&self, key: char) -> Result<(), AutoError> {
        log::trace!("Pressing {key} key");
        let key = &key.to_string();
        self.0.key_down(key)?;
        self.0.key_up(key)?;
        Ok(())
    }
    /// Press and hold.
    pub fn hold(&self, key: char) -> Result<(), AutoError> {
        log::trace!("Holding {key} key");
        self.0.key_down(&key.to_string())?;
        Ok(())
    }
    /// Release key.
    pub fn release(&self, key: char) -> Result<(), AutoError> {
        log::trace!("Releasing {key} key");
        self.0.key_up(&key.to_string())?;
        Ok(())
    }
    /// Type an entire input string to the keyboard.
    pub fn type_string(&self, input: &str) -> Result<(), AutoError> {
        log::trace!("Typing {input} string");
        self.0.keyboard_input(input)?;
        Ok(())
    }
    /// Press and hold for the specified `duration`
    pub fn press_for(&self, key: char, duration: Duration) -> Result<(), AutoError> {
        log::trace!("Pressing {key} for {} seconds", duration.as_secs_f32());
        let key = &key.to_string();
        self.0.key_down(key)?;
        thread::sleep(duration);
        self.0.key_up(key)?;
        Ok(())
    }
    /// Place a global input hook on the selected key. The provided closure will
    /// run when this key is pressed.
    pub fn callback<F>(&self, key: char, callback: F) -> Result<(), AutoError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        log::trace!("Placing input hook on {key}");
        let key = KeyboardKey::from(key);
        inputbot::KeybdKey::from(key).bind(callback);
        Ok(())
    }
}

struct KeyboardKey(inputbot::KeybdKey);
impl From<char> for KeyboardKey {
    fn from(value: char) -> Self {
        Self(inputbot::KeybdKey::from(u64::from(value)))
    }
}
impl TryFrom<&str> for KeyboardKey {
    type Error = AutoGuiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.len() != 1 {
            return Err(AutoGuiError::UnSupportedKey(value.to_owned()));
        }

        let key = value.chars().last();
        let key = key.ok_or_else(|| AutoGuiError::UnSupportedKey(value.to_owned()))?;
        Ok(KeyboardKey::from(key))
    }
}
impl From<KeyboardKey> for KeybdKey {
    fn from(value: KeyboardKey) -> Self {
        let char = char::from(value);
        let u64 = u64::from(char);
        KeybdKey::from(u64)
    }
}
impl From<KeyboardKey> for char {
    fn from(value: KeyboardKey) -> Self {
        // This should not fail since KeyboardKey is guaranteed to be closed under chars.
        inputbot::from_keybd_key(value.0).expect("This should NOT fail.")
    }
}
