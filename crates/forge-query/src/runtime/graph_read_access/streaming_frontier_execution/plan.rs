use super::ForgeQueryGraphReadStreamingPageBudget;
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadStreamingPlan {
    digest: String,
    admission_digest: String,
    requirement_set_digest: String,
    page_budget: ForgeQueryGraphReadStreamingPageBudget,
    planned_frontier_page_count_floor: usize,
    canonical_result_basis_digest: String,
    replay_basis_digest: String,
}

impl ForgeQueryGraphReadStreamingPlan {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn page_budget(&self) -> &ForgeQueryGraphReadStreamingPageBudget {
        &self.page_budget
    }

    pub fn planned_frontier_page_count_floor(&self) -> usize {
        self.planned_frontier_page_count_floor
    }

    pub fn canonical_result_basis_digest(&self) -> &str {
        &self.canonical_result_basis_digest
    }

    pub fn replay_basis_digest(&self) -> &str {
        &self.replay_basis_digest
    }

    pub(crate) fn from_admission(admission: &ForgeQueryGraphReadAccessAdmission) -> Option<Self> {
        if admission.posture() != &ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
        {
            return None;
        }
        let page_budget = ForgeQueryGraphReadStreamingPageBudget::frontier_default();
        let planned_frontier_page_count_floor = admission
            .cost_estimate()
            .intrinsic()
            .candidate_roots()
            .max(1);
        let admission_digest = admission.digest().to_string();
        let requirement_set_digest = admission.requirement_set().digest().as_str().to_string();
        let canonical_result_basis_digest = hash_parts(&[
            "forge_query_graph_read_streaming_canonical_result_basis_v1".to_string(),
            format!("requirements:{requirement_set_digest}"),
            format!(
                "selectivity:{}",
                admission.requirement_set().selectivity_shape_digest()
            ),
        ]);
        let replay_basis_digest = hash_parts(&[
            "forge_query_graph_read_streaming_replay_basis_v1".to_string(),
            format!("admission:{admission_digest}"),
            format!("requirements:{requirement_set_digest}"),
            format!("page_budget:{}", page_budget.digest()),
        ]);
        let digest = hash_parts(&[
            "forge_query_graph_read_streaming_plan_v1".to_string(),
            format!("admission:{admission_digest}"),
            format!("requirements:{requirement_set_digest}"),
            format!("page_budget:{}", page_budget.digest()),
            format!("planned_frontier_page_count_floor:{planned_frontier_page_count_floor}"),
            format!("canonical_result_basis:{canonical_result_basis_digest}"),
            format!("replay_basis:{replay_basis_digest}"),
        ]);
        Some(Self {
            digest,
            admission_digest,
            requirement_set_digest,
            page_budget,
            planned_frontier_page_count_floor,
            canonical_result_basis_digest,
            replay_basis_digest,
        })
    }
}

pub(crate) fn streaming_frontier_is_admissible(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
) -> bool {
    requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency)
        && requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset)
        && requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::VisitedSet)
        && requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::DedupSet)
        && requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::ProofSupport)
        && !requirements
            .requires_kind(ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport)
}
