use super::{UnsupportedReplayReasonCode, UnsupportedReplayWorkload};
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedArtifactSet {
    retained_planar_facts: RetainedPlanarFactsReceipt,
    projection_consumed_facts: Option<ProjectionConsumedPlanarFactsReceipt>,
}

impl RetainedArtifactSet {
    pub fn from_retained_planar_facts(retained_planar_facts: RetainedPlanarFactsReceipt) -> Self {
        Self {
            retained_planar_facts,
            projection_consumed_facts: None,
        }
    }

    pub fn with_projection_consumed_facts(
        mut self,
        projection_consumed_facts: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.projection_consumed_facts = Some(projection_consumed_facts);
        self
    }

    pub fn retained_planar_facts(&self) -> &RetainedPlanarFactsReceipt {
        &self.retained_planar_facts
    }

    pub fn retained_artifact_identity(&self) -> String {
        match &self.projection_consumed_facts {
            Some(projection) => format!(
                "retained-artifacts:{}:{}",
                self.retained_planar_facts.retained_fact_digest(),
                projection.projection_consumption_digest()
            ),
            None => format!(
                "retained-artifacts:{}:missing-projection-consumed-facts",
                self.retained_planar_facts.retained_fact_digest()
            ),
        }
    }

    pub fn retained_basis_identity(&self) -> String {
        format!(
            "retained-basis:{}:{}:{}:{}:{}",
            self.retained_planar_facts.declaration_digest(),
            self.retained_planar_facts.progression_digest(),
            self.retained_planar_facts.route_plan_digest(),
            self.retained_planar_facts.query_receipt_digest(),
            self.retained_planar_facts.envelope_digest()
        )
    }

    pub fn replay_checkpoint_identity(&self) -> String {
        format!(
            "retained-replay-checkpoint:{}:{}",
            self.retained_planar_facts.query_receipt_digest(),
            self.retained_planar_facts.retained_fact_digest()
        )
    }

    pub fn retained_artifact_rows(&self) -> usize {
        1 + usize::from(self.projection_consumed_facts.is_some())
    }

    pub fn projection_consumed_rows(&self) -> usize {
        self.projection_consumed_facts
            .as_ref()
            .map(|projection| projection.counters().projection_receipts_consumed())
            .unwrap_or(0)
    }

    pub(crate) fn require_projection_consumed_facts(
        &self,
    ) -> Result<&ProjectionConsumedPlanarFactsReceipt, UnsupportedReplayWorkload> {
        let Some(projection) = self.projection_consumed_facts.as_ref() else {
            return Err(UnsupportedReplayWorkload::new(
                UnsupportedReplayReasonCode::MissingProjectionConsumedFacts,
                "Retained replay workload requires projection-consumed facts captured from the retained planar artifact.",
            ));
        };
        if projection.retained_planar_fact_digest()
            != self.retained_planar_facts.retained_fact_digest()
        {
            return Err(UnsupportedReplayWorkload::new(
                UnsupportedReplayReasonCode::RetainedProjectionDrift,
                "Retained replay workload requires projection-consumed facts to reference the same retained planar fact digest as the retained artifact.",
            ));
        }
        Ok(projection)
    }
}
