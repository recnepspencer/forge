use std::collections::BTreeSet;

use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::selected_plan::{
    BatchAdmissionSupportingConflictFamilyRow, BatchAdmissionSupportingConflictLane,
};
use crate::workload_composition::{
    ConflictIndependenceDisposition, ConflictPlanDownstreamProofCategory,
    ConflictPlanExecutionAdmission, SelectedSpatialConflictPlan, SelectedTopologyConflictPlan,
    SpatialConflictIndependenceProof, TopologyConflictIndependenceProof,
};

#[derive(Clone, Copy)]
pub enum BatchAdmissionCandidate<'a> {
    Topology(&'a SelectedTopologyConflictPlan<'a>),
    Spatial(&'a SelectedSpatialConflictPlan<'a>),
}

#[derive(Clone, Copy)]
pub enum BatchAdmissionPairwiseIndependenceProof<'a> {
    Topology(&'a TopologyConflictIndependenceProof<'a>),
    Spatial(&'a SpatialConflictIndependenceProof<'a>),
}

#[derive(Clone)]
pub struct BatchAdmissionGroupedInput<'a> {
    candidates: Vec<BatchAdmissionCandidate<'a>>,
    pairwise_independence: Vec<BatchAdmissionPairwiseIndependenceProof<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchAdmissionGroupedInputAdmissionErrorKind {
    RequiresAtLeastTwoCandidates,
    DuplicateSelectedPlanIdentity,
    ProofEndpointNotInGroup,
    ProofDoesNotBindDistinctParticipants,
    DuplicatePairwiseProofCoverage,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionGroupedInputAdmissionError {
    kind: BatchAdmissionGroupedInputAdmissionErrorKind,
    detail: String,
}

#[derive(Clone)]
pub struct AdmittedBatchAdmissionGroupedInput<'a> {
    candidates: Vec<BatchAdmissionCandidate<'a>>,
    pairwise_independence: Vec<BatchAdmissionPairwiseIndependenceProof<'a>>,
    grouped_input_digest: String,
}

impl<'a> BatchAdmissionCandidate<'a> {
    pub fn authority_digest(&self) -> &str {
        match self {
            Self::Topology(plan) => plan.touched_closure().closure_digest(),
            Self::Spatial(plan) => plan.authority().digest().as_str(),
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        match self {
            Self::Topology(plan) => plan.selected_plan_digest(),
            Self::Spatial(plan) => plan.selected_plan_digest(),
        }
    }

    pub fn overlap_identity_digest(&self) -> &str {
        match self {
            Self::Topology(plan) => plan.overlap_identity_digest(),
            Self::Spatial(plan) => plan.overlap_identity_digest(),
        }
    }

    pub fn locality_footprint_digest(&self) -> &str {
        match self {
            Self::Topology(plan) => plan.locality_footprint_digest(),
            Self::Spatial(plan) => plan.locality_footprint_digest(),
        }
    }

    pub const fn overlap_category(&self) -> ConflictOverlapCategory {
        match self {
            Self::Topology(plan) => plan.overlap_category(),
            Self::Spatial(plan) => plan.overlap_category(),
        }
    }

    pub const fn downstream_proof_category(&self) -> ConflictPlanDownstreamProofCategory {
        match self {
            Self::Topology(plan) => plan.downstream_proof_category(),
            Self::Spatial(plan) => plan.downstream_proof_category(),
        }
    }

    pub const fn execution_admission(&self) -> ConflictPlanExecutionAdmission {
        match self {
            Self::Topology(plan) => plan.execution_admission(),
            Self::Spatial(plan) => plan.execution_admission(),
        }
    }

    pub fn supporting_conflict_family_rows(
        &self,
    ) -> Vec<BatchAdmissionSupportingConflictFamilyRow> {
        match self {
            Self::Topology(plan) => plan
                .selected_families()
                .iter()
                .map(|row| {
                    BatchAdmissionSupportingConflictFamilyRow::new(
                        plan.selected_plan_digest(),
                        BatchAdmissionSupportingConflictLane::Topology,
                        row.identity().as_str(),
                        row.declaration_digest(),
                    )
                })
                .collect(),
            Self::Spatial(plan) => plan
                .selected_families()
                .iter()
                .map(|row| {
                    BatchAdmissionSupportingConflictFamilyRow::new(
                        plan.selected_plan_digest(),
                        BatchAdmissionSupportingConflictLane::Spatial,
                        row.identity().as_str(),
                        row.declaration_digest(),
                    )
                })
                .collect(),
        }
    }
}

impl<'a> BatchAdmissionPairwiseIndependenceProof<'a> {
    pub fn left_plan_digest(&self) -> &str {
        match self {
            Self::Topology(proof) => proof.left().selected_plan_digest(),
            Self::Spatial(proof) => proof.left().selected_plan_digest(),
        }
    }

    pub fn right_plan_digest(&self) -> &str {
        match self {
            Self::Topology(proof) => proof.right().selected_plan_digest(),
            Self::Spatial(proof) => proof.right().selected_plan_digest(),
        }
    }

