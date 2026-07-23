use crate::runtime::{
    WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiNodeReplacementPlan {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    classifications: Vec<WorthUiNodeReplacementClassification>,
    changed_classifications: Vec<WorthUiNodeReplacementClassification>,
    candidate_structural_node_count: usize,
    counters: WorthUiNodeReplacementCounters,
}

impl WorthUiNodeReplacementPlan {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut classifications: Vec<WorthUiNodeReplacementClassification>,
        counters: WorthUiNodeReplacementCounters,
    ) -> Self {
        classifications.sort_by(|left, right| {
            left.identity_basis()
                .cmp(right.identity_basis())
                .then_with(|| left.transition().cmp(&right.transition()))
        });
        let changed_classifications = classifications
            .iter()
            .filter(|classification| {
                classification.transition() != WorthUiNodeLifecycleTransition::Preserve
            })
            .cloned()
            .collect();
        let candidate_structural_node_count = classifications
            .iter()
            .filter(|classification| {
                matches!(classification.candidate_kind(), Some(kind)
                    if kind != crate::runtime::WorthUiIdentityMatchNodeKind::Binding)
            })
            .count();
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            classifications,
            changed_classifications,
            candidate_structural_node_count,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn classifications(&self) -> &[WorthUiNodeReplacementClassification] {
        &self.classifications
    }

    pub(crate) fn changed_classifications(&self) -> &[WorthUiNodeReplacementClassification] {
        &self.changed_classifications
    }

    pub(crate) fn candidate_structural_node_count(&self) -> usize {
        self.candidate_structural_node_count
    }

    pub fn counters(&self) -> WorthUiNodeReplacementCounters {
        self.counters
    }

    pub fn is_unambiguous(&self) -> bool {
        self.counters.ambiguous_node_count() == 0
    }

    pub fn transition_for_identity(
        &self,
        identity_basis: &str,
    ) -> Option<WorthUiNodeLifecycleTransition> {
        self.classifications
            .iter()
            .find(|classification| classification.identity_basis() == identity_basis)
            .map(WorthUiNodeReplacementClassification::transition)
    }
}
