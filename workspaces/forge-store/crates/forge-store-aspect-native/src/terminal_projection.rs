#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreTerminalProjectionText {
    terminal_projection_text: String,
}

impl StoreTerminalProjectionText {
    pub fn new_terminal_projection_text(terminal_projection_text: impl Into<String>) -> Self {
        Self {
            terminal_projection_text: terminal_projection_text.into(),
        }
    }

    pub fn terminal_projection_text(&self) -> &str {
        &self.terminal_projection_text
    }
}
mod terminal_projection_denial;
mod terminal_projection_digest_separation;
mod terminal_projection_display;

pub use terminal_projection_denial::StoreTerminalProjectionDenial;
pub use terminal_projection_digest_separation::{
    StoreTerminalChecksumAlgorithm, StoreTerminalChecksumScope, StoreTerminalDocumentChecksum,
    StoreTerminalProjectionDocumentBytes,
};
pub use terminal_projection_display::StoreTerminalProjectionDisplayLabel;
