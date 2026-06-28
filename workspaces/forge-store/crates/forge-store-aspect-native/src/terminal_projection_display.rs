use crate::StoreTerminalProjectionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreTerminalProjectionDisplayLabel {
    terminal_display_label: String,
}

impl StoreTerminalProjectionDisplayLabel {
    pub fn new(
        terminal_display_label: impl Into<String>,
    ) -> Result<Self, StoreTerminalProjectionDenial> {
        let terminal_display_label = terminal_display_label.into();
        if terminal_display_label.trim().is_empty() {
            return Err(StoreTerminalProjectionDenial::EmptyTerminalProjectionDisplayLabel);
        }

        Ok(Self {
            terminal_display_label,
        })
    }

    pub fn terminal_display_label(&self) -> &str {
        &self.terminal_display_label
    }
}
