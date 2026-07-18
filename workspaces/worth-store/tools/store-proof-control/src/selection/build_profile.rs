use serde::{Deserialize, Serialize};

use super::StoreProofMode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum StoreBuildProfileIdentity {
    LocalTest,
    CiTest,
    Release,
    DiagnosticTest,
}

impl StoreBuildProfileIdentity {
    pub(crate) const fn for_mode(mode: StoreProofMode) -> Self {
        match mode {
            StoreProofMode::Owner | StoreProofMode::Smoke | StoreProofMode::Ui => Self::LocalTest,
            StoreProofMode::Ci | StoreProofMode::Soak => Self::CiTest,
            StoreProofMode::Release | StoreProofMode::Hardware => Self::Release,
        }
    }

    pub const fn cargo_profile(self) -> &'static str {
        match self {
            Self::LocalTest => "test",
            Self::CiTest => "ci-test",
            Self::Release => "release",
            Self::DiagnosticTest => "diagnostic-test",
        }
    }
}
