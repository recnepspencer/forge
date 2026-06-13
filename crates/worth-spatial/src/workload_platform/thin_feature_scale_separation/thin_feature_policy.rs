use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThinFeatureTinyRotationPressure {
    RequiredAndSupported,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThinFeaturePredicateCertification {
    Certified,
    Uncertain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThinFeatureScalePolicy {
    Admit,
    RequiresUserDecision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThinFeatureEvidenceIntegrity {
    Consistent,
    MismatchedLocalFrameProjection { stage: WorkloadEvidenceStage },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ThinFeatureScaleSeparationWorkloadError {
    MissingReceiptBackedStage(WorkloadEvidenceStage),
    MissingTopologyEvidence,
    MissingSurfaceSupportEvidence,
    MissingPrecisionEvidence,
    MissingLocalFrameEvidence,
    MissingPlatformProjectionEvidence,
    MissingProjectionEvidence,
    MissingProjectionConsumedBasis,
    MissingTransformEvidence,
    MissingResponseEvidence,
    PrecisionBasisFailure,
    PredicateUncertain,
    PolicyRequired,
    UnsupportedTinyRotationPosture,
    IntegrityMismatch { stage: WorkloadEvidenceStage },
}

impl ThinFeatureScaleSeparationWorkloadError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingReceiptBackedStage(stage) => {
                format!(
                    "thin-feature scale separation requires receipt-backed {}",
                    stage.human_name()
                )
            }
            Self::MissingTopologyEvidence => {
                "thin-feature scale separation requires topology-bound feature evidence for at least twelve local features".to_string()
            }
            Self::MissingSurfaceSupportEvidence => {
                "thin-feature scale separation requires certified planar surface support".to_string()
            }
            Self::MissingPrecisionEvidence => {
                "thin-feature scale separation requires a precision receipt with local feature scale and world magnitude".to_string()
            }
            Self::MissingLocalFrameEvidence => {
                "thin-feature scale separation requires a local-frame receipt tied to the precision receipt".to_string()
            }
            Self::MissingPlatformProjectionEvidence => {
                "thin-feature scale separation requires the workload catalog projection receipt that produced the platform evidence".to_string()
            }
            Self::MissingProjectionEvidence => {
                "thin-feature scale separation requires projected topology evidence with a local basis".to_string()
            }
            Self::MissingProjectionConsumedBasis => {
                "thin-feature scale separation requires projection-consumed facts preserving the certified local basis".to_string()
            }
            Self::MissingTransformEvidence => {
                "thin-feature scale separation requires movement and tiny-rotation transform evidence".to_string()
            }
            Self::MissingResponseEvidence => {
                "thin-feature scale separation requires diagnostic and user response evidence".to_string()
            }
            Self::PrecisionBasisFailure => {
                "thin-feature scale separation requires local feature scale, world magnitude, and precision basis to agree".to_string()
            }
            Self::PredicateUncertain => {
                "predicate authority could not certify the thin feature; inspect exact predicate evidence before boolean execution".to_string()
            }
            Self::PolicyRequired => {
                "thin-feature scale separation needs a user policy decision before boolean execution".to_string()
            }
            Self::UnsupportedTinyRotationPosture => {
                "thin-feature tiny-rotation posture is unsupported for this workload profile".to_string()
            }
            Self::IntegrityMismatch { stage } => {
                format!(
                    "thin-feature scale separation evidence must preserve the same local frame through {}",
                    stage.human_name()
                )
            }
        }
    }
}
