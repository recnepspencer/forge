#![forbid(unsafe_code)]

mod layout_access;

pub use layout_access::{
    BranchDeltaLayoutAccessDenial, BranchDeltaLayoutAccessDenialKind, BranchDeltaLayoutReport,
    BranchDeltaLayoutSupportEstimate, ContinuationLayoutReport, ContinuationLayoutSupportEstimate,
    StableBasisLayoutReport, StableBasisLayoutSupportEstimate,
};
pub use forge_store_layout_indexes::layout_strategy_admission::{
    AdmittedBranchDeltaLayoutRule, AdmittedContinuationLayoutRule,
    AdmittedStableBasisLayoutRule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchSemanticAuthority;

pub const fn branch_semantic_authority() -> BranchSemanticAuthority {
    BranchSemanticAuthority
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchDeltaLayerId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SameBranchDescendantWitness {
    branch_lineage: String,
}

impl SameBranchDescendantWitness {
    pub(crate) fn new(branch_lineage: impl Into<String>) -> Self {
        Self {
            branch_lineage: branch_lineage.into(),
        }
    }

    pub fn branch_lineage(&self) -> &str {
        &self.branch_lineage
    }

    pub fn admit_branch_delta_layout(
        &self,
        plan: &BranchDeltaReadPlan,
    ) -> Result<BranchDeltaLayoutReport, BranchDeltaLayoutAccessDenial> {
        layout_access::admit_branch_delta_layout(plan, self)
    }
}

impl BranchSemanticAuthority {
    pub fn admit_same_branch_descendant(
        self,
        branch_lineage: impl Into<String>,
    ) -> SameBranchDescendantWitness {
        SameBranchDescendantWitness::new(branch_lineage)
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaReadRequest {
    layer_id: BranchDeltaLayerId,
    branch_lineage: String,
}

impl BranchDeltaReadRequest {
    pub fn new(layer_id: BranchDeltaLayerId, branch_lineage: impl Into<String>) -> Self {
        Self {
            layer_id,
            branch_lineage: branch_lineage.into(),
        }
    }

    pub const fn layer_id(&self) -> BranchDeltaLayerId {
        self.layer_id
    }

    pub fn branch_lineage(&self) -> &str {
        &self.branch_lineage
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaReadPlan {
    request: BranchDeltaReadRequest,
    declared_delta_rows: u32,
}

impl BranchDeltaReadPlan {
    pub const fn new(request: BranchDeltaReadRequest, declared_delta_rows: u32) -> Self {
        Self {
            request,
            declared_delta_rows,
        }
    }

    pub const fn request(&self) -> &BranchDeltaReadRequest {
        &self.request
    }

    pub const fn declared_delta_rows(&self) -> u32 {
        self.declared_delta_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaReadResult {
    layer_id: BranchDeltaLayerId,
    returned_delta_rows: u32,
}

impl BranchDeltaReadResult {
    pub const fn new(layer_id: BranchDeltaLayerId, returned_delta_rows: u32) -> Self {
        Self {
            layer_id,
            returned_delta_rows,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaRewritePlan {
    layer_id: BranchDeltaLayerId,
    rewritten_delta_rows: u32,
}

impl BranchDeltaRewritePlan {
    pub const fn new(layer_id: BranchDeltaLayerId, rewritten_delta_rows: u32) -> Self {
        Self {
            layer_id,
            rewritten_delta_rows,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaRebuildReceipt {
    layer_id: BranchDeltaLayerId,
    rebuilt_delta_rows: u32,
}

impl BranchDeltaRebuildReceipt {
    pub const fn new(layer_id: BranchDeltaLayerId, rebuilt_delta_rows: u32) -> Self {
        Self {
            layer_id,
            rebuilt_delta_rows,
        }
    }
}
