#![forbid(unsafe_code)]

mod identity;
mod layout_projection;
mod maintenance;
mod read;
mod semantic_authority;

pub use identity::BranchDeltaLayerId;
pub use layout_projection::{
    BranchDeltaLayoutAccessDenial, BranchDeltaLayoutAccessDenialKind, BranchDeltaLayoutReport,
    BranchDeltaLayoutSupportEstimate, ContinuationLayoutReport, ContinuationLayoutSupportEstimate,
    StableBasisLayoutReport, StableBasisLayoutSupportEstimate,
};
pub use maintenance::{BranchDeltaRebuildReceipt, BranchDeltaRewritePlan};
pub use read::{BranchDeltaReadPlan, BranchDeltaReadRequest, BranchDeltaReadResult};
pub use semantic_authority::{
    branch_semantic_authority, BranchSemanticAuthority, SameBranchDescendantWitness,
};

pub fn reject_branch_delta_read_plan(
    plan: &BranchDeltaReadPlan,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    layout_projection::reject_branch_delta_read_plan(plan)
}

pub fn admit_stable_basis_layout_support(
    plan: &worth_store_live_query::StableBasisReadPlan,
) -> Result<StableBasisLayoutReport, BranchDeltaLayoutAccessDenial> {
    layout_projection::admit_stable_basis_layout_support(plan)
}

pub fn reject_stable_basis_layout_descriptor(
    stable_basis_id: worth_store_live_query::StableBasisId,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    layout_projection::reject_stable_basis_layout_descriptor(stable_basis_id)
}

pub fn admit_continuation_layout_support(
    plan: &worth_store_live_query::CursorContinuationPlan,
) -> Result<ContinuationLayoutReport, BranchDeltaLayoutAccessDenial> {
    layout_projection::admit_continuation_layout_support(plan)
}

pub fn reject_broadened_continuation_receipt(
    receipt: &worth_store_live_query::BroadenedBatchReceipt,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    layout_projection::reject_broadened_continuation_receipt(receipt)
}
