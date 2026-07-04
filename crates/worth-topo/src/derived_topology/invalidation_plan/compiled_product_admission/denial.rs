use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyCompiledProductAdmissionErrorKind {
    NoDeclaredFamilyForConsumer,
    InvalidTruthBasisCount,
    SelectedPlanRequired,
    TouchedClosureNotBoundToSelectedPlan,
    ReadBasisNotBoundToTouchedClosure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyCompiledProductAdmissionError {
    kind: TopologyCompiledProductAdmissionErrorKind,
    message: String,
}

impl TopologyCompiledProductAdmissionError {
    pub fn new(
        kind: TopologyCompiledProductAdmissionErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub const fn kind(&self) -> TopologyCompiledProductAdmissionErrorKind {
        self.kind
    }
}

impl core::fmt::Display for TopologyCompiledProductAdmissionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TopologyCompiledProductAdmissionError {}
