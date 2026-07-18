use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProofProcessModel {
    LibtestProcess,
    RustdocTestProcess,
    NestedCargoProcess,
    StandardizedUiHarness,
    CargoCheckProcess,
    LibtestWithDeclaredSubprocesses,
    LibtestWithFreshChildProcess,
    LibtestWithNestedCargoProcess,
    AllocatorGlobalProcess,
}

impl ProofProcessModel {
    pub const fn requires_ui_proof_evidence(self) -> bool {
        matches!(self, Self::StandardizedUiHarness)
    }

    pub const fn requires_process_probe_evidence(self) -> bool {
        matches!(
            self,
            Self::LibtestWithFreshChildProcess | Self::LibtestWithDeclaredSubprocesses
        )
    }

    pub const fn is_plain_libtest(self) -> bool {
        matches!(self, Self::LibtestProcess)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LibtestProcess => "libtest-process",
            Self::RustdocTestProcess => "rustdoc-test-process",
            Self::NestedCargoProcess => "nested-cargo-process",
            Self::StandardizedUiHarness => "standardized-ui-harness",
            Self::CargoCheckProcess => "cargo-check-process",
            Self::LibtestWithDeclaredSubprocesses => "libtest-with-declared-subprocesses",
            Self::LibtestWithFreshChildProcess => "libtest-with-fresh-child-process",
            Self::LibtestWithNestedCargoProcess => "libtest-with-nested-cargo-process",
            Self::AllocatorGlobalProcess => "allocator-global-process",
        }
    }
}

impl fmt::Display for ProofProcessModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
