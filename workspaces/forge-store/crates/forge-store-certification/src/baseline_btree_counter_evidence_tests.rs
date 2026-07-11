use forge_store_budgets::CounterEvidenceStrength;
use forge_store_test_support::harness::execute_s8_layout_runtime_receipt;

#[test]
fn exact_strategy_counters_come_from_real_btree_execution() {
    let executed = execute_s8_layout_runtime_receipt();

    assert_eq!(executed.amplification_receipt().page_touches(), 2);
    assert_eq!(executed.amplification_receipt().index_probes(), 2);
    assert_eq!(executed.amplification_receipt().read_amplification(), 2);
    assert_eq!(executed.amplification_receipt().write_amplification(), 0);
    assert_eq!(executed.planned_vs_observed().planned().point_lookups(), 1);
    assert_eq!(executed.planned_vs_observed().observed().point_lookups(), 1);
    assert!(executed.planned_vs_observed().parity_holds());
    assert_eq!(
        executed.performance_receipt().counter_strength(),
        CounterEvidenceStrength::Exact,
    );
}
