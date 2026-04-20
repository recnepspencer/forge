use forge_relational::facade::history::CommitId;

use crate::backend::records::{BranchHeadRecord, BranchRecord};

#[derive(Debug)]
pub(crate) struct AppliedAuthoritativeAppend {
    pub(crate) branch_identity: String,
    pub(crate) commit_id: CommitId,
    pub(crate) parent_count: usize,
    pub(crate) created_branch: bool,
    pub(crate) previous_next_commit_sequence: u64,
    pub(crate) previous_next_head_update_sequence: u64,
    pub(crate) previous_branch_record: Option<BranchRecord>,
    pub(crate) previous_branch_head_record: Option<BranchHeadRecord>,
    pub(crate) inserted_support_summary: bool,
    pub(crate) inserted_schema_support: bool,
    pub(crate) inserted_lineage_support: bool,
    pub(crate) inserted_branch_delta_layer_id: Option<u64>,
}
