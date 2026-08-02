use super::LayoutOwnerObservationLedger;
use worth_store_test_support::harness::observe_lsm_owner_cases;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    record_durable_membership_cases(ledger);
}

fn record_durable_membership_cases(ledger: &mut LayoutOwnerObservationLedger) {
    let observations = observe_lsm_owner_cases();
    for observation in observations.membership() {
        ledger.record_lsm_membership(observation);
    }
}
