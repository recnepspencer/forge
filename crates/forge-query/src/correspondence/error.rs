use super::cost::CorrespondenceCostPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceEvaluationFailureClass {
    InvalidRequest,
    UnsupportedTopology,
    UnsupportedStructuralFamily,
    UnsupportedMixedEvidence,
    BroadStructuralScanRequired,
    StructuralBreadthExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceEvaluationError {
    MissingEvidence,
    BroadStructuralScanRequired,
    StructuralBreadthExceeded { budget: usize, actual: usize },
    UnsupportedStructuralFamily { family: &'static str },
    UnsupportedMixedEvidence { reason: &'static str },
    UnsupportedTopology { topology: &'static str },
}

impl CorrespondenceEvaluationError {
    pub fn failure_class(&self) -> CorrespondenceEvaluationFailureClass {
        match self {
            Self::MissingEvidence => CorrespondenceEvaluationFailureClass::InvalidRequest,
            Self::BroadStructuralScanRequired => {
                CorrespondenceEvaluationFailureClass::BroadStructuralScanRequired
            }
            Self::StructuralBreadthExceeded { .. } => {
                CorrespondenceEvaluationFailureClass::StructuralBreadthExceeded
            }
            Self::UnsupportedStructuralFamily { .. } => {
                CorrespondenceEvaluationFailureClass::UnsupportedStructuralFamily
            }
            Self::UnsupportedMixedEvidence { .. } => {
                CorrespondenceEvaluationFailureClass::UnsupportedMixedEvidence
            }
            Self::UnsupportedTopology { .. } => {
                CorrespondenceEvaluationFailureClass::UnsupportedTopology
            }
        }
    }

    pub(crate) fn denial_posture(&self) -> CorrespondenceCostPosture {
        match self {
            Self::BroadStructuralScanRequired | Self::StructuralBreadthExceeded { .. } => {
                CorrespondenceCostPosture::CorrespondenceDeniedByBreadth
            }
            Self::UnsupportedTopology { .. } => {
                CorrespondenceCostPosture::CorrespondenceDeniedByTopology
            }
            Self::UnsupportedStructuralFamily { .. } | Self::UnsupportedMixedEvidence { .. } => {
                CorrespondenceCostPosture::CorrespondenceDeniedByUnsupportedEvidence
            }
            Self::MissingEvidence => {
                CorrespondenceCostPosture::CorrespondenceDeniedByUnsupportedEvidence
            }
        }
    }

    pub(crate) fn reason(&self) -> &'static str {
        match self {
            Self::MissingEvidence => {
                "correspondence evaluation request requires lineage or structural evidence"
            }
            Self::BroadStructuralScanRequired => {
                "structural evidence lowering would require a broad scan"
            }
            Self::StructuralBreadthExceeded { .. } => {
                "structural candidate discovery exceeded the planned budget"
            }
            Self::UnsupportedStructuralFamily { .. } => {
                "structural evidence family is not admitted by phase 2 correspondence lowering"
            }
            Self::UnsupportedMixedEvidence { reason } => reason,
            Self::UnsupportedTopology { .. } => {
                "lineage topology cannot be lowered as authoritative continuity"
            }
        }
    }
}
