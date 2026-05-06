use super::*;
use crate::runtime::ForgeQueryAspectValue;
use serde_json::json;

#[test]
fn memory_workspace_insert_aspects_tracks_changed_paths() {
    let mut workspace = ForgeQueryMemoryWorkspace::collection(
        "Task",
        [
            aspect("identity.id", "identity.id"),
            aspect("title.value", "title.value"),
        ],
    )
    .expect("memory workspace should build");

    let receipt = workspace
        .insert_aspects(vec![
            ForgeQueryAspectValue::new("identity.id", json!("task-1")).expect("identity aspect"),
            ForgeQueryAspectValue::new("title.value", json!("First task")).expect("title aspect"),
        ])
        .expect("insert should succeed");

    assert_eq!(receipt.deltas.len(), 1);
    assert_eq!(receipt.deltas[0].kind, ForgeQueryMutationKind::Created);
    assert_eq!(
        receipt.deltas[0].aspect_paths,
        ["identity.id", "title.value"]
    );
    assert_eq!(workspace.entities().len(), 1);
}

#[test]
fn memory_workspace_update_and_delete_preserve_entity_lifecycle() {
    let mut workspace = ForgeQueryMemoryWorkspace::collection(
        "Task",
        [
            aspect("identity.id", "identity.id"),
            aspect("title.value", "title.value"),
        ],
    )
    .expect("memory workspace should build");

    let insert = workspace
        .insert(json!({
            "identity": { "id": "task-1" },
            "title": { "value": "First task" }
        }))
        .expect("seed insert should succeed");
    let entity_identity = insert.deltas[0].entity_identity.clone();

    let update = workspace
        .update_aspect(&entity_identity, "title.value", json!("Updated task"))
        .expect("update should succeed");
    assert_eq!(update.deltas[0].kind, ForgeQueryMutationKind::Updated);
    assert_eq!(update.deltas[0].aspect_paths, ["title.value"]);
    assert_eq!(
        workspace.entities()[0].payload["title"]["value"],
        json!("Updated task")
    );

    let delete = workspace
        .delete(&entity_identity)
        .expect("delete should succeed");
    assert_eq!(delete.deltas[0].kind, ForgeQueryMutationKind::Deleted);
    assert!(workspace.entities().is_empty());
}

fn aspect(label: &str, payload_path: &str) -> crate::memory_workspace::ForgeQueryAspect {
    crate::memory_workspace::ForgeQueryAspect::new(label, payload_path)
}
