use super::*;
use crate::runtime::ForgeQueryAdmittedAspectValue;
use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};

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
            ForgeQueryAdmittedAspectValue::new(touch("identity.id"), text("task-1"))
                .expect("identity aspect"),
            ForgeQueryAdmittedAspectValue::new(touch("title.value"), text("First task"))
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
            ForgeQueryAdmittedAspectValue::new(touch("identity.id"), text("task-1"))
                .expect("identity aspect"),
            ForgeQueryAdmittedAspectValue::new(touch("title.value"), text("First task"))
                .expect("title aspect"),
        ])
        .expect("seed insert should succeed");
    let entity_identity = insert.deltas[0].entity_identity.clone();

    let update = workspace
        .update_aspects(
            entity_identity.clone(),
            vec![
                ForgeQueryAdmittedAspectValue::new(touch("title.value"), text("Updated task"))
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
        .insert_aspects(vec![ForgeQueryAdmittedAspectValue::new(
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

#[test]
fn memory_workspace_aspect_rejects_mismatched_native_field_path() {
    let denial = crate::memory_workspace::ForgeQueryAspect::new(
        touch("title.value"),
        field_path("identity.id"),
    )
    .expect_err("mismatched aspect touch and native field path should be denied");

    assert!(denial
        .message()
        .contains("must use native field path rooted at `title`"));
}

fn aspect(label: &str, native_field_path: &str) -> crate::memory_workspace::ForgeQueryAspect {
    crate::memory_workspace::ForgeQueryAspect::new(touch(label), field_path(native_field_path))
        .expect("test aspect should admit")
}

fn field_path(path: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.split('.')
            .map(FieldKey::new)
            .collect::<Option<Vec<_>>>()
            .expect("test field path segments should be valid"),
    )
    .expect("test field path should not be empty")
}

fn touch(touch_fixture: &str) -> crate::runtime::ForgeQueryAspectTouch {
    let mut segments = touch_fixture.split('.');
    let aspect_key = AspectKey::new(
        segments
            .next()
            .expect("test touch fixture should name an aspect"),
    )
    .expect("test aspect key should admit");
    let field_segments = segments
        .map(|field| FieldKey::new(field).expect("test field key should admit"))
        .collect::<Vec<_>>();
    if field_segments.is_empty() {
        crate::runtime::ForgeQueryAspectTouch::whole_aspect(aspect_key)
    } else {
        crate::runtime::ForgeQueryAspectTouch::aspect_field_path(
            aspect_key,
            CanonicalFieldPath::new(field_segments).expect("test field path should admit"),
        )
    }
}

fn text(value: impl Into<String>) -> AspectValue {
    crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(value)
}
