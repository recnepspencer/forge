use crate::facade::history::BranchId;
use crate::facade::merge::{MergeExecutionError, MergeExecutionRequest, MergeIntent};
use crate::tests::support::{
    create_branch_from_main, create_entity, create_entity_outcome_on_branch,
    persisted_runtime_with_test_schema,
};

use super::schema_fixtures::drifted_schema_registry;

#[test]
fn authoritative_merge_execution_certification_rejects_stale_prepared_merge_after_target_advances()
{
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    create_entity(&mut runtime, "main-advance");

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("main".to_string()));
        }
        other => panic!("expected target stale-head rejection, got {other:?}"),
    }
}

#[test]
fn authoritative_merge_execution_certification_rejects_stale_prepared_merge_after_source_advances()
{
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-advance",
        BranchId("feature".to_string()),
    );

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("feature".to_string()));
        }
        other => panic!("expected source stale-head rejection, got {other:?}"),
    }
}

#[test]
fn authoritative_merge_execution_certification_rejects_schema_semantic_drift_after_planning() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    runtime.config.schema.registry = drifted_schema_registry();

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::SchemaSemanticDrift { .. }) => {}
        other => panic!("expected schema semantic drift rejection, got {other:?}"),
    }
}
