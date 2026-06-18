use forge_query::facade::consumer_kit::{
    compare_test_backend_write_receipts, in_memory_test_runtime, ForgeQueryTestBackendSchema,
};
use forge_query::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryWorkspaceErrorKind, ForgeQueryWriteReceipt,
};
use serde_json::Value;

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn in_memory_test_backend_facade_builds_a_real_workspace() {
    let schema = ForgeQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should be valid")
        .aspect("title.value", "title.value")
        .expect("title aspect should be valid");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .invariant_catalog(InvariantCatalog::default())
        .workspace("consumer-kit.facade.test")
        .expect("facade should build an in-memory test workspace");
    let tasks = workspace
        .live_view::<Value>("consumer-kit.facade.tasks", |view| {
            view.from("Task").select(["identity.id", "title.value"])
        })
        .expect("facade workspace should declare a live view");

    workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Facade proof")
        })
        .expect("facade workspace should write through the real runtime");

    assert_eq!(workspace.read(&tasks).len(), 1);
}

#[test]
fn in_memory_test_backend_facade_admits_preview_and_invariant_denial() {
    let schema = task_schema();
    let mut preview_workspace = in_memory_test_runtime()
        .with_schema(schema.clone())
        .workspace("consumer-kit.facade.preview")
        .expect("facade should build preview workspace");
    let mut preview = preview_workspace
        .preview(
            forge_query::facade::ForgeQuerySessionLabel::scoped_strs(
                "consumer-kit-facade",
                ["preview"],
            )
            .expect("preview label"),
        )
        .expect("facade should admit preview");
    preview
        .insert("Task", |task| {
            task.aspect("identity.id", "task-preview")
                .aspect("title.value", "Preview facade proof")
        })
        .expect("facade preview should stage schema-backed write");
    assert_eq!(preview.discard().authoritative_residue_count(), 0);

    let mut invariant_workspace = in_memory_test_runtime()
        .with_schema(schema)
        .invariant_catalog(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::MaxMergedIntents(0),
            )],
        })
        .workspace("consumer-kit.facade.invariant-denial")
        .expect("facade should build invariant workspace");
    let error = invariant_workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-denied")
                .aspect("title.value", "Denied")
        })
        .expect_err("facade invariant should deny through runtime write path");

    assert_workspace_error_kind(error, ForgeQueryWorkspaceErrorKind::Unclassified);
}

#[test]
fn in_memory_test_backend_matches_bridge_harness_for_covered_live_write_path() {
    let schema = task_schema();
    let mut in_memory_workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace("consumer-kit.facade.equivalence.in-memory")
        .expect("in-memory facade should build a workspace");
    let bridge_harness = PublicBridgeRuntimeHarness::new();
    let bridge_runtime = bridge_harness.bridge_backed_runtime();
    let mut bridge_workspace = bridge_runtime
        .workspace("consumer-kit.facade.equivalence.bridge")
        .expect("bridge harness should build a workspace");
    let in_memory_tasks = in_memory_workspace
        .live_view::<Value>("consumer-kit.facade.equivalence.in-memory.tasks", |view| {
            view.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
        })
        .expect("in-memory facade should declare live view");
    let bridge_tasks = bridge_workspace
        .live_view::<Value>("consumer-kit.facade.equivalence.bridge.tasks", |view| {
            view.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
        })
        .expect("bridge harness should declare live view");

    let in_memory_receipt = in_memory_workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-equivalence")
                .aspect("title.value", "Equivalent task")
        })
        .expect("in-memory write should execute through runtime");
    let bridge_receipt = bridge_workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-equivalence")
                .aspect("title.value", "Equivalent task")
        })
        .expect("bridge write should execute through runtime");
    let in_memory_identity = in_memory_receipt
        .target_entity_identity()
        .expect("in-memory insert identity")
        .clone();
    let bridge_identity = bridge_receipt
        .target_entity_identity()
        .expect("bridge insert identity")
        .clone();

    let in_memory_rows = in_memory_workspace
        .read(&in_memory_tasks)
        .into_iter()
        .map(|row| row.external_row().clone())
        .collect::<Vec<_>>();
    let bridge_rows = bridge_workspace
        .read(&bridge_tasks)
        .into_iter()
        .map(|row| row.external_row().clone())
        .collect::<Vec<_>>();

    assert_eq!(in_memory_rows, bridge_rows);
    assert_test_backend_receipt_equivalence(&in_memory_receipt, &bridge_receipt);

    let in_memory_update = in_memory_workspace
        .update(in_memory_identity.clone(), |task| {
            task.aspect("title.value", "Updated equivalently")
        })
        .expect("in-memory update should execute");
    let bridge_update = bridge_workspace
        .update(bridge_identity.clone(), |task| {
            task.aspect("title.value", "Updated equivalently")
        })
        .expect("bridge update should execute");
    assert_test_backend_receipt_equivalence(&in_memory_update, &bridge_update);
    assert_eq!(
        in_memory_workspace.read(&in_memory_tasks)[0].external_row(),
        bridge_workspace.read(&bridge_tasks)[0].external_row()
    );

    let in_memory_preview = in_memory_workspace
        .preview(
            forge_query::facade::ForgeQuerySessionLabel::scoped_strs(
                "consumer-kit-facade",
                ["in-memory-equivalence-preview"],
            )
            .expect("preview label"),
        )
        .expect("in-memory preview should admit");
    let bridge_preview = bridge_workspace
        .preview(
            forge_query::facade::ForgeQuerySessionLabel::scoped_strs(
                "consumer-kit-facade",
                ["bridge-equivalence-preview"],
            )
            .expect("preview label"),
        )
        .expect("bridge preview should admit");
    assert_eq!(
        in_memory_preview.basis_admission().effect_policy(),
        bridge_preview.basis_admission().effect_policy()
    );

    let in_memory_delete = in_memory_workspace
        .delete(in_memory_identity)
        .expect("in-memory delete should execute");
    let bridge_delete = bridge_workspace
        .delete(bridge_identity)
        .expect("bridge delete should execute");
    assert_test_backend_receipt_equivalence(&in_memory_delete, &bridge_delete);
    assert!(in_memory_workspace.read(&in_memory_tasks).is_empty());
    assert!(bridge_workspace.read(&bridge_tasks).is_empty());
}

fn assert_test_backend_receipt_equivalence(
    in_memory: &ForgeQueryWriteReceipt,
    bridge: &ForgeQueryWriteReceipt,
) {
    let report = compare_test_backend_write_receipts(in_memory, bridge);
    assert!(report.matched(), "{:?}", report.rows());
    assert_eq!(report.rows().len(), 8);
    assert!(!report
        .report_identity()
        .terminal_projection_for_reporting()
        .is_empty());
}

fn task_schema() -> ForgeQueryTestBackendSchema {
    ForgeQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should be valid")
        .aspect("title.value", "title.value")
        .expect("title aspect should be valid")
}

fn assert_workspace_error_kind(error: ForgeQueryRuntimeError, kind: ForgeQueryWorkspaceErrorKind) {
    match error {
        ForgeQueryRuntimeError::Workspace(workspace_error) => {
            assert_eq!(workspace_error.kind(), kind, "{workspace_error}");
        }
        other => panic!("expected workspace error `{kind:?}`, got {other:?}"),
    }
}
