use crate::branch::RelationalForkDenial;
use crate::history::data::BranchId;
use crate::tests::support::{create_entity_outcome, runtime_with_test_schema};

#[test]
fn missing_source_denial_leaves_registry_and_catalog_unchanged() {
    let mut runtime = runtime_with_test_schema();
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_catalog_count = runtime.history.commit_catalog.len();
    let missing = BranchId("missing-source".to_owned());

    assert!(matches!(
        runtime.observe_fork_source(&missing),
        Err(RelationalForkDenial::SourceBranchMissing)
    ));

    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.branch_count(), 1);
    assert_eq!(
        runtime.history.commit_catalog.len(),
        before_catalog_count,
        "a missing source must not materialize or publish an artifact"
    );
}

#[test]
fn missing_artifact_denial_leaves_source_and_registry_unchanged() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "fork-source");
    let source = BranchId("main".to_owned());
    let target = BranchId("missing-artifact-target".to_owned());
    let (_, basis) = runtime
        .observe_fork_source(&source)
        .expect("committed source is forkable");

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(commit.commit.commit_id));
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_source = runtime
        .history
        .branch_cell(&source)
        .expect("source remains registered")
        .checkpoint();
    let before_catalog_count = runtime.history.commit_catalog.len();

    assert_eq!(
        runtime.fork_branch(target.clone(), basis),
        Err(RelationalForkDenial::MissingArtifact)
    );

    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.branch_count(), 1);
    assert_eq!(
        runtime
            .history
            .branch_cell(&source)
            .expect("source remains registered")
            .checkpoint(),
        before_source,
        "missing artifact denial must not retain or advance the source"
    );
    assert!(runtime.history.branch_cell(&target).is_none());
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog_count);
}

#[test]
fn retention_overflow_denial_leaves_fork_source_and_registry_unchanged() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "retention-source");
    let source = BranchId("main".to_owned());
    let target = BranchId("retention-overflow-target".to_owned());
    let (_, basis) = runtime
        .observe_fork_source(&source)
        .expect("committed source is forkable");
    runtime
        .history
        .branch_cell_mut(&source)
        .expect("source remains registered")
        .set_head_retention_obligations_for_test(u32::MAX);

    let before_cells = runtime.history.branch_cells_snapshot();
    let before_source = runtime
        .history
        .branch_cell(&source)
        .expect("source remains registered")
        .checkpoint();
    let before_catalog_count = runtime.history.commit_catalog.len();
    let before_envelope_count = runtime.history.commit_envelopes.len();

    assert_eq!(
        runtime.fork_branch(target.clone(), basis),
        Err(RelationalForkDenial::Cell(
            crate::branch::RelationalBranchCellDenial::RetentionOverflow,
        ))
    );

    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.branch_count(), 1);
    assert_eq!(
        runtime
            .history
            .branch_cell(&source)
            .expect("source remains registered")
            .checkpoint(),
        before_source,
        "retention denial must not mutate the source cell"
    );
    assert!(runtime.history.branch_cell(&target).is_none());
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog_count);
    assert_eq!(
        runtime.history.commit_envelopes.len(),
        before_envelope_count
    );
    assert_eq!(
        runtime
            .history
            .commit_envelopes
            .get(&commit.commit.commit_id)
            .map(|envelope| envelope.commit.commit_id),
        Some(commit.commit.commit_id)
    );
}
