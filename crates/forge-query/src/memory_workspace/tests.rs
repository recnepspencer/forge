use super::*;
use crate::runtime::ForgeQueryAspectValue;
use forge_foundational::facade::AspectValue;

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
            ForgeQueryAspectValue::new(touch("identity.id"), text("task-1"))
                .expect("identity aspect"),
            ForgeQueryAspectValue::new(touch("title.value"), text("First task"))
                .expect("title aspect"),
        ])
        .expect("insert should succeed");

    assert_eq!(receipt.deltas.len(), 1);
    assert_eq!(receipt.deltas[0].kind, ForgeQueryMutationKind::Created);
    assert_eq!(
        receipt.deltas[0].admitted_touched_aspects(),
        &[touch("identity.id"), touch("title.value")]
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
        .insert_aspects(vec![
            ForgeQueryAspectValue::new(touch("identity.id"), text("task-1"))
                .expect("identity aspect"),
            ForgeQueryAspectValue::new(touch("title.value"), text("First task"))
                .expect("title aspect"),
        ])
        .expect("seed insert should succeed");
    let entity_identity = insert.deltas[0].entity_identity.clone();

    let update = workspace
        .update_aspects(
            entity_identity.clone(),
            vec![
                ForgeQueryAspectValue::new(touch("title.value"), text("Updated task"))
                    .expect("title aspect"),
            ],
        )
        .expect("update should succeed");
    assert_eq!(update.deltas[0].kind, ForgeQueryMutationKind::Updated);
    assert_eq!(
        update.deltas[0].admitted_touched_aspects(),
        &[touch("title.value")]
    );
    assert_eq!(
        workspace.entities()[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Updated task"))
    );
    assert_eq!(
        workspace.entities()[0].aspect_value(
            &forge_foundational::facade::AspectKey::new("title")
                .expect("title should be an aspect key"),
        ),
        Some(&text("Updated task"))
    );

    let delete = workspace
        .delete(entity_identity)
        .expect("delete should succeed");
    assert_eq!(delete.deltas[0].kind, ForgeQueryMutationKind::Deleted);
    assert!(workspace.entities().is_empty());
}

#[test]
fn memory_workspace_matches_declared_aspects_with_native_touches() {
    let mut workspace =
        ForgeQueryMemoryWorkspace::collection("Task", [aspect("title", "title.value")])
            .expect("memory workspace should build");

    workspace
        .insert_aspects(vec![ForgeQueryAspectValue::new(
            touch("title.value"),
            text("Native match"),
        )
        .expect("title field touch")])
        .expect("field touch should match whole-aspect declaration natively");

    assert_eq!(
        workspace.entities()[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Native match"))
    );
    assert_eq!(
        workspace.entities()[0].aspect_value(
            &forge_foundational::facade::AspectKey::new("title")
                .expect("title should be an aspect key"),
        ),
        Some(&text("Native match"))
    );
}

fn aspect(
    label: &str,
    external_projection_path: &str,
) -> crate::memory_workspace::ForgeQueryAspect {
    crate::memory_workspace::ForgeQueryAspect::new(
        touch(label),
        field_path(external_projection_path),
    )
}

fn field_path(path: &str) -> forge_foundational::facade::CanonicalFieldPath {
    forge_foundational::facade::CanonicalFieldPath::new(
        path.split('.')
            .map(forge_foundational::facade::FieldKey::new)
            .collect::<Option<Vec<_>>>()
            .expect("test field path segments should be valid"),
    )
    .expect("test field path should not be empty")
}

fn touch(aspect_path: &str) -> crate::runtime::ForgeQueryAspectTouch {
    crate::runtime::ForgeQueryAspectTouch::from_authoring_path(aspect_path.to_string())
        .expect("test aspect path should parse")
}

fn text(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}
