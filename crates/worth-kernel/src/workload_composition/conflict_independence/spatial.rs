use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;

use super::common::{canonical_pair_digest, canonical_pair_parts, ConflictIndependenceDisposition};
use super::spatial_lowering::classify;
use crate::workload_composition::SelectedSpatialConflictPlan;

#[derive(Clone, Copy)]
pub struct SpatialConflictIndependenceRequest<'a> {
    left: &'a SelectedSpatialConflictPlan<'a>,
    right: &'a SelectedSpatialConflictPlan<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConflictIndependenceDenialKind {
    SelectedPlanDenied,
    DeclaredDenied,
    MissingPositiveProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConflictIndependenceDenial {
    pub(super) kind: SpatialConflictIndependenceDenialKind,
    pub(super) detail: String,
}

#[derive(Clone)]
pub struct SpatialConflictIndependenceProof<'a> {
    left: &'a SelectedSpatialConflictPlan<'a>,
    right: &'a SelectedSpatialConflictPlan<'a>,
    disposition: ConflictIndependenceDisposition,
    denial: Option<SpatialConflictIndependenceDenial>,
    proof_digest: String,
}

impl<'a> SpatialConflictIndependenceRequest<'a> {
    pub fn new(
        left: &'a SelectedSpatialConflictPlan<'a>,
        right: &'a SelectedSpatialConflictPlan<'a>,
    ) -> Self {
        Self { left, right }
    }

    pub const fn left(&self) -> &'a SelectedSpatialConflictPlan<'a> {
        self.left
    }

    pub const fn right(&self) -> &'a SelectedSpatialConflictPlan<'a> {
        self.right
    }
}

pub fn prove_spatial_conflict_independence<'a>(
    request: SpatialConflictIndependenceRequest<'a>,
) -> SpatialConflictIndependenceProof<'a> {
    let context = classify(&request);
    let overlap_parts = canonical_pair_parts(
        "overlap",
        format!("{:?}", request.left.overlap_category()),
        format!("{:?}", request.right.overlap_category()),
    );
    let locality_parts = canonical_pair_parts(
        "locality",
        request.left.authority().digest().as_str().to_string(),
        request.right.authority().digest().as_str().to_string(),
    );
    let proof_digest = canonical_pair_digest(
        "worth-kernel:spatial-conflict-independence-proof:v1",
        request.left.selected_plan_digest(),
        request.right.selected_plan_digest(),
        &[
            format!("disposition:{}", context.disposition.as_str()),
            overlap_parts[0].clone(),
            overlap_parts[1].clone(),
            locality_parts[0].clone(),
            locality_parts[1].clone(),
            format!(
                "denial:{:?}",
                context
                    .denial
                    .as_ref()
                    .map(SpatialConflictIndependenceDenial::kind)
            ),
        ],
    );
    SpatialConflictIndependenceProof {
        left: request.left,
        right: request.right,
        disposition: context.disposition,
        denial: context.denial,
        proof_digest,
    }
}

impl SpatialConflictIndependenceDenial {
    pub const fn kind(&self) -> SpatialConflictIndependenceDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl<'a> SpatialConflictIndependenceProof<'a> {
    pub const fn left(&self) -> &'a SelectedSpatialConflictPlan<'a> {
        self.left
    }

    pub const fn right(&self) -> &'a SelectedSpatialConflictPlan<'a> {
        self.right
    }

    pub const fn disposition(&self) -> ConflictIndependenceDisposition {
        self.disposition
    }

    pub fn denial(&self) -> Option<&SpatialConflictIndependenceDenial> {
        self.denial.as_ref()
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub const fn left_overlap_category(&self) -> ConflictOverlapCategory {
        self.left.overlap_category()
    }

    pub const fn right_overlap_category(&self) -> ConflictOverlapCategory {
        self.right.overlap_category()
    }
}
