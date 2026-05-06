use super::super::super::support::*;
use crate::authoring::RelationName;
use crate::schema_view::{QuerySchemaView, SchemaRelationView};

pub(crate) fn read_runtime() -> ForgeQueryRuntime {
    bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ))
}

pub(crate) fn manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 1)],
    )
}

pub(crate) fn expanded_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-expanded",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 2)],
    )
}

pub(crate) fn frontier_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-frontier",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [
            SchemaRelationView::new("manager", 1),
            SchemaRelationView::new("mentor", 1),
        ],
    )
}

pub(crate) fn expanded_frontier_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-frontier-expanded",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [
            SchemaRelationView::new("manager", 2),
            SchemaRelationView::new("mentor", 2),
        ],
    )
}

pub(crate) fn searchable_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-searchable",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String)
                .text_predicate_queryable()
                .membership_predicate_queryable()
                .presence_predicate_queryable(),
        ],
        [SchemaRelationView::new("manager", 1)],
    )
}

pub(crate) fn searchable_expanded_frontier_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-searchable-frontier-expanded",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String)
                .text_predicate_queryable()
                .membership_predicate_queryable()
                .presence_predicate_queryable(),
        ],
        [
            SchemaRelationView::new("manager", 2),
            SchemaRelationView::new("mentor", 2),
        ],
    )
}

pub(crate) fn manager_relation_name() -> RelationName {
    RelationName::new("manager").expect("test relation name should validate")
}

pub(crate) fn mentor_relation_name() -> RelationName {
    RelationName::new("mentor").expect("test relation name should validate")
}
