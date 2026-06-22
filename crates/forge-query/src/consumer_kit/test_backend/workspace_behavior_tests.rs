use serde_json::Value;

use crate::memory_workspace::ForgeQueryWorkspaceErrorKind;
use crate::runtime::{
    ForgeQueryInspection, ForgeQueryMutationFamily, ForgeQueryPreviewOptions,
    ForgeQueryRuntimeError, InvariantCatalog, InvariantRegistration, InvariantRule,
};
use crate::session_label::ForgeQuerySessionLabel;

use super::{in_memory_test_runtime, ForgeQueryTestBackendSchema};

#[test]
fn in_memory_test_runtime_executes_public_insert_and_live_read() {
    let mut workspace = task_workspace();
    let tasks = workspace
        .live_view::<Value>("consumer-kit.test.tasks", |view| {
            view.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
        })
        .expect("test backend should declare a live view for its collection");

    let receipt = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Write real tests")
        })
        .expect("test backend should execute public workspace insert");

    assert_eq!(workspace.read(&tasks).len(), 1);
    assert_eq!(
        workspace
            .read_live_by_name("consumer-kit.test.tasks")
            .expect("named live read should execute")
            .rows()
            .len(),
        1
    );
    match workspace
        .inspect(&receipt)
        .expect("write receipt should inspect")
    {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection.runtime_evidence().artifact_family(),
                "consumer-kit-in-memory-test-write-receipt"
            );
            assert_eq!(
                inspection.runtime_evidence().evidence(),
                &["consumer-kit-in-memory-test-inspection".to_string()]
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
    let preview_label =
        ForgeQuerySessionLabel::scoped_strs("consumer-kit-test-backend", ["preview"])
            .expect("preview label");
    let preview = workspace
        .preview(preview_label.clone())
        .expect("test backend should admit preview basis");
    assert_eq!(preview.basis_admission().label(), preview_label.display());
    assert_eq!(
        preview.basis_admission().evidence(),
        vec!["consumer-kit-in-memory-test-backend".to_string()]
    );
}

#[test]
fn in_memory_test_runtime_executes_update_delete_and_live_routing() {
    let mut workspace = task_workspace();
    let tasks = workspace
        .live_view::<Value>("consumer-kit.test.crud.tasks", |view| {
            view.from("Task").select(["identity.id", "title.value"])
        })
        .expect("task live view should declare");
    let insert = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-crud")
                .aspect("title.value", "Draft")
        })
        .expect("insert should execute");
    let entity_identity = insert
        .target_entity_identity()
        .expect("insert should expose target identity")
        .clone();

    let update = workspace
        .update(entity_identity.clone(), |task| {
            task.aspect("title.value", "Updated")
        })
        .expect("update should execute");
    assert_eq!(update.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(
        update.affected_live_view_ids(),
        &["consumer-kit.test.crud.tasks".to_string()]
    );
    assert_eq!(
        workspace.read(&tasks)[0].external_row()["title"]["value"].as_str(),
        Some("Updated")
    );

    let delete = workspace
        .delete(entity_identity)
        .expect("delete should execute");
    assert_eq!(delete.mutation_family(), ForgeQueryMutationFamily::Delete);
    assert_eq!(
        delete.affected_live_view_ids(),
        &["consumer-kit.test.crud.tasks".to_string()]
    );
    assert!(workspace.read(&tasks).is_empty());
}

