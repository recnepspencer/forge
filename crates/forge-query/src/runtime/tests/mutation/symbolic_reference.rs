use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

#[test]
fn mixed_batch_symbolic_and_existing_targets_preserve_distinct_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.mixed-target-batch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.mixed-target-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-mixed-target-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-existing")
                .aspect("title.value", "Existing")
        })
        .expect("seed insert should execute");
    let existing_binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new(
                "authority:task-existing",
            )
            .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("existing binding should build")
    .in_target_collection("Task")
    .expect("existing binding collection should build");
    let symbolic = ForgeQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic reference collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft")
                })
                .update_symbolic(symbolic.clone(), |task| {
                    task.aspect("title.value", "Draft renamed")
                })
                .update_existing(existing_binding, |task| {
                    task.aspect("title.value", "Existing renamed")
                })
        })
        .expect("mixed target batch should execute");
    let inspection = workspace.inspect(&receipt).expect("batch should inspect");

    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_binding_count(),
        1
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        1
    );

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_target_reference_count(),
                1
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .symbolic_target_reference_evidence()
                    .expect("symbolic component should retain symbolic evidence")
                    .symbol()
                    .as_str(),
                "draft-task"
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .existing_truth_binding_evidence()
                    .expect("existing component should retain existing-target evidence")
                    .authoritative_identity()
                    .as_str(),
                "authority:task-existing"
            );
        }
        other => panic!("expected batch inspection, got {other:?}"),
    }
}

#[test]
fn symbolic_target_reference_denies_missing_same_batch_target() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.symbolic-denial")
        .expect("task runtime should open a named workspace");
    let symbolic = ForgeQuerySymbolicTargetReference::new("missing-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic reference collection should build");

    let error = workspace
        .batch(|batch| {
            batch.update_symbolic(symbolic, |task| task.aspect("title.value", "No target"))
        })
        .expect_err("missing same-batch symbolic target should deny");

    match error {
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected symbolic target denial, got {other:?}"),
    }
}

#[test]
fn preview_batch_symbolic_target_preserves_symbolic_evidence() {
    let runtime = stateful_bridge_task_runtime();
    let mut workspace = runtime
        .workspace("tasks.preview-symbolic")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-symbolic-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-symbolic-table")
        })
        .expect("live view should declare");

    let mut preview = workspace
        .preview(test_session_label("symbolic-preview"))
        .expect("preview should open");
    let receipt = preview
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft")
                })
                .update_symbolic(
                    ForgeQuerySymbolicTargetReference::new("draft-task")
                        .expect("symbolic reference should build")
                        .in_target_collection("Task")
                        .expect("symbolic reference collection should build"),
                    |task| task.aspect("title.value", "Draft preview renamed"),
                )
        })
        .expect("preview batch should execute");

    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        1
    );
    assert_eq!(
        receipt.write_receipts()[1]
            .symbolic_target_reference_evidence()
            .expect("preview component should retain symbolic evidence")
            .symbol()
            .as_str(),
        "draft-task"
    );
}

#[test]
fn symbolic_aspect_reference_requires_batch_context() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.symbolic-aspect-single")
        .expect("runtime should open a named workspace");
    let command = ForgeQueryAspectMutationBuilder::new()
        .aspect("edge.kind", "depends_on")
        .symbolic_entity_identity(
            "edge.source_identity",
            ForgeQuerySymbolicTargetReference::new("draft-task")
                .expect("symbolic reference should build"),
        )
        .aspect("edge.target_identity", "task-existing")
        .build_insert("TaskEdge")
        .expect("insert command should build");

    let error = workspace
        .write(command)
        .expect_err("symbolic aspect references must fail closed outside batch execution");

    match error {
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext
            );
        }
        other => panic!("expected symbolic aspect batch-context denial, got {other:?}"),
    }
}

#[test]
fn symbolic_aspect_reference_resolves_same_batch_created_entity_identity() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.symbolic-aspect-batch")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.symbolic-aspect-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-symbolic-aspect-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.symbolic-aspect-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("tasks-symbolic-aspect-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft task")
                })
                .insert("TaskEdge", |edge| {
                    edge.aspect("edge.kind", "depends_on")
                        .symbolic_entity_identity(
                            "edge.source_identity",
                            ForgeQuerySymbolicTargetReference::new("draft-task")
                                .expect("symbolic reference should build"),
                        )
                        .aspect("edge.target_identity", "task-existing")
                })
        })
        .expect("symbolic aspect batch should execute");

    let draft_identity = receipt.write_receipts()[0].deltas()[0]
        .entity_identity
        .clone();
    let edge_rows = workspace.read(&edges);
    assert_eq!(edge_rows.len(), 1);
    assert_eq!(
        edge_rows[0].external_row()["edge"]["source_identity"].as_str(),
        Some(draft_identity.evidence_identity().terminal_projection_for_reporting())
    );
    assert_eq!(
        edge_rows[0].external_row()["edge"]["target_identity"].as_str(),
        Some("task-existing")
    );
}
