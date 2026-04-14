use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthDiagnosticsAspect {
    Decisions,
    Interpretations,
}

impl WorthDiagnosticsAspect {
    pub const ALL: [Self; 2] = [Self::Decisions, Self::Interpretations];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Decisions => "diagnostics",
            Self::Interpretations => "diagnostics.interpretations",
        }
    }
}
