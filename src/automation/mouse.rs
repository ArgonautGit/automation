use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};

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
    should_cancel: Arc<AtomicBool>,
}

impl MouseRecording {
    /// Start recording a mouse path immediately for as long as `duration`.
    /// `polling_rate` is the time between each cursor position snapshot.
    /// `escape_flag` is to tell the function to cancel.
    pub async fn new(
        duration: Duration,
        polling_rate: Duration,
        escape_flag: Arc<AtomicBool>, // Should we cancel the macro?
    ) -> Result<MouseRecording, Error> {
        if polling_rate > Duration::from_millis(100) {
            return Err(Error::ResolutionError);
        }

        log::info!("Starting mouse path recording");

        // Spawn a new thread to avoid blocking the gui thread.
        let moved_escape_flag = Arc::clone(&escape_flag); // Pas
        let task: smol::Task<Result<Vec<CursorPosition>, Error>> = smol::spawn(async move {
            let mut path = Vec::new();
            let start = Instant::now();
            while start.elapsed() < duration {
                // Until the required duration has passed.
                // If we should cancel the macro.
                if moved_escape_flag.load(Ordering::Relaxed) {
                    moved_escape_flag.store(false, Ordering::Relaxed); // Say we cancelled.
                    log::info!("Recording macro cancelled");
                    return Err(Error::MacroCancel);
                }

                path.push(get_position()?);
                smol::Timer::after(polling_rate).await;
            }

            Ok(path)
        });

        // Avoid blocking the thread.
        let path = task.await?;

        Ok(MouseRecording {
            path,
            polling_rate,
            should_cancel: escape_flag,
        })
    }

    // Asynchrously replay mouse path.
    #[allow(dead_code)]
    pub async fn replay(&self) -> Result<(), Error> {
        for position in &self.path {
            // Check if replay was cancelled.
            if self.should_cancel.load(Ordering::Relaxed) {
                self.should_cancel.store(false, Ordering::Relaxed);
                log::info!("Cancelling mouse path replay");
                break;
            }

            move_to(&position)?;
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
    AutoGuiError(#[from] AutoGuiError),
    #[error("Replay resolution is too coarse")]
    ResolutionError,
    #[error("Replay error")]
    ReplayError,
    #[error("Macro cancelled")]
    MacroCancel,
}

#[test]
fn testing() -> Result<(), Error> {
    // MouseRecording::new(Duration::from_secs(3), Duration::from_millis(10)).await.unwrap();

    Ok(())
}
