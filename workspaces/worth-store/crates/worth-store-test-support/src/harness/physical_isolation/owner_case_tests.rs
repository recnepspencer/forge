use super::compaction::{admitted_compaction_plan, execute_compaction_cutover};
use crate::harness::recovery::compaction_mutation::complete_compaction_mutation_receipts;
use std::collections::BTreeSet;
use worth_store_physical_isolation::{
    compaction_owner_case_inventory, CompactionCutoverStabilityProof,
    CompactionDeferredReclaimQueue,
};

#[test]
fn physical_compaction_inventory_equals_ordinary_owner_observations() {
    let plan = admitted_compaction_plan();
    let (publication, recovery, pre_cutover_read, _) =
        execute_compaction_cutover(&plan).into_parts();
    let proof = CompactionCutoverStabilityProof::admit(publication.clone(), recovery)
        .expect("ordinary recovery evidence admits cutover stability");
    let queue = CompactionDeferredReclaimQueue::admit(publication)
        .expect("ordinary publication admits deferred reclaim");
    let drained = queue
        .clone()
        .drain_after_release(pre_cutover_read.read_plan_release())
        .expect("the executed reader release drains reclaim");

    let mut observed = BTreeSet::from([
        proof
            .publication()
            .delta()
            .owner_case_observation()
            .id()
            .name(),
        proof.publication().owner_case_observation().id().name(),
        proof.owner_case_observation().id().name(),
        queue.owner_case_observation().id().name(),
        drained.owner_case_observation().id().name(),
    ]);
    observed.extend(
        complete_compaction_mutation_receipts()
            .into_iter()
            .map(|receipt| receipt.owner_case_observation().id().name()),
    );
    let declared = compaction_owner_case_inventory()
        .map(|case| case.id().name())
        .collect::<BTreeSet<_>>();

    assert_eq!(observed, declared);
    assert_eq!(declared.len(), 11);
}
