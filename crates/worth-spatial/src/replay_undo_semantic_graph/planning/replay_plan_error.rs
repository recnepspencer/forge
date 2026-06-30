use crate::replay_family_catalog::{
    SpatialReplayFamilyIdentity, SpatialReplayFamilyLocalityPosture,
    SpatialReplayFamilyPriorProofPosture, SpatialReplayFamilyScopeProductPosture,
    SpatialReplayFamilyStageIndexPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialReplayPlanError {
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
}
