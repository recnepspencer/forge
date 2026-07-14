use super::WorthQueryGraphReadStreamingPageBudget;
use crate::identity::hash_parts;
use crate::runtime::{
    WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadAccessRequirementSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadStreamingPlan {
    digest: String,
    admission_digest: String,
    requirement_set_digest: String,
    page_budget: WorthQueryGraphReadStreamingPageBudget,
    planned_frontier_page_count_floor: usize,
    canonical_result_basis_digest: String,
    replay_basis_digest: String,
}

impl WorthQueryGraphReadStreamingPlan {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn page_budget(&self) -> &WorthQueryGraphReadStreamingPageBudget {
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

    pub(crate) fn from_admission(admission: &WorthQueryGraphReadAccessAdmission) -> Option<Self> {
        if admission.posture() != &WorthQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
        {
            return None;
        }
        let page_budget = WorthQueryGraphReadStreamingPageBudget::frontier_default();
        let planned_frontier_page_count_floor = admission
            .cost_estimate()
            .intrinsic()
            .candidate_roots()
            .max(1);
        let admission_digest = admission.digest().to_string();
        let requirement_set_digest = admission.requirement_set().digest().as_str().to_string();
        let canonical_result_basis_digest = hash_parts(&[
            "worth_query_graph_read_streaming_canonical_result_basis_v1".to_string(),
            format!("requirements:{requirement_set_digest}"),
            format!(
                "selectivity:{}",
                admission.requirement_set().selectivity_shape_digest()
            ),
        ]);
        let replay_basis_digest = hash_parts(&[
            "worth_query_graph_read_streaming_replay_basis_v1".to_string(),
            format!("admission:{admission_digest}"),
            format!("requirements:{requirement_set_digest}"),
            format!("page_budget:{}", page_budget.digest()),
        ]);
        let digest = hash_parts(&[
            "worth_query_graph_read_streaming_plan_v1".to_string(),
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
    requirements: &WorthQueryGraphReadAccessRequirementSet,
) -> bool {
    requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::ReverseAdjacency)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::TraversalWorkset)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::VisitedSet)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::DedupSet)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::ProofSupport)
        && !requirements
            .requires_kind(WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport)
}
