use super::*;
use crate::declarative_live::{DeclarativeLiveViewShape, DeclarativeProjectionField};
use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
use crate::view_shape_live::ViewShapePatchFamily;
use serde_json::json;

#[test]
fn memory_app_routes_mutations_through_declared_live_views() {
    let mut app = ForgeQueryMemoryApp::new([ForgeQueryCollection::new(
        "Task",
        [
            ForgeQueryAspect::new("identity.id", "identity.id"),
            ForgeQueryAspect::new("title.value", "title.value"),
        ],
    )])
    .expect("memory app should build");
    app.declare_live_view(
        "tasks.table",
        crate::declarative_live::DeclarativeLiveQueryRequest::new(
            "Task",
            DeclarativeLiveViewShape::table(),
        )
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .order_by(DeclarativeProjectionField::new("title", "value")),
        QuerySchemaView::new(
            "todo-task",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            ],
            [],
        ),
    )
    .expect("live view should declare");

    let insert = app
        .insert(
            "Task",
            json!({
                "identity": { "id": "task-1" },
                "title": { "value": "First task" }
            }),
        )
        .expect("insert should execute");

    let patches = app.drain_live_patches("tasks.table");
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].entity_identity, insert.deltas[0].entity_identity);
    assert_eq!(patches[0].mutation_kind, ForgeQueryMutationKind::Created);
    assert_eq!(
        patches[0].envelope.patch_family(),
        Some(ViewShapePatchFamily::TableRowPatch)
    );
    assert_eq!(app.live_entities("tasks.table").len(), 1);
}

#[test]
fn memory_app_declares_grouped_live_view_with_internal_baseline() {
    let mut app = ForgeQueryMemoryApp::new([ForgeQueryCollection::new(
        "Task",
        [
            ForgeQueryAspect::new("identity.id", "identity.id"),
            ForgeQueryAspect::new("title.value", "title.value"),
            ForgeQueryAspect::new("status.value", "status.value"),
        ],
    )])
    .expect("memory app should build");
    app.declare_live_view(
        "tasks.seed-table",
        crate::declarative_live::DeclarativeLiveQueryRequest::new(
            "Task",
            DeclarativeLiveViewShape::table(),
        )
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("status", "value").delivered_as("status.value"))
        .order_by(DeclarativeProjectionField::new("status", "value")),
        grouped_task_schema(),
    )
    .expect("seed table live view should declare");
    app.insert(
        "Task",
        json!({
            "identity": { "id": "task-1" },
            "title": { "value": "First task" },
            "status": { "value": "todo" }
        }),
    )
    .expect("seed insert should execute");
    app.insert(
        "Task",
        json!({
            "identity": { "id": "task-2" },
            "title": { "value": "Second task" },
            "status": { "value": "doing" }
        }),
    )
    .expect("second seed insert should execute");

    app.declare_live_view(
        "tasks.kanban",
        crate::declarative_live::DeclarativeLiveQueryRequest::new(
            "Task",
            DeclarativeLiveViewShape::kanban_grouped("status"),
        )
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title.value"))
        .project(DeclarativeProjectionField::new("status", "value").delivered_as("status.value")),
        grouped_task_schema(),
    )
    .expect("kanban grouped live view should declare");

    let live_entities = app.live_entities("tasks.kanban");
    assert_eq!(live_entities.len(), 2);
}

fn grouped_task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "grouped-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [],
    )
}
