use super::LayoutOwnerObservationLedger;
use forge_store_physical_isolation::{
    CompactionCutoverStabilityProof, CompactionDeferredReclaimQueue,
};
use forge_store_test_support::harness::{
    observe_lsm_owner_cases, physical_isolation::compaction, recovery::compaction_mutation,
};

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    record_durable_membership_cases(ledger);
    record_physical_compaction_cases(ledger);
}

fn record_durable_membership_cases(ledger: &mut LayoutOwnerObservationLedger) {
    let observations = observe_lsm_owner_cases();
    for observation in observations.membership() {
        ledger.record_lsm_membership(observation);
    }
    for observation in observations.execution() {
        ledger.record_lsm_execution(observation);
    }
}

fn record_physical_compaction_cases(ledger: &mut LayoutOwnerObservationLedger) {
    let plan = compaction::admitted_compaction_plan();
    let (publication, recovery, pre_cutover_read, _) =
        compaction::execute_compaction_cutover(&plan).into_parts();
    let proof = CompactionCutoverStabilityProof::admit(publication.clone(), recovery)
        .expect("ordinary recovery evidence admits cutover stability");
    let queue = CompactionDeferredReclaimQueue::admit(publication)
        .expect("ordinary publication admits deferred reclaim");
    let drained = queue
        .clone()
        .drain_after_release(pre_cutover_read.read_plan_release())
        .expect("the executed reader release drains reclaim");

    for observation in [
        proof.publication().delta().owner_case_observation(),
        proof.publication().owner_case_observation(),
        proof.owner_case_observation(),
        queue.owner_case_observation(),
        drained.owner_case_observation(),
    ] {
        ledger.record_physical_compaction(observation);
    }
    for receipt in compaction_mutation::complete_compaction_mutation_receipts() {
        ledger.record_physical_compaction(receipt.owner_case_observation());
    }
}
