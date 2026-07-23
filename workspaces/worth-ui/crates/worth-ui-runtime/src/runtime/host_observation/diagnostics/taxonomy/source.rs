use crate::runtime::{
    WorthUiExecutionLane, WorthUiReloadCheckedStopPosture, WorthUiReloadFailureStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiDiagnosticSource {
    ReloadFailure {
        stage: WorthUiReloadFailureStage,
        checked_stop_posture: WorthUiReloadCheckedStopPosture,
        upstream_evidence_digest: Option<u64>,
    },
    PhaseDenial {
        evidence_digest: u64,
    },
    LaneAdmission {
        lane: Option<WorthUiExecutionLane>,
        evidence_digest: u64,
    },
    ProjectionHook {
        hook_digest: u64,
    },
}

impl WorthUiDiagnosticSource {
    pub fn evidence_digest(self) -> Option<u64> {
        match self {
            Self::ReloadFailure {
                upstream_evidence_digest,
                ..
            } => upstream_evidence_digest,
            Self::PhaseDenial { evidence_digest }
            | Self::LaneAdmission {
                evidence_digest, ..
            } => Some(evidence_digest),
            Self::ProjectionHook { hook_digest } => Some(hook_digest),
        }
    }
}
