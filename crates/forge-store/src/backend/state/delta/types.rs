use crate::backend::{
    records::BranchDeltaLayerRecord,
    state::branch_lifecycle::AppliedBranchCreation,
};
use forge_relational::facade::history::BranchId;

#[derive(Debug)]
pub(crate) struct AppliedSharedBaseBranchCreation {
    pub(super) branch_creation: AppliedBranchCreation,
    pub(super) branch_identity: String,
}

#[derive(Debug)]
pub(crate) struct AppliedBranchDeltaRewrite {
    pub(super) replacement_layer_id: Option<u64>,
    pub(super) removed_layers: Vec<BranchDeltaLayerRecord>,
    pub(super) previous_next_branch_delta_layer_id: u64,
}

#[derive(Debug)]
pub(crate) struct AppliedBranchDeltaRebuild {
    pub(super) branch_id: BranchId,
    pub(super) inserted_layer_ids: Vec<u64>,
    pub(super) removed_layers: Vec<BranchDeltaLayerRecord>,
    pub(super) previous_next_branch_delta_layer_id: u64,
}
