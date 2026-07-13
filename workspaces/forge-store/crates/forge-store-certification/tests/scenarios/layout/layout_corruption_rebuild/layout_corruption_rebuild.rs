use forge_store_certification::{
    require_complete_layout_runtime_matrix, LayoutRuntimeCompletenessDenial,
    LayoutRuntimeStrategyEquivalenceClass,
};
use forge_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;
use forge_store_physical_certification::layout_harness::runtime::{
    LayoutRuntimeCoverageMatrix, LayoutRuntimeObligation,
};

#[test]
fn corruption_rebuild_cannot_be_certified_from_scenario_inventory() {
    let matrix = LayoutRuntimeCoverageMatrix::default();

    assert_eq!(
        require_complete_layout_runtime_matrix(&matrix),
        Err(
            LayoutRuntimeCompletenessDenial::MissingExecutedStrategyCase {
                strategy: LayoutStrategyFamily::AppendLog,
                equivalence_class: LayoutRuntimeStrategyEquivalenceClass::RecoveryReplayStructure,
                case: LayoutRuntimeObligation::Success,
            }
        )
    );
}