    pub const fn disposition(&self) -> ConflictIndependenceDisposition {
        match self {
            Self::Topology(proof) => proof.disposition(),
            Self::Spatial(proof) => proof.disposition(),
        }
    }

    pub fn proof_digest(&self) -> &str {
        match self {
            Self::Topology(proof) => proof.proof_digest(),
            Self::Spatial(proof) => proof.proof_digest(),
        }
    }
}

impl<'a> BatchAdmissionGroupedInput<'a> {
    pub fn new<const N: usize>(candidates: [BatchAdmissionCandidate<'a>; N]) -> Self {
        Self {
            candidates: candidates.into_iter().collect(),
            pairwise_independence: Vec::new(),
        }
    }

    pub fn with_pairwise_independence(
        mut self,
        proof: BatchAdmissionPairwiseIndependenceProof<'a>,
    ) -> Self {
        self.pairwise_independence.push(proof);
        self
    }
}

impl BatchAdmissionGroupedInputAdmissionError {
    pub const fn kind(&self) -> BatchAdmissionGroupedInputAdmissionErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl<'a> AdmittedBatchAdmissionGroupedInput<'a> {
    pub fn candidates(&self) -> &[BatchAdmissionCandidate<'a>] {
        &self.candidates
    }

    pub fn pairwise_independence(&self) -> &[BatchAdmissionPairwiseIndependenceProof<'a>] {
        &self.pairwise_independence
    }

    pub fn grouped_input_digest(&self) -> &str {
        &self.grouped_input_digest
    }
}

pub fn admit_batch_admission_grouped_input<'a>(
    input: BatchAdmissionGroupedInput<'a>,
) -> Result<AdmittedBatchAdmissionGroupedInput<'a>, BatchAdmissionGroupedInputAdmissionError> {
    if input.candidates.len() < 2 {
        return Err(BatchAdmissionGroupedInputAdmissionError {
            kind: BatchAdmissionGroupedInputAdmissionErrorKind::RequiresAtLeastTwoCandidates,
            detail: "batch admission grouped input requires at least two selected conflict plans"
                .to_string(),
        });
    }
    let mut candidate_digests = BTreeSet::new();
    for candidate in &input.candidates {
        if !candidate_digests.insert(candidate.selected_plan_digest().to_string()) {
            return Err(BatchAdmissionGroupedInputAdmissionError {
                kind: BatchAdmissionGroupedInputAdmissionErrorKind::DuplicateSelectedPlanIdentity,
                detail: "batch admission grouped input requires each grouped selected conflict plan to carry a unique selected plan digest"
                    .to_string(),
            });
        }
    }
    let mut covered_pairs = BTreeSet::new();
    for proof in &input.pairwise_independence {
        if proof.left_plan_digest() == proof.right_plan_digest() {
            return Err(BatchAdmissionGroupedInputAdmissionError {
                kind: BatchAdmissionGroupedInputAdmissionErrorKind::ProofDoesNotBindDistinctParticipants,
                detail: "batch admission pairwise independence proof must bind two distinct grouped selected conflict plans"
                    .to_string(),
            });
        }
        if !candidate_digests.contains(proof.left_plan_digest())
            || !candidate_digests.contains(proof.right_plan_digest())
        {
            return Err(BatchAdmissionGroupedInputAdmissionError {
                kind: BatchAdmissionGroupedInputAdmissionErrorKind::ProofEndpointNotInGroup,
                detail: "batch admission pairwise independence proof must bind only grouped selected conflict plans"
                    .to_string(),
            });
        }
        let pair_key = canonical_pair_key(proof.left_plan_digest(), proof.right_plan_digest());
        if !covered_pairs.insert(pair_key) {
            return Err(BatchAdmissionGroupedInputAdmissionError {
                kind: BatchAdmissionGroupedInputAdmissionErrorKind::DuplicatePairwiseProofCoverage,
                detail: "batch admission grouped input requires at most one explicit independence proof for each unordered participant pair"
                    .to_string(),
            });
        }
    }
    let mut candidates = input.candidates;
    candidates.sort_by_key(|candidate| candidate.selected_plan_digest().to_string());
    let mut pairwise_independence = input.pairwise_independence;
    pairwise_independence.sort_by_key(|proof| proof.proof_digest().to_string());
    let grouped_input_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &candidates
            .iter()
            .map(|candidate| format!("candidate:{}", candidate.selected_plan_digest()))
            .chain(
                pairwise_independence
                    .iter()
                    .map(|proof| format!("proof:{}", proof.proof_digest())),
            )
            .chain(std::iter::once(
                "worth-kernel:batch-admission-grouped-input:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );
    Ok(AdmittedBatchAdmissionGroupedInput {
        candidates,
        pairwise_independence,
        grouped_input_digest,
    })
}

fn canonical_pair_key(left: &str, right: &str) -> String {
    if left <= right {
        format!("{left}|{right}")
    } else {
        format!("{right}|{left}")
    }
}
