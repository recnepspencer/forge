#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplaySemanticGraphSelectedPlanIdentity {
    digest: String,
}

impl TopologyReplaySemanticGraphSelectedPlanIdentity {
    pub(crate) fn from_invalidation_selected_plan_digest(
        invalidation_selected_plan_digest: &str,
    ) -> Self {
        Self {
            digest: invalidation_selected_plan_digest.to_string(),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
