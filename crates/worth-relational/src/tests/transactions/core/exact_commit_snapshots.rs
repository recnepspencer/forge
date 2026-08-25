use crate::facade::history::{BranchId, CommitId};
use crate::facade::runtime::RelationalRetainedCommitSnapshotDenialKind;
use crate::tests::support::*;

#[test]
fn exact_commit_snapshot_survives_later_update_and_delete_without_reconstruction() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let created_commit = created.commit.clone();
    let updated = update_entity(&mut runtime, entity, "after");
    let updated_commit = updated.commit.clone();
    let runtime_instance_id = runtime.runtime_instance_id();

    runtime.performance_access().reset_counters();
    let created_observation = runtime
        .snapshots()
        .retained_snapshot_for_commit(runtime_instance_id, &created_commit)
        .expect("created commit publication remains retained");
    let counters = runtime.performance_access().counters();
    assert_eq!(counters.visibility_cache_miss_reconstructions, 0);
    assert!(counters.visibility_exact_state_materializations <= 1);
    assert_eq!(created_observation.commit(), &created_commit);
    assert_eq!(
        created_observation.snapshot_handle().version_id,
        created_commit.version_id
    );
    let created_record = runtime
        .read_truth()
        .project_snapshot(created_observation.snapshot_handle())
        .expect("retained handle resolves")
        .all_authoritative_entity_records()
        .into_iter()
        .find(|record| record.entity_id == entity)
        .expect("entity existed at creation commit");
    assert_eq!(read_entity_name(&created_record), Some("before".to_owned()));

    delete_entity(&mut runtime, entity);
    let updated_observation = runtime
        .snapshots()
        .retained_snapshot_for_commit(runtime_instance_id, &updated_commit)
        .expect("updated commit publication remains retained after deletion");
    let updated_record = runtime
        .read_truth()
        .project_snapshot(updated_observation.snapshot_handle())
        .expect("retained update handle resolves")
        .all_authoritative_entity_records()
        .into_iter()
        .find(|record| record.entity_id == entity)
        .expect("entity existed at update commit");
    assert_eq!(read_entity_name(&updated_record), Some("after".to_owned()));
    assert!(runtime
        .read_truth()
        .project_historical_version(runtime.current_version_id())
        .all_authoritative_entity_records()
        .into_iter()
        .all(|record| record.entity_id != entity));
}

#[test]
fn exact_commit_snapshot_rejects_foreign_runtime_branch_and_commit_identity() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "owned");
    let runtime_instance_id = runtime.runtime_instance_id();

    let mut foreign_runtime = runtime_with_test_schema();
    let foreign = foreign_runtime
        .snapshots()
        .retained_snapshot_for_commit(runtime_instance_id, &created.commit)
        .expect_err("foreign runtime identity must be denied first");
    assert_eq!(
        foreign.kind(),
        RelationalRetainedCommitSnapshotDenialKind::ForeignRuntime
    );

    let mut wrong_branch = created.commit.clone();
    wrong_branch.branch_id = BranchId("foreign-branch".to_owned());
    let branch_denial = runtime
        .snapshots()
        .retained_snapshot_for_commit(runtime_instance_id, &wrong_branch)
        .expect_err("branch substitution must be denied");
    assert_eq!(
        branch_denial.kind(),
        RelationalRetainedCommitSnapshotDenialKind::BranchMismatch
    );

    let mut wrong_commit = created.commit.clone();
    wrong_commit.parents.push(CommitId(u64::MAX));
    let commit_denial = runtime
        .snapshots()
        .retained_snapshot_for_commit(runtime_instance_id, &wrong_commit)
        .expect_err("partial commit-reference equality must be denied");
    assert_eq!(
        commit_denial.kind(),
        RelationalRetainedCommitSnapshotDenialKind::CommitMismatch
    );
}

#[test]
fn exact_commit_snapshot_refuses_pruned_publication_instead_of_reconstructing() {
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

    runtime.performance_access().reset_counters();
    let denial = runtime
        .snapshots()
        .retained_snapshot_for_commit(runtime_instance_id, &first.commit)
        .expect_err("pruned publication must not reconstruct");
    let counters = runtime.performance_access().counters();

    assert_eq!(
        denial.kind(),
        RelationalRetainedCommitSnapshotDenialKind::SnapshotNotRetained
    );
    assert_eq!(counters.visibility_cache_miss_reconstructions, 0);
}
