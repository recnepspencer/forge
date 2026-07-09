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
