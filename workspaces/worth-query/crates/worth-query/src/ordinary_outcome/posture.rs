use super::{
    WorthQueryOrdinaryCheckedTopology, WorthQueryOrdinaryNextStep, WorthQueryOrdinaryPostureKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrdinaryPosture {
    reason: String,
    kind: WorthQueryOrdinaryPostureKind,
    next_step: WorthQueryOrdinaryNextStep,
    checked_topology: WorthQueryOrdinaryCheckedTopology,
}

impl WorthQueryOrdinaryPosture {
    pub fn new(
        reason: impl Into<String>,
        kind: WorthQueryOrdinaryPostureKind,
        next_step: WorthQueryOrdinaryNextStep,
        checked_topology: WorthQueryOrdinaryCheckedTopology,
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

    pub fn kind(&self) -> WorthQueryOrdinaryPostureKind {
        self.kind
    }

    pub fn next_step(&self) -> WorthQueryOrdinaryNextStep {
        self.next_step
    }

    pub fn checked_topology(&self) -> &WorthQueryOrdinaryCheckedTopology {
        &self.checked_topology
    }
}
