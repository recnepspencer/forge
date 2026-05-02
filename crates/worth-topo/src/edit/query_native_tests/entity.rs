use forge_query::facade::ForgeQueryMutationFamily;
use forge_relational::facade::history::BranchId;
use worth_schema::facade::{seed_minimal_topology, WorthTopologyEntityKind};

use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditBatch, WorthTopologyEditContract,
    WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn query_native_edit_runner_applies_create_topology_entity_on_production_runtime() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.create").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
            "worth.query-native-edit.vertex",
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native create should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::CreateTopologyEntity]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert!(execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.label == "worth.query-native-edit.vertex"));
}

#[test]
fn query_native_edit_runner_denies_branch_local_mode_until_runtime_supports_it() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.branch").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
            "worth.query-native-edit.branch.vertex",
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(
            batch,
            WorthTopologyEditApplicationMode::BranchLocal(BranchId("feature".to_string())),
        )
        .expect_err("branch-local mode should fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedMode(
            WorthTopologyEditApplicationMode::BranchLocal(_)
        )
    ));
}

#[test]
fn query_native_edit_runner_applies_retire_topology_entity_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.retire").expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.retire").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::retire_topology_entity(
            seeded.vertex,
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native retire should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::RetireTopologyEntity]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert_eq!(
        execution.receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Delete
    );
    assert_eq!(
        execution.inspection.component_operations()[0].family(),
        "delete"
    );
    let binding = execution.inspection.component_operations()[0]
        .existing_truth_binding_evidence()
        .expect("retire execution should preserve existing-truth binding evidence");
    assert_eq!(binding.family().as_str(), "direct-entity-identity");
    assert_eq!(binding.target_collection(), Some("WorthTopologyEntity"));
    assert!(!execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.entity_id == seeded.vertex));
}

#[test]
fn query_native_edit_runner_denies_retire_kind_mismatch_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.retire-kind").expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.retire-kind").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::retire_topology_entity(
            seeded.face,
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("retire kind mismatch must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
            entity_id,
            expected: WorthTopologyEntityKind::Vertex,
            actual: WorthTopologyEntityKind::Face,
        } if entity_id == seeded.face
    ));
}
