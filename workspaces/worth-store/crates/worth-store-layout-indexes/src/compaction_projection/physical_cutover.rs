use worth_store_physical_isolation::{
    CompactionCutoverState, CompactionOwnerCaseId, CompactionOwnerCaseObservation,
};

pub const fn observe_physical_cutover(
    owner_case: &CompactionOwnerCaseObservation,
) -> (
    CompactionCutoverState,
    CompactionOwnerCaseId,
    CompactionCutoverState,
) {
    (owner_case.from(), owner_case.id(), owner_case.to())
}

#[cfg(test)]
mod tests {
    use super::observe_physical_cutover;
    use std::collections::BTreeSet;
    use worth_store_physical_isolation::{
        compaction_owner_case_inventory, CompactionCutoverStabilityProof,
        CompactionDeferredReclaimQueue, CompactionOwnerCaseObservation,
    };
    use worth_store_test_support::harness::physical_isolation::compaction;
    use worth_store_test_support::harness::recovery::compaction_mutation;

    #[test]
    fn projection_accepts_only_owner_observations_and_preserves_identity() {
        let plan = compaction::admitted_compaction_plan();
        let executed = compaction::execute_compaction_cutover(&plan);
        let (publication, recovery, pre_cutover_read, _) = executed.into_parts();
        let proof = CompactionCutoverStabilityProof::admit(publication.clone(), recovery).unwrap();
        let queue = CompactionDeferredReclaimQueue::admit(publication.clone()).unwrap();
        let drained = queue
            .clone()
            .drain_after_release(pre_cutover_read.read_plan_release())
            .unwrap();
        let mut observations = vec![
            publication.delta().owner_case_observation(),
            publication.owner_case_observation(),
            proof.owner_case_observation(),
            queue.owner_case_observation(),
            drained.owner_case_observation(),
        ];
        observations.extend(
            compaction_mutation::complete_compaction_mutation_receipts()
                .into_iter()
                .map(|receipt| receipt.owner_case_observation()),
        );

        let projected = observations
            .into_iter()
            .map(|observation| {
                assert_projection_preserves(observation);
                observe_physical_cutover(&observation).1.name()
            })
            .collect::<BTreeSet<_>>();
        let declared = compaction_owner_case_inventory()
            .map(|case| case.id().name())
            .collect::<BTreeSet<_>>();

        assert_eq!(projected, declared);
        assert_eq!(projected.len(), 11);
    }

    fn assert_projection_preserves(observation: CompactionOwnerCaseObservation) {
        let projected = observe_physical_cutover(&observation);
        assert_eq!(
            projected,
            (observation.from(), observation.id(), observation.to())
        );
    }
}
