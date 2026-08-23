use super::super::owner_coverage::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let observed =
        worth_store_test_support::harness::layout_evolution::observe_layout_evolution_owner_cases();
    for case in observed.binding() {
        ledger.record_layout_binding_admission(case);
    }
    for case in observed.migration_planning() {
        ledger.record_migration_planning(case);
    }
    for case in observed.rollback_planning() {
        ledger.record_rollback_planning(case);
    }
    for case in observed.backward_read() {
        ledger.record_backward_read_compatibility(case);
    }
}
