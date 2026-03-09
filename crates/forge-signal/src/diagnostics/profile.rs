use serde::{Deserialize, Serialize};

/// Runtime diagnostics richness profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DiagnosticsProfile {
    /// Low-overhead summaries and bounded detail for live systems.
    #[default]
    Operational,
    /// Richer detail suitable for everyday debugging.
    Development,
    /// Deep diagnostics for hard investigations.
    Forensic,
}

impl DiagnosticsProfile {
    /// Maximum number of retained execution-history summaries.
    pub fn history_limit(self) -> usize {
        match self {
            Self::Operational => 4,
            Self::Development => 16,
            Self::Forensic => 64,
        }
    }

    /// Maximum number of node-level details retained in summary collections.
    pub fn detail_limit(self) -> usize {
        match self {
            Self::Operational => 16,
            Self::Development => 64,
            Self::Forensic => 256,
        }
    }

    /// Whether this profile should retain detailed node-level history.
    pub fn retains_history_details(self) -> bool {
        !matches!(self, Self::Operational)
    }

    /// Whether this profile should retain richer explanation detail in flow diagnostics.
    pub fn retains_flow_explanation(self) -> bool {
        matches!(self, Self::Development | Self::Forensic)
    }

    /// Whether rendering should include richer contextual detail.
    pub fn renders_rich_detail(self) -> bool {
        matches!(self, Self::Development | Self::Forensic)
    }
}
