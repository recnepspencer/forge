use super::BridgeSubscriptionCounters;

#[test]
fn basis_kind_divergence_rejection_counters_match_actual_work() {
    let counters = BridgeSubscriptionCounters::from_basis_kind_divergence_rejection();

    assert_eq!(counters.declaration_rejection_count(), 0);
    assert_eq!(counters.basis_request_count(), 1);
    assert_eq!(counters.basis_binding_count(), 0);
    assert_eq!(counters.basis_rejection_count(), 1);
    assert_eq!(counters.signal_strategy_selection_count(), 0);
    assert_eq!(counters.signal_strategy_rejection_count(), 0);
}
