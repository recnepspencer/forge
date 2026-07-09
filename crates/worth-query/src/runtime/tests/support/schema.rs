use super::*;

pub(in crate::runtime::tests) fn task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(
            DeclarativeProjectionField::from_authoring_parts("identity", "id")
                .delivered_as("identity.id"),
        )
        .project(
            DeclarativeProjectionField::from_authoring_parts("title", "value")
                .delivered_as("title"),
        )
        .order_by(DeclarativeProjectionField::from_authoring_parts(
            "title", "value",
        ))
}

pub(in crate::runtime::tests) fn task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-task",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("title")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

pub(in crate::runtime::tests) fn issue_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Issue", DeclarativeLiveViewShape::table())
        .project(
            DeclarativeProjectionField::from_authoring_parts("identity", "id")
                .delivered_as("identity.id"),
        )
        .project(
            DeclarativeProjectionField::from_authoring_parts("summary", "value")
                .delivered_as("summary"),
        )
        .order_by(DeclarativeProjectionField::from_authoring_parts(
            "summary", "value",
        ))
}

pub(in crate::runtime::tests) fn issue_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-issue",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("summary")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

pub(in crate::runtime::tests) fn grouped_task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new(
        "Task",
        DeclarativeLiveViewShape::kanban_grouped(aspect_key("status")),
    )
    .project(
        DeclarativeProjectionField::from_authoring_parts("identity", "id")
            .delivered_as("identity.id"),
    )
    .project(
        DeclarativeProjectionField::from_authoring_parts("title", "value").delivered_as("title"),
    )
    .project(
        DeclarativeProjectionField::from_authoring_parts("status", "value").delivered_as("status"),
    )
    .order_by(DeclarativeProjectionField::from_authoring_parts(
        "title", "value",
    ))
}

pub(in crate::runtime::tests) fn grouped_task_table_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(
            DeclarativeProjectionField::from_authoring_parts("identity", "id")
                .delivered_as("identity.id"),
        )
        .project(
            DeclarativeProjectionField::from_authoring_parts("title", "value")
                .delivered_as("title"),
        )
        .project(
            DeclarativeProjectionField::from_authoring_parts("status", "value")
                .delivered_as("status"),
        )
        .order_by(DeclarativeProjectionField::from_authoring_parts(
            "title", "value",
        ))
}

pub(in crate::runtime::tests) fn grouped_task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-grouped-task",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("title")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn aspect_key(value: &str) -> worth_foundational::facade::AspectKey {
    worth_foundational::facade::AspectKey::new(value)
        .expect("runtime test grouped aspect must be foundational")
}
