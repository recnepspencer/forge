use std::fs;
use std::path::Path;

#[test]
fn hostile_current_head_baseline_callers_use_current_head_materialized_helper() {
    let expected_files = [
        "src/certification/projection_closeout/tests/materialization.rs",
        "src/certification/topology_operator_closeout/scenario_programs/ambiguous_local_rewire.rs",
        "src/certification/topology_operator_closeout/scenario_programs/bowtie_adjacent.rs",
        "src/certification/topology_operator_closeout/scenario_programs/broken_radial_localization.rs",
        "src/certification/topology_operator_closeout/scenario_programs/cancellation_chain.rs",
        "src/certification/topology_operator_closeout/scenario_programs/split_collapse_churn.rs",
    ];

    for relative in expected_files {
        let source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(relative))
            .expect("current-head baseline proof file should remain readable");
        assert!(
            source.contains("current_head_materialized_topology("),
            "{relative} should use the explicit current-head materialized helper",
        );
        assert!(
            !source.contains("historical_query_snapshot_for_read_basis("),
            "{relative} should not rebuild a historical snapshot just to read the current-head baseline",
        );
    }
}

#[test]
fn current_head_materialized_helper_has_no_local_materialization_fallback() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/support/current_head_materialized_topology.rs"),
    )
    .expect("current-head materialized helper should remain readable");

    assert!(!source.contains("fallback_current_head_materialized_topology"));
    assert!(!source.contains("TopologyMaterializer::materialize_query_input"));
    assert!(!source.contains("TopologyQueryMaterializationInput::decode"));
}
