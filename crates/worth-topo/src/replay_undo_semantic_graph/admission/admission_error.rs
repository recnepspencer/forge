use crate::replay_family_catalog::{
    TopologyReplayFamilyIdentity, TopologyReplayFamilyLocalityPosture,
    TopologyReplayFamilyPriorProofPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyReplaySemanticGraphAdmissionError {
    MissingReplayFamilyDeclaration {
        family_identity: TopologyReplayFamilyIdentity,
    },
    MissingRequiredStageReceiptAuthority {
        family_identity: TopologyReplayFamilyIdentity,
    },
    MissingRequiredStageIdentity {
        family_identity: TopologyReplayFamilyIdentity,
    },
    UnsupportedLocalityPosture {
        family_identity: TopologyReplayFamilyIdentity,
        locality_posture: TopologyReplayFamilyLocalityPosture,
    },
    UnsupportedPriorProofPosture {
        family_identity: TopologyReplayFamilyIdentity,
        prior_proof_posture: TopologyReplayFamilyPriorProofPosture,
    },
    InvalidationReceiptTouchedClosureMismatch {
        touched_closure_digest: String,
        receipt_touched_closure_digest: String,
    },
    StageReceiptFamilyMismatch {
        family_identity: TopologyReplayFamilyIdentity,
        stage_receipt_family_identity: TopologyReplayFamilyIdentity,
    },
    StageReceiptSelectedPlanMismatch {
        invalidation_selected_plan_digest: String,
        stage_receipt_selected_plan_digest: String,
    },
    StageReceiptTouchedClosureMismatch {
        touched_closure_digest: String,
        stage_receipt_touched_closure_digest: String,
    },
    StageIdentityMismatch {
        declared_stage_identity_digest: String,
        stage_receipt_stage_identity_digest: String,
    },
}
