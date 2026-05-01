use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([
            ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "TaskEdge",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("edge.kind", "edge.kind"),
                    crate::memory_workspace::ForgeQueryAspect::new(
                        "edge.source_identity",
                        "edge.source_identity",
                    ),
                    crate::memory_workspace::ForgeQueryAspect::new(
                        "edge.target_identity",
                        "edge.target_identity",
                    ),
                ],
            ),
        ])
        .build()
        .expect("runtime should build")
}

#[test]
fn compose_graph_preserves_symbolic_resolution_and_mixed_edge_meaning() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("tasks-graph-composition-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-draft")
                    .aspect("title.value", "Draft task")
            })?;
            graph.insert_relation("TaskEdge", |edge| {
                edge.aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity("edge.target_identity", "task-existing")
            })?;
            Ok(())
        })
        .expect("graph composition should execute");
    let draft_identity = receipt.write_receipts()[0].deltas()[0]
        .entity_identity
        .clone();
    let edge_rows = workspace.read(&edges);

    assert_eq!(receipt.write_receipts().len(), 2);
    assert_eq!(edge_rows.len(), 1);
    assert_eq!(
        edge_rows[0].payload["edge"]["source_identity"].as_str(),
        Some(draft_identity.as_str())
    );
    assert_eq!(
        edge_rows[0].payload["edge"]["target_identity"].as_str(),
        Some("task-existing")
    );

    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "mixed_existing_and_symbolic_entity_identity_edges"));
}

#[test]
fn compose_graph_supports_symbolic_relation_followup_mutation() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-relation-followup")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-relation-followup-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-relation-followup-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-relation-followup-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("tasks-graph-composition-relation-followup-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-draft")
                    .aspect("title.value", "Draft task")
            })?;
            let edge = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |relation| {
                relation
                    .aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity("edge.target_identity", "task-existing")
            })?;
            graph.update_relation(&edge, |relation| relation.aspect("edge.kind", "blocks"))?;
            Ok(())
        })
        .expect("graph composition with relation followup should execute");
    let edge_rows = workspace.read(&edges);

    assert_eq!(receipt.write_receipts().len(), 3);
    assert_eq!(edge_rows.len(), 1);
    assert_eq!(
        edge_rows[0].payload["edge"]["kind"].as_str(),
        Some("blocks")
    );

    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "same_batch_symbolic_relation_followup_mutation"));
}

#[test]
fn compose_graph_denies_duplicate_symbol_declarations() {
    let mut workspace = task_runtime()
        .workspace("tasks.graph-composition-duplicate")
        .expect("task runtime should open a named workspace");

    let error = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-draft-one")
                    .aspect("title.value", "Draft one")
            })?;
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-draft-two")
                    .aspect("title.value", "Draft two")
            })?;
            Ok(())
        })
        .expect_err("duplicate graph symbols should deny");

    match error {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("declared more than once"));
        }
        other => panic!("expected workspace denial, got {other:?}"),
    }
}
