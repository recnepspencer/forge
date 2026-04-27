use super::*;

pub(in crate::runtime::tests) fn task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

pub(in crate::runtime::tests) fn task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
        ],
        [],
    )
}

pub(in crate::runtime::tests) fn issue_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Issue", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("summary", "value").delivered_as("summary"))
        .order_by(DeclarativeProjectionField::new("summary", "value"))
}

pub(in crate::runtime::tests) fn issue_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-issue",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("summary", "value", SchemaFieldKind::String),
        ],
        [],
    )
}

pub(in crate::runtime::tests) fn grouped_task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::kanban_grouped("status"))
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .project(DeclarativeProjectionField::new("status", "value").delivered_as("status"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

pub(in crate::runtime::tests) fn grouped_task_table_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .project(DeclarativeProjectionField::new("status", "value").delivered_as("status"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

pub(in crate::runtime::tests) fn grouped_task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-grouped-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [],
    )
}
