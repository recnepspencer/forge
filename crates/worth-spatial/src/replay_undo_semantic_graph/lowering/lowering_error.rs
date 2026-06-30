use crate::replay_family_catalog::{
    SpatialReplayFamilyIdentity, SpatialReplayFamilyLocalityPosture,
    SpatialReplayFamilyPriorProofPosture, SpatialReplayFamilyScopeProductPosture,
    SpatialReplayFamilyStageIndexPosture,
};
use crate::replay_undo_semantic_graph::{
    SpatialReplayPlanError, SpatialReplaySemanticGraphAdmissionError, SpatialUndoPlanError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoSemanticGraphLoweringError {
    StageIndexIdentityMismatch {
        authority_stage_index_identity: String,
        product_stage_index_identity: String,
    },
    StageReceiptIdentityMismatch {
        authority_stage_receipt_identity: String,
        handoff_stage_receipt_identity: String,
    },
    LookupExecutionReceiptMismatch {
        receipt_execution_digest: String,
        handoff_execution_digest: String,
    },
    MissingCoveredFamily {
        family_identity: SpatialReplayFamilyIdentity,
    },
    MissingRequiredRetainedReplayReceipt {
        family_identity: SpatialReplayFamilyIdentity,
    },
    UnexpectedRetainedReplayReceipt {
        family_identity: SpatialReplayFamilyIdentity,
    },
    RetainedReplayReceiptMismatch {
        authority_retained_replay_identity: String,
        retained_replay_receipt_identity: String,
    },
    MissingUndoFamily {
        family_identity: String,
    },
    MissingRequiredLookupConsumedWorkload {
        family_identity: String,
    },
    UnexpectedLookupConsumedWorkload {
        family_identity: String,
    },
    UnsupportedLocalityPosture {
        family_identity: SpatialReplayFamilyIdentity,
        locality_posture: SpatialReplayFamilyLocalityPosture,
    },
    UnsupportedPriorProofPosture {
        family_identity: SpatialReplayFamilyIdentity,
        prior_proof_posture: SpatialReplayFamilyPriorProofPosture,
    },
    UnsupportedStageIndexPosture {
        family_identity: SpatialReplayFamilyIdentity,
        stage_index_posture: SpatialReplayFamilyStageIndexPosture,
    },
    UnsupportedScopeProductPosture {
        family_identity: SpatialReplayFamilyIdentity,
        scope_product_posture: SpatialReplayFamilyScopeProductPosture,
    },
    UnsupportedUndoScopeProductPosture {
        family_identity: String,
    },
}

impl From<SpatialReplaySemanticGraphAdmissionError> for ReplayUndoSemanticGraphLoweringError {
    fn from(value: SpatialReplaySemanticGraphAdmissionError) -> Self {
        match value {
            SpatialReplaySemanticGraphAdmissionError::StageIndexIdentityMismatch {
                authority_stage_index_identity,
                product_stage_index_identity,
            } => Self::StageIndexIdentityMismatch {
                authority_stage_index_identity,
                product_stage_index_identity,
            },
            SpatialReplaySemanticGraphAdmissionError::StageReceiptIdentityMismatch {
                authority_stage_receipt_identity,
                handoff_stage_receipt_identity,
            } => Self::StageReceiptIdentityMismatch {
                authority_stage_receipt_identity,
                handoff_stage_receipt_identity,
            },
            SpatialReplaySemanticGraphAdmissionError::LookupExecutionReceiptMismatch {
                receipt_execution_digest,
                handoff_execution_digest,
            } => Self::LookupExecutionReceiptMismatch {
                receipt_execution_digest,
                handoff_execution_digest,
            },
            SpatialReplaySemanticGraphAdmissionError::MissingCoveredFamily { family_identity } => {
                Self::MissingCoveredFamily { family_identity }
            }
            SpatialReplaySemanticGraphAdmissionError::MissingRequiredRetainedReplayReceipt {
                family_identity,
            } => Self::MissingRequiredRetainedReplayReceipt { family_identity },
            SpatialReplaySemanticGraphAdmissionError::UnexpectedRetainedReplayReceipt {
                family_identity,
            } => Self::UnexpectedRetainedReplayReceipt { family_identity },
            SpatialReplaySemanticGraphAdmissionError::RetainedReplayReceiptMismatch {
                authority_retained_replay_identity,
                retained_replay_receipt_identity,
            } => Self::RetainedReplayReceiptMismatch {
                authority_retained_replay_identity,
                retained_replay_receipt_identity,
            },
            SpatialReplaySemanticGraphAdmissionError::MissingUndoFamily { family_identity } => {
                Self::MissingUndoFamily {
                    family_identity: family_identity.as_str().to_string(),
                }
            }
            SpatialReplaySemanticGraphAdmissionError::MissingRequiredLookupConsumedWorkload {
                family_identity,
            } => Self::MissingRequiredLookupConsumedWorkload {
                family_identity: family_identity.as_str().to_string(),
            },
            SpatialReplaySemanticGraphAdmissionError::UnexpectedLookupConsumedWorkload {
                family_identity,
            } => Self::UnexpectedLookupConsumedWorkload {
                family_identity: family_identity.as_str().to_string(),
            },
        }
    }
}

impl From<SpatialReplayPlanError> for ReplayUndoSemanticGraphLoweringError {
    fn from(value: SpatialReplayPlanError) -> Self {
        match value {
            SpatialReplayPlanError::UnsupportedLocalityPosture {
                family_identity,
                locality_posture,
            } => Self::UnsupportedLocalityPosture {
                family_identity,
                locality_posture,
            },
            SpatialReplayPlanError::UnsupportedPriorProofPosture {
                family_identity,
                prior_proof_posture,
            } => Self::UnsupportedPriorProofPosture {
                family_identity,
                prior_proof_posture,
            },
            SpatialReplayPlanError::UnsupportedStageIndexPosture {
                family_identity,
                stage_index_posture,
            } => Self::UnsupportedStageIndexPosture {
                family_identity,
                stage_index_posture,
            },
            SpatialReplayPlanError::UnsupportedScopeProductPosture {
                family_identity,
                scope_product_posture,
            } => Self::UnsupportedScopeProductPosture {
                family_identity,
                scope_product_posture,
            },
        }
    }
}

impl From<SpatialUndoPlanError> for ReplayUndoSemanticGraphLoweringError {
    fn from(value: SpatialUndoPlanError) -> Self {
        match value {
            SpatialUndoPlanError::UnsupportedScopeProductPosture {
                family_identity, ..
            } => Self::UnsupportedUndoScopeProductPosture {
                family_identity: family_identity.as_str().to_string(),
            },
        }
    }
}
