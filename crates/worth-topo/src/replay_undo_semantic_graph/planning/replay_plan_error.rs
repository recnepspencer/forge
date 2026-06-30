use crate::replay_family_catalog::{
    TopologyReplayFamilyIdentity, TopologyReplayFamilyScopeProductPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyReplayPlanError {
    UnsupportedScopeProductPosture {
        family_identity: TopologyReplayFamilyIdentity,
        scope_product_posture: TopologyReplayFamilyScopeProductPosture,
    },
}
