use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;

use super::common::{canonical_pair_digest, canonical_pair_parts, ConflictIndependenceDisposition};
use super::topology_lowering::classify;
use crate::workload_composition::SelectedTopologyConflictPlan;

#[derive(Clone, Copy)]
pub struct TopologyConflictIndependenceRequest<'a> {
    left: &'a SelectedTopologyConflictPlan<'a>,
    right: &'a SelectedTopologyConflictPlan<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConflictIndependenceDenialKind {
    SelectedPlanDenied,
    DeclaredDenied,
    MissingPositiveProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConflictIndependenceDenial {
    pub(super) kind: TopologyConflictIndependenceDenialKind,
    pub(super) detail: String,
}

#[derive(Clone)]
pub struct TopologyConflictIndependenceProof<'a> {
    left: &'a SelectedTopologyConflictPlan<'a>,
    right: &'a SelectedTopologyConflictPlan<'a>,
    disposition: ConflictIndependenceDisposition,
    denial: Option<TopologyConflictIndependenceDenial>,
    proof_digest: String,
}

impl<'a> TopologyConflictIndependenceRequest<'a> {
    pub fn new(
        left: &'a SelectedTopologyConflictPlan<'a>,
        right: &'a SelectedTopologyConflictPlan<'a>,
    ) -> Self {
        Self { left, right }
    }

    pub const fn left(&self) -> &'a SelectedTopologyConflictPlan<'a> {
        self.left
    }

    pub const fn right(&self) -> &'a SelectedTopologyConflictPlan<'a> {
        self.right
    }
}

pub fn prove_topology_conflict_independence<'a>(
    request: TopologyConflictIndependenceRequest<'a>,
) -> TopologyConflictIndependenceProof<'a> {
    let context = classify(&request);
    let overlap_parts = canonical_pair_parts(
        "overlap",
        format!("{:?}", request.left.overlap_category()),
        format!("{:?}", request.right.overlap_category()),
    );
    let locality_parts = canonical_pair_parts(
        "locality",
        request.left.touched_closure().closure_digest().to_string(),
        request.right.touched_closure().closure_digest().to_string(),
    );
    let proof_digest = canonical_pair_digest(
        "worth-kernel:topology-conflict-independence-proof:v1",
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
                    .map(TopologyConflictIndependenceDenial::kind)
            ),
        ],
    );
    TopologyConflictIndependenceProof {
        left: request.left,
        right: request.right,
        disposition: context.disposition,
        denial: context.denial,
        proof_digest,
    }
}

impl TopologyConflictIndependenceDenial {
    pub const fn kind(&self) -> TopologyConflictIndependenceDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl<'a> TopologyConflictIndependenceProof<'a> {
    pub const fn left(&self) -> &'a SelectedTopologyConflictPlan<'a> {
        self.left
    }

    pub const fn right(&self) -> &'a SelectedTopologyConflictPlan<'a> {
        self.right
    }

    pub const fn disposition(&self) -> ConflictIndependenceDisposition {
        self.disposition
    }

    pub fn denial(&self) -> Option<&TopologyConflictIndependenceDenial> {
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
