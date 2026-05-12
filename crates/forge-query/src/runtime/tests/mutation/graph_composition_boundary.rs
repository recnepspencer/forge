use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

#[test]
fn ordinary_batch_does_not_claim_graph_composition_evidence() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-ordinary-batch")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-ordinary-batch-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-ordinary-batch-tasks")
        })
        .expect("task live view should declare");

    let receipt = workspace
        .batch(|batch| {
            batch.insert("Task", |task| {
                task.aspect("identity.id", "task-ordinary")
                    .aspect("title.value", "Ordinary task")
            })
        })
        .expect("ordinary batch should execute");
    let inspection = workspace.inspect(&receipt).expect("receipt should inspect");

    assert!(receipt.graph_composition_program().is_none());
    assert!(receipt.graph_composition_lifecycle_outcomes().is_none());
    assert!(receipt.graph_composition_evidence().is_none());
    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert!(inspection.graph_composition_program().is_none());
            assert!(inspection.graph_composition_lifecycle_outcomes().is_none());
            assert!(inspection.graph_composition_evidence().is_none())
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn reconstructed_graph_composition_receipt_without_breadth_fails_closed() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-reconstructed-boundary")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view(
            "tasks.graph-composition-reconstructed-boundary-tasks",
            |q| {
                q.from("Task")
                    .select(["identity.id", "title.value"])
                    .order_by("title.value")
                    .schema_basis("tasks-graph-composition-reconstructed-boundary-tasks")
            },
        )
        .expect("task live view should declare");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view(
            "tasks.graph-composition-reconstructed-boundary-edges",
            |q| {
                q.from("TaskEdge")
                    .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                    .order_by("edge.kind")
                    .schema_basis("tasks-graph-composition-reconstructed-boundary-edges")
            },
        )
        .expect("edge live view should declare");

    let composed = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-reconstructed")
                    .aspect("title.value", "Reconstructed task")
            })?;
            graph.insert_relation("TaskEdge", |edge| {
                edge.aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity("edge.target_identity", "task-existing")
            })?;
            Ok(())
        })
        .expect("graph composition should execute");
    let reconstructed =
        ForgeQueryBatchWriteReceipt::from_write_receipts(composed.write_receipts().to_vec())
            .expect("reconstructing from component receipts should succeed");
    let inspection = workspace
        .inspect(&reconstructed)
        .expect("reconstructed receipt should inspect");

    assert!(reconstructed.graph_composition_program().is_none());
    assert!(reconstructed
        .graph_composition_lifecycle_outcomes()
        .is_none());
    assert!(reconstructed.graph_composition_evidence().is_none());
    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert!(inspection.graph_composition_program().is_none());
            assert!(inspection.graph_composition_lifecycle_outcomes().is_none());
            assert!(inspection.graph_composition_evidence().is_none())
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
