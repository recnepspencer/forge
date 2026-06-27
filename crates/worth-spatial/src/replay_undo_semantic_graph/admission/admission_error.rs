use crate::replay_family_catalog::SpatialReplayFamilyIdentity;
use crate::undo_family_catalog::SpatialUndoFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialReplaySemanticGraphAdmissionError {
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
        family_identity: SpatialUndoFamilyIdentity,
    },
    MissingRequiredLookupConsumedWorkload {
        family_identity: SpatialUndoFamilyIdentity,
    },
    UnexpectedLookupConsumedWorkload {
        family_identity: SpatialUndoFamilyIdentity,
    },
}
