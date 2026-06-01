use super::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryPostureKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrdinaryPosture {
    reason: String,
    kind: ForgeQueryOrdinaryPostureKind,
    next_step: ForgeQueryOrdinaryNextStep,
    checked_topology: ForgeQueryOrdinaryCheckedTopology,
}

impl ForgeQueryOrdinaryPosture {
    pub(crate) fn new(
        reason: impl Into<String>,
        kind: ForgeQueryOrdinaryPostureKind,
        next_step: ForgeQueryOrdinaryNextStep,
        checked_topology: ForgeQueryOrdinaryCheckedTopology,
    ) -> Self {
        Self {
            reason: reason.into(),
            kind,
            next_step,
            checked_topology,
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn kind(&self) -> ForgeQueryOrdinaryPostureKind {
        self.kind
    }

    pub fn next_step(&self) -> ForgeQueryOrdinaryNextStep {
        self.next_step
    }

    pub fn checked_topology(&self) -> &ForgeQueryOrdinaryCheckedTopology {
        &self.checked_topology
    }
}
