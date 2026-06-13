use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NmtRadialFanDenial {
    MissingReceiptBackedStage(WorkloadEvidenceStage),
    WrongTopologyPattern { pattern_name: String },
    WrongTopologyPosture { posture: String },
    MismatchedTopologyConstructionReceipt,
    MismatchedProjectionReceipt,
    MismatchedTransformReceipt,
    MismatchedRetainedReplayReceipt,
    MissingOpenBoundaryEvidence,
    MissingRadialAdjacencyEvidence,
    InsufficientIncidentFaces { incident_faces: usize },
    MissingTopologyEvidence,
    MissingProjectionEvidence,
    MissingTransformEvidence,
    LabelOnlyMotion,
    MissingRetainedReplayEvidence,
    ClosedManifoldLaunderingAttempt { source_identity: String },
    UnsupportedSurfaceFamily { family: String },
    DirtyInput { reason: String },
    PredicateUncertain { reason: String },
}

impl NmtRadialFanDenial {
    pub fn closed_manifold_laundering_attempt(source_identity: impl Into<String>) -> Self {
        Self::ClosedManifoldLaunderingAttempt {
            source_identity: source_identity.into(),
        }
    }

    pub fn unsupported_surface_family(family: impl Into<String>) -> Self {
        Self::UnsupportedSurfaceFamily {
            family: family.into(),
        }
    }

    pub fn dirty_input(reason: impl Into<String>) -> Self {
        Self::DirtyInput {
            reason: reason.into(),
        }
    }

    pub fn predicate_uncertain(reason: impl Into<String>) -> Self {
        Self::PredicateUncertain {
            reason: reason.into(),
        }
    }

    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingReceiptBackedStage(stage) => {
                format!(
                    "open radial fan certification requires receipt-backed {}",
                    stage.human_name()
                )
            }
            Self::WrongTopologyPattern { pattern_name } => format!(
                "open radial fan certification requires an open radial fan topology receipt, but received {pattern_name}"
            ),
            Self::WrongTopologyPosture { posture } => format!(
                "open radial fan certification requires open non-manifold topology posture, but received {posture}"
            ),
            Self::MismatchedTopologyConstructionReceipt => {
                "open radial fan certification requires topology ledger evidence from the same NMT construction receipt"
                    .to_string()
            }
            Self::MismatchedProjectionReceipt => {
                "open radial fan certification requires projection ledger evidence from the same projected fan workload"
                    .to_string()
            }
            Self::MismatchedTransformReceipt => {
                "open radial fan certification requires transform ledger evidence from the same movement and rotation receipt"
                    .to_string()
            }
            Self::MismatchedRetainedReplayReceipt => {
                "open radial fan certification requires retained replay ledger evidence from the same replay receipt"
                    .to_string()
            }
            Self::MissingOpenBoundaryEvidence => {
                "open radial fan certification requires open-boundary evidence from topology construction"
                    .to_string()
            }
            Self::MissingRadialAdjacencyEvidence => {
                "open radial fan certification requires radial adjacency evidence proving a non-manifold edge"
                    .to_string()
            }
            Self::InsufficientIncidentFaces { incident_faces } => format!(
                "open radial fan certification requires at least three incident faces; received {incident_faces}"
            ),
            Self::MissingTopologyEvidence => {
                "open radial fan certification requires topology evidence for every incident fan face"
                    .to_string()
            }
            Self::MissingProjectionEvidence => {
                "open radial fan certification requires projected face evidence and a local projection basis"
                    .to_string()
            }
            Self::MissingTransformEvidence => {
                "open radial fan certification requires movement and rotation transform evidence"
                    .to_string()
            }
            Self::LabelOnlyMotion => {
                "open radial fan certification rejected movement evidence because no coordinates changed"
                    .to_string()
            }
            Self::MissingRetainedReplayEvidence => {
                "open radial fan certification requires retained artifacts and replay checkpoints"
                    .to_string()
            }
            Self::ClosedManifoldLaunderingAttempt { .. } => {
                "open non-manifold radial fan cannot be laundered as a closed manifold shell"
                    .to_string()
            }
            Self::UnsupportedSurfaceFamily { family } => format!(
                "open radial fan certification does not support {family} surface analysis today"
            ),
            Self::DirtyInput { reason } => format!(
                "open radial fan certification cannot continue because the input is dirty: {reason}"
            ),
            Self::PredicateUncertain { reason } => format!(
                "open radial fan certification requires resolved predicate authority before classification: {reason}"
            ),
        }
    }
}
