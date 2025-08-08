use rustautogui::errors::AutoGuiError;

#[derive(thiserror::Error, Debug)]
pub enum AutoError {
    #[error("Error invoking mouse")]
    Mouse(#[from] AutoGuiError),
    #[error("Error invoking keyboard")]
    Keyboard(AutoGuiError),
}