use forge_store_certification::{
    require_complete_s8_runtime_matrix, S8RuntimeMatrixDenial, S8RuntimeStrategyEquivalenceClass,
};
use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;
use forge_store_physical_certification::layout_harness::runtime::{
    S8RuntimeCase, S8RuntimeCoverageMatrix,
};

#[test]
fn corruption_rebuild_cannot_be_certified_from_scenario_inventory() {
    let matrix = S8RuntimeCoverageMatrix::default();

    assert_eq!(
        require_complete_s8_runtime_matrix(&matrix),
        Err(S8RuntimeMatrixDenial::MissingExecutedStrategyCase {
            strategy: S8LayoutStrategyFamily::AppendLog,
            equivalence_class: S8RuntimeStrategyEquivalenceClass::RecoveryReplayStructure,
            case: S8RuntimeCase::Success,
        })
    );
}
