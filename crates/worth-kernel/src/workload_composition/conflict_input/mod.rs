mod handoff_guards;
mod spatial;
mod topology;

pub use spatial::{
    admit_spatial_conflict_input, AdmittedSpatialConflictInput, AdmittedSpatialConflictRoute,
    SpatialConflictInputRequest,
};
pub use topology::{
    admit_topology_conflict_input, AdmittedTopologyConflictInput, AdmittedTopologyConflictRoute,
    TopologyConflictInputRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictInputAdmissionErrorKind {
    MissingTopologyConflictRoute,
    MissingTouchedAspect,
    MissingTouchedParticipants,
    MissingSpatialConflictRoute,
    WrongAuthority,
    WrongReceiptFamily,
    StageIndexMismatch,
    RawRowScanDenied,
    BroadReceiptScanDenied,
    CallerOwnedScanDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictInputAdmissionError {
    kind: ConflictInputAdmissionErrorKind,
    detail: String,
}

impl ConflictInputAdmissionError {
    pub(crate) fn new(kind: ConflictInputAdmissionErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ConflictInputAdmissionErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[cfg(test)]
mod tests;
