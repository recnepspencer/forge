#[path = "layout/materialization.rs"]
mod materialization;
#[path = "layout/milestone_6_records.rs"]
mod milestone_6_records;

pub(crate) use materialization::Milestone6LayoutMaterializationRecord;
pub(crate) use milestone_6_records::{
    Milestone6ChunkMembershipRecord, Milestone6CommitCoupledLayoutSeedRecord,
    Milestone6ScopeSliceMembershipRecord, Milestone6StructuralBlockRecord,
};
