use std::collections::HashMap;

use super::counters::PlanarBooleanSplitDecisionLogCounters;
use super::denial::{PlanarBooleanSplitDecisionLogDenial, PlanarBooleanSplitDecisionLogDenialKind};
use super::row::PlanarBooleanSplitDecisionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlanarBooleanSplitDecisionLookupIndex {
    by_decision_identity: HashMap<String, usize>,
    by_affected_artifact_identity: HashMap<String, Vec<usize>>,
}

impl PlanarBooleanSplitDecisionLookupIndex {
    pub(super) fn build(
        rows: &[PlanarBooleanSplitDecisionRow],
        counters: &mut PlanarBooleanSplitDecisionLogCounters,
    ) -> Result<Self, PlanarBooleanSplitDecisionLogDenial> {
        let mut by_decision_identity = HashMap::with_capacity(rows.len());
        let mut by_affected_artifact_identity = HashMap::<String, Vec<usize>>::new();
        for (index, row) in rows.iter().enumerate() {
            if row.decision_identity().is_empty() {
                counters.rejected_missing_coverage();
                return Err(PlanarBooleanSplitDecisionLogDenial::new(
                    PlanarBooleanSplitDecisionLogDenialKind::MissingDecisionIdentity,
                    row.affected_artifact_identity(),
                    *counters,
                    "split decision rows require a canonical decision identity",
                ));
            }
            if by_decision_identity
                .insert(row.decision_identity().to_string(), index)
                .is_some()
            {
                counters.rejected_duplicate_decision_identity();
                return Err(PlanarBooleanSplitDecisionLogDenial::new(
                    PlanarBooleanSplitDecisionLogDenialKind::DuplicateDecisionIdentity,
                    row.decision_identity(),
                    *counters,
                    "split decision log rejects duplicate decision identities",
                ));
            }
            by_affected_artifact_identity
                .entry(row.affected_artifact_identity().to_string())
                .or_default()
                .push(index);
        }
        counters.set_index_entries(
            by_decision_identity.len(),
            by_affected_artifact_identity.len(),
        );
        Ok(Self {
            by_decision_identity,
            by_affected_artifact_identity,
        })
    }

    pub(super) fn decision_index(&self, decision_identity: &str) -> Option<usize> {
        self.by_decision_identity.get(decision_identity).copied()
    }

    pub(super) fn artifact_indexes(&self, artifact_identity: &str) -> Option<&[usize]> {
        self.by_affected_artifact_identity
            .get(artifact_identity)
            .map(Vec::as_slice)
    }

    pub(super) fn has_artifact_identity(&self, artifact_identity: &str) -> bool {
        self.by_affected_artifact_identity
            .contains_key(artifact_identity)
    }
}
