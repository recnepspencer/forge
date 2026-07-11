use crate::{BranchDeltaLayoutAccessDenial, BranchDeltaLayoutReport, BranchDeltaReadPlan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchSemanticAuthority;

pub const fn branch_semantic_authority() -> BranchSemanticAuthority {
    BranchSemanticAuthority
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SameBranchDescendantWitness {
    branch_lineage: String,
}

impl SameBranchDescendantWitness {
    fn new(branch_lineage: impl Into<String>) -> Self {
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
        crate::layout_access::admit_branch_delta_layout(plan, self)
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
