//! Progress indicators for CLI operations

use indicatif::{ProgressBar, ProgressStyle};
use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// Global flag to disable colors (set from --no-color)
static NO_COLOR: AtomicBool = AtomicBool::new(false);

/// Set whether colors should be disabled
pub fn set_no_color(no_color: bool) {
    NO_COLOR.store(no_color, Ordering::Relaxed);
}

/// Check if colors are disabled
pub fn is_no_color() -> bool {
    NO_COLOR.load(Ordering::Relaxed) || std::env::var("NO_COLOR").is_ok()
}

/// A spinner that only shows when connected to a TTY
pub struct Spinner {
    bar: Option<ProgressBar>,
}

impl Spinner {
    /// Create a new spinner with the given message
    /// Only shows if stderr is connected to a TTY
    pub fn new(message: &str) -> Self {
        if std::io::stderr().is_terminal() {
            let bar = ProgressBar::new_spinner();

            // Use cyan for spinner - visible on both light and dark terminals
            // Fallback to no color if --no-color or NO_COLOR env is set
            let template = if is_no_color() {
                "{spinner} {msg}"
            } else {
                // Cyan is ANSI 36, readable on both dark and light backgrounds
                "{spinner:.cyan} {msg}"
            };

            bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template(template)
                    .expect("Invalid spinner template"),
            );
            bar.set_message(message.to_string());
            bar.enable_steady_tick(Duration::from_millis(80));
            Self { bar: Some(bar) }
        } else {
            Self { bar: None }
        }
    }

    /// Update the spinner message
    #[allow(dead_code)]
    pub fn set_message(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.set_message(message.to_string());
        }
    }

    /// Finish the spinner with a success message (clears the line)
    pub fn finish(&self) {
        if let Some(bar) = &self.bar {
            bar.finish_and_clear();
        }
    }

    /// Finish the spinner with a custom message
    #[allow(dead_code)]
    pub fn finish_with_message(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.finish_with_message(message.to_string());
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if let Some(bar) = &self.bar {
            bar.finish_and_clear();
        }
    }
}

/// Execute an async operation with a spinner
pub async fn with_spinner<F, T, E>(message: &str, future: F) -> Result<T, E>
where
    F: std::future::Future<Output = Result<T, E>>,
{
    let spinner = Spinner::new(message);
    let result = future.await;
    spinner.finish();
    result
}
