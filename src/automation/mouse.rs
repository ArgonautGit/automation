#![allow(dead_code)]

use std::time::{Duration, Instant};

use rustautogui::{MouseClick, RustAutoGui, errors::AutoGuiError};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CursorPosition(pub i32, pub i32);

#[allow(dead_code)]
pub fn get_position() -> Result<CursorPosition, Error> {
    let auto = RustAutoGui::new(false)?;
    let (x, y) = auto.get_mouse_position()?;
    Ok(CursorPosition(x, y))
}

#[allow(dead_code)]
pub fn move_to(position: &CursorPosition) -> Result<(), Error> {
    let auto = RustAutoGui::new(false)?;
    auto.move_mouse_to(Some(position.0 as u32), Some(position.1 as u32), 0.0)?;
    Ok(())
}

#[allow(dead_code)]
pub fn click(button: MouseClick) -> Result<(), Error> {
    let auto = RustAutoGui::new(false)?;
    auto.click(button)?;
    Ok(())
}

pub struct MouseRecording {
    path: Vec<CursorPosition>,
    polling_rate: Duration,
}

impl MouseRecording {
    /// Start recording a mouse path immediately for as long as `duration`.
    /// `polling_rate` is the time between each cursor position snapshot.
    /// `escape_flag` is to tell the function to cancel.
    pub async fn new(duration: Duration, polling_rate: Duration) -> Result<MouseRecording, Error> {
        if polling_rate > Duration::from_millis(100) {
            return Err(Error::Resolution);
        }

        log::info!("Starting mouse path recording");

        // Spawn a new thread to avoid blocking the gui thread.
        let task: smol::Task<Result<Vec<CursorPosition>, Error>> = smol::spawn(async move {
            let mut path = Vec::new();
            let start = Instant::now();
            // Until the required duration has passed.
            while start.elapsed() < duration {

                path.push(get_position()?);
                smol::Timer::after(polling_rate).await;
            }

            Ok(path)
        });

        // Avoid blocking the thread.
        let path = task.await?;

        Ok(MouseRecording { path, polling_rate })
    }

    // Asynchrously replay mouse path.
    pub async fn replay(&self) -> Result<(), Error> {
        for position in &self.path {
            move_to(position)?;
            // Avoid blocking the thread.
            smol::Timer::after(self.polling_rate).await;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("AutoGuiError occured: {0}")]
    AutoGui(#[from] AutoGuiError),
    #[error("Replay resolution is too coarse")]
    Resolution,
    #[error("Replay error")]
    Replay,
    #[error("Macro cancelled")]
    MacroCancel,
}

#[test]
fn testing() -> Result<(), Error> {
    // MouseRecording::new(Duration::from_secs(3), Duration::from_millis(10)).await.unwrap();

    Ok(())
}
