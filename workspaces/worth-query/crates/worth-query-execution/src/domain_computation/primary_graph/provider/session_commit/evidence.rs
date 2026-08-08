//! Evidence handed from provider commit to the exact receipt resolver.

use std::collections::BTreeMap;

use crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage;
use worth_relational::facade::history::CommitId;

use super::super::mutation_work::WorthQueryPrimaryMutationWorkEvidence;

pub(in crate::domain_computation::primary_graph) struct WorthQueryPrimaryGraphCommitEvidence {
    mutation_work: WorthQueryPrimaryMutationWorkEvidence,
    retained_preimage: Option<WorthQueryRetainedPreImage>,
}

/// Affine handoff indexed by the Relational commit that produced the evidence.
#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryCompletedCommitEvidenceStore {
    by_commit: BTreeMap<CommitId, WorthQueryPrimaryGraphCommitEvidence>,
}

impl WorthQueryCompletedCommitEvidenceStore {
    pub(super) fn record(
        &mut self,
        commit: CommitId,
        evidence: WorthQueryPrimaryGraphCommitEvidence,
    ) {
        assert!(
            self.by_commit.insert(commit, evidence).is_none(),
            "one Relational commit may record provider evidence only once"
        );
    }

    pub(in crate::domain_computation::primary_graph) fn take(
        &mut self,
        commit: CommitId,
    ) -> Option<WorthQueryPrimaryGraphCommitEvidence> {
        self.by_commit.remove(&commit)
    }

    #[cfg(test)]
    pub(in crate::domain_computation::primary_graph) fn len(&self) -> usize {
        self.by_commit.len()
    }
}

impl WorthQueryPrimaryGraphCommitEvidence {
    pub(in crate::domain_computation::primary_graph) const fn new(
        mutation_work: WorthQueryPrimaryMutationWorkEvidence,
        retained_preimage: Option<WorthQueryRetainedPreImage>,
    ) -> Self {
        Self {
            mutation_work,
            retained_preimage,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn into_parts(
        self,
    ) -> (
        WorthQueryPrimaryMutationWorkEvidence,
        Option<WorthQueryRetainedPreImage>,
    ) {
        (self.mutation_work, self.retained_preimage)
    }
}

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{
        AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey,
    };
    use worth_query_installation::facade::InstalledCorrectionMechanism;
    use worth_relational::facade::history::CommitId;
    use worth_relational::facade::identity::{EntityId, KindId, PartitionId};
    use worth_relational::facade::transactions::{planned_aspect_field_locator, RecordRef};

    use super::super::super::mutation_work::{
        WorthQueryPrimaryMutationWorkCounters, WorthQueryPrimaryMutationWorkEvidence,
    };
    use super::*;
    use crate::domain_computation::application_aftermath::{
        undo_preimage::retain_preimage_from_test_footprint, WorthQueryObservedPreImageCandidate,
    };

    #[test]
    fn wrong_commit_cannot_consume_or_substitute_completed_evidence() {
        let mut store = WorthQueryCompletedCommitEvidenceStore::default();
        store.record(CommitId(11), evidence(1, 1));
        store.record(CommitId(12), evidence(2, 2));

        assert!(store.take(CommitId(99)).is_none());
        let (second, _) = store.take(CommitId(12)).unwrap().into_parts();
        assert_eq!(second.decision_fact_count(), 2);
        assert_eq!(second.touched_records()[0].record(), &record(2));
        let (first, _) = store.take(CommitId(11)).unwrap().into_parts();
        assert_eq!(first.decision_fact_count(), 1);
        assert_eq!(first.touched_records()[0].record(), &record(1));
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn retained_preimage_and_mutation_work_move_as_one_exact_commit_bundle() {
        let mut store = WorthQueryCompletedCommitEvidenceStore::default();
        store.record(
            CommitId(11),
            WorthQueryPrimaryGraphCommitEvidence::new(work(1, 1), Some(retained_preimage())),
        );
        store.record(
            CommitId(12),
            WorthQueryPrimaryGraphCommitEvidence::new(work(2, 2), None),
        );

        let (ordinary_work, ordinary_preimage) = store.take(CommitId(12)).unwrap().into_parts();
        assert_eq!(ordinary_work.decision_fact_count(), 2);
        assert!(ordinary_preimage.is_none());
        let (inverse_work, inverse_preimage) = store.take(CommitId(11)).unwrap().into_parts();
        assert_eq!(inverse_work.decision_fact_count(), 1);
        assert_eq!(
            inverse_preimage.unwrap().field("frozen").unwrap().value(),
            &AspectValue::Bool(false)
        );
    }

    fn evidence(decision_facts: usize, entity_slot: u64) -> WorthQueryPrimaryGraphCommitEvidence {
        WorthQueryPrimaryGraphCommitEvidence::new(work(decision_facts, entity_slot), None)
    }

    fn work(decision_facts: usize, entity_slot: u64) -> WorthQueryPrimaryMutationWorkEvidence {
        let counters = WorthQueryPrimaryMutationWorkCounters::new(decision_facts, 1, 1, 1, 1, 1);
        WorthQueryPrimaryMutationWorkEvidence::from_commit(counters, &[record(entity_slot)])
    }

    fn retained_preimage() -> WorthQueryRetainedPreImage {
        let aftermath = crate::domain_computation::application_aftermath::aftermath_schema_fixture::freeze_account();
        let Some(InstalledCorrectionMechanism::RecordedInverse(inverse)) = aftermath.mechanism()
        else {
            panic!("fixture must install a recorded inverse")
        };
        let locator = locator();
        retain_preimage_from_test_footprint(
            inverse.preimage_demand(),
            &[WorthQueryObservedPreImageCandidate::from_observed_field(
                locator.clone(),
                AspectValue::Bool(false),
                entity(1),
                KindId(7),
            )],
            &[(record(1), locator)],
        )
        .unwrap()
    }

    fn locator() -> AspectFieldLocator {
        planned_aspect_field_locator(
            AspectKey::new("estate").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("frozen").unwrap()),
        )
    }

    fn record(slot: u64) -> RecordRef {
        RecordRef::Entity(entity(slot))
    }

    fn entity(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }
}
