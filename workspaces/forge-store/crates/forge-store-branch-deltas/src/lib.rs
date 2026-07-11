#![forbid(unsafe_code)]

mod identity;
mod layout_access;
mod maintenance;
mod read;
mod semantic_authority;

pub use forge_store_layout_indexes::layout_strategy_admission::{
    AdmittedBranchDeltaLayoutRule, AdmittedContinuationLayoutRule, AdmittedStableBasisLayoutRule,
};
pub use identity::BranchDeltaLayerId;
pub use layout_access::{
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
    layout_access::reject_branch_delta_read_plan(plan)
}

pub fn admit_stable_basis_layout_support(
    plan: &forge_store_live_query::StableBasisReadPlan,
) -> Result<StableBasisLayoutReport, BranchDeltaLayoutAccessDenial> {
    layout_access::admit_stable_basis_layout_support(plan)
}

pub fn reject_stable_basis_layout_descriptor(
    stable_basis_id: forge_store_live_query::StableBasisId,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    layout_access::reject_stable_basis_layout_descriptor(stable_basis_id)
}

pub fn admit_continuation_layout_support(
    plan: &forge_store_live_query::CursorContinuationPlan,
) -> Result<ContinuationLayoutReport, BranchDeltaLayoutAccessDenial> {
    layout_access::admit_continuation_layout_support(plan)
}

pub fn reject_broadened_continuation_receipt(
    receipt: &forge_store_live_query::BroadenedBatchReceipt,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    layout_access::reject_broadened_continuation_receipt(receipt)
}
