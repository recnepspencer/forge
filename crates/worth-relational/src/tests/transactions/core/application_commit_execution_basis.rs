use std::sync::Arc;

use crate::facade::history::{BranchId, CommitId};
use crate::facade::runtime::{
    RelationalApplicationCommitBasisDenial, RelationalApplicationCommitBasisSource,
    RelationalExecutionBasisLease, RelationalRetainedCommitSnapshotDenialKind,
};
use crate::tests::support::*;

#[test]
fn application_commit_source_admits_the_whole_exact_retained_commit() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "retained");
    let runtime = Arc::new(runtime);
    let runtime_instance_id = runtime.runtime_instance_id();
    let branch_cells_before = runtime.history().branch_cells_snapshot();
    let source = RelationalApplicationCommitBasisSource::for_runtime(Arc::clone(&runtime));

    let lease = source
        .admit_application_commit(runtime_instance_id, &created.commit)
        .expect("the whole retained application commit admits one execution basis");

    assert_eq!(lease.identity().runtime_instance_id(), runtime_instance_id);
    assert_eq!(lease.identity().branch_id(), &created.commit.branch_id);
    assert_eq!(lease.version_id(), created.commit.version_id);
    assert_eq!(
        runtime.history().branch_cells_snapshot(),
        branch_cells_before,
        "historical application-commit admission cannot move branch currentness"
    );
}

#[test]
fn application_commit_source_rejects_foreign_and_partial_commit_substitution() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "owned");
    let runtime_instance_id = runtime.runtime_instance_id();
    let source = RelationalApplicationCommitBasisSource::for_runtime(Arc::new(runtime));

    let foreign =
        denial_from(source.admit_application_commit(runtime_instance_id + 1, &created.commit));
    assert!(matches!(
        foreign,
        RelationalApplicationCommitBasisDenial::RetainedCommit(ref denial)
            if denial.kind() == RelationalRetainedCommitSnapshotDenialKind::ForeignRuntime
    ));

    let mut wrong_branch = created.commit.clone();
    wrong_branch.branch_id = BranchId("other-branch".to_owned());
    let branch = denial_from(source.admit_application_commit(runtime_instance_id, &wrong_branch));
    assert!(matches!(
        branch,
        RelationalApplicationCommitBasisDenial::RetainedCommit(ref denial)
            if denial.kind() == RelationalRetainedCommitSnapshotDenialKind::BranchMismatch
    ));

    let mut wrong_parents = created.commit.clone();
    wrong_parents.parents.push(CommitId(u64::MAX));
    let commit = denial_from(source.admit_application_commit(runtime_instance_id, &wrong_parents));
    assert!(matches!(
        commit,
        RelationalApplicationCommitBasisDenial::RetainedCommit(ref denial)
            if denial.kind() == RelationalRetainedCommitSnapshotDenialKind::CommitMismatch
    ));
}

#[test]
fn application_commit_source_refuses_pruned_publication() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 1,
        })
        .build();
    let first = create_entity_outcome(&mut runtime, "first");
    create_entity(&mut runtime, "second");
    let runtime_instance_id = runtime.runtime_instance_id();
    let source = RelationalApplicationCommitBasisSource::for_runtime(Arc::new(runtime));

    let denial = denial_from(source.admit_application_commit(runtime_instance_id, &first.commit));

    assert!(matches!(
        denial,
        RelationalApplicationCommitBasisDenial::RetainedCommit(ref denial)
            if denial.kind() == RelationalRetainedCommitSnapshotDenialKind::SnapshotNotRetained
    ));
}

fn denial_from(
    result: Result<RelationalExecutionBasisLease, RelationalApplicationCommitBasisDenial>,
) -> RelationalApplicationCommitBasisDenial {
    match result {
        Ok(lease) => {
            drop(lease);
            panic!("application commit basis admission unexpectedly succeeded")
        }
        Err(denial) => denial,
    }
}