#[test]
fn in_memory_test_runtime_stages_sandboxed_preview_writes_without_authoritative_residue() {
    let mut workspace = task_workspace();
    let preview_label =
        ForgeQuerySessionLabel::scoped_strs("consumer-kit-test-backend", ["sandboxed-preview"])
            .expect("preview label");
    let mut preview = workspace
        .preview_with_options(
            preview_label,
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("sandboxed preview should admit");

    preview
        .insert("Task", |task| {
            task.aspect("identity.id", "preview-task")
                .aspect("title.value", "Preview only")
        })
        .expect("sandboxed preview write should stage");
    let outcome = preview.discard();

    assert!(outcome.discarded());
    assert_eq!(outcome.closeout_evidence().preview_write_staging_count(), 1);
    assert_eq!(outcome.write_count(), 1);
    assert_eq!(outcome.authoritative_residue_count(), 0);
    let tasks = workspace
        .live_view::<Value>("consumer-kit.test.after-preview", |view| {
            view.from("Task").select(["identity.id", "title.value"])
        })
        .expect("live view should declare after preview discard");
    assert!(workspace.read(&tasks).is_empty());
}

#[test]
fn in_memory_test_runtime_denies_wrong_collection_preview_before_residue() {
    let mut workspace = task_workspace();
    let preview_label =
        ForgeQuerySessionLabel::scoped_strs("consumer-kit-test-backend", ["bad-preview"])
            .expect("preview label");
    let mut preview = workspace
        .preview_with_options(
            preview_label,
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("sandboxed preview should admit");

    let error = preview
        .insert("Issue", |issue| {
            issue.aspect("identity.id", "issue-preview")
        })
        .expect_err("preview write should honor backend schema before staging");
    assert_workspace_error_kind(error, ForgeQueryWorkspaceErrorKind::UnsupportedCollection);
    let outcome = preview.discard();
    assert_eq!(outcome.closeout_evidence().preview_write_staging_count(), 0);
    assert_eq!(outcome.write_count(), 0);
    assert_eq!(outcome.authoritative_residue_count(), 0);
}

#[test]
fn in_memory_test_runtime_denies_multi_command_batch_before_partial_residue() {
    let mut workspace = task_workspace();
    let tasks = workspace
        .live_view::<Value>("consumer-kit.test.batch-denial.tasks", |view| {
            view.from("Task").select(["identity.id", "title.value"])
        })
        .expect("task live view should declare");

    let error = workspace
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.aspect("identity.id", "task-batch-1")
                        .aspect("title.value", "First")
                })
                .insert("Issue", |issue| {
                    issue
                        .aspect("identity.id", "issue-batch-2")
                        .aspect("title.value", "Second")
                })
        })
        .expect_err("scaffold backend should deny multi-command batch before execution");

    assert_workspace_error_kind(
        error,
        ForgeQueryWorkspaceErrorKind::BatchAtomicityUnsupported,
    );
    assert!(workspace.read(&tasks).is_empty());
}

#[test]
fn in_memory_test_runtime_fails_closed_for_unsupported_collections() {
    let mut workspace = task_workspace();
    let error = workspace
        .insert("Issue", |issue| {
            issue
                .aspect("identity.id", "issue-1")
                .aspect("title.value", "Wrong family")
        })
        .expect_err("test backend should reject collections outside its schema");

    assert_workspace_error_kind(error, ForgeQueryWorkspaceErrorKind::UnsupportedCollection);
}

#[test]
fn in_memory_test_runtime_lowers_invariant_catalog_into_real_write_denial() {
    let schema = task_schema();
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .invariant_catalog(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::MaxMergedIntents(0),
            )],
        })
        .workspace("consumer-kit.test-backend.invariant-denial")
        .expect("in-memory test runtime with invariant catalog should build");

    let error = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-denied")
                .aspect("title.value", "Denied by invariant")
        })
        .expect_err("registered invariant should deny the write");

    assert_workspace_error_kind(error, ForgeQueryWorkspaceErrorKind::Unclassified);
}

#[test]
fn in_memory_test_runtime_merges_repeated_invariant_catalog_inputs() {
    let schema = task_schema();
    let blocking_catalog = InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(0),
        )],
    };
    let harmless_catalog = InvariantCatalog {
        registrations: vec![InvariantRegistration::snapshot_publication_blocking(
            InvariantRule::MaxSnapshotEntities(99),
        )],
    };
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .invariant_catalog(blocking_catalog)
        .invariant_catalog(harmless_catalog)
        .workspace("consumer-kit.test-backend.invariant-merge")
        .expect("in-memory test runtime with merged invariant catalogs should build");

    let error = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-denied")
                .aspect("title.value", "Denied by merged invariant")
        })
        .expect_err("first invariant catalog must not be overwritten by the second");

    assert_workspace_error_kind(error, ForgeQueryWorkspaceErrorKind::Unclassified);
}

fn task_workspace() -> crate::runtime::ForgeQueryWorkspace {
    let schema = task_schema();
    in_memory_test_runtime()
        .with_schema(schema)
        .workspace("consumer-kit.test-backend")
        .expect("in-memory test runtime should build")
}

fn task_schema() -> ForgeQueryTestBackendSchema {
    ForgeQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("title.value", "title.value")
        .expect("title aspect")
}

fn assert_workspace_error_kind(error: ForgeQueryRuntimeError, kind: ForgeQueryWorkspaceErrorKind) {
    match error {
        ForgeQueryRuntimeError::Workspace(workspace_error) => {
            assert_eq!(workspace_error.kind(), kind, "{workspace_error}");
        }
        other => panic!("expected workspace error `{kind:?}`, got {other:?}"),
    }
}
