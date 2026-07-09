use serde::{Deserialize, Serialize};

/// Canonical diagnostics access/richness class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DiagnosticsTier {
    /// Low-overhead retained summaries for live systems.
    #[default]
    Operational,
    /// Richer retained detail for everyday debugging.
    Development,
    /// Deep retained detail for hard investigations.
    Forensic,
}

impl DiagnosticsTier {
    pub fn label(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Development => "development",
            Self::Forensic => "forensic",
        }
    }

    pub fn renders_rich_detail(self) -> bool {
        matches!(self, Self::Development | Self::Forensic)
    }
}
