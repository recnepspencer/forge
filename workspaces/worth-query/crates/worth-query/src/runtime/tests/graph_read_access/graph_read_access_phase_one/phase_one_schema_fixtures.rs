//! Frozen schema fixtures for the phase-one graph-read access proofs.

use crate::facade::foundation::{AspectName, FieldName, RelationName};
use crate::runtime::{QuerySchemaView, ScalarAspectType, SchemaFieldView, SchemaRelationView};

pub(super) fn manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-manager",
        [
            string_field("identity", "id"),
            string_field("profile", "display_name"),
        ],
        [relation("manager")],
    )
}

pub(super) fn frontier_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-frontier",
        [
            string_field("identity", "id"),
            string_field("profile", "display_name"),
        ],
        [relation("manager"), relation("mentor")],
    )
}

pub(super) fn wide_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-wide",
        [
            string_field("identity", "id"),
            string_field("profile", "display_name"),
            string_field("profile", "title"),
            string_field("profile", "department"),
        ],
        [relation("manager")],
    )
}

fn string_field(aspect: &str, field: &str) -> SchemaFieldView {
    SchemaFieldView::new(
        AspectName::new(aspect).expect("schema aspect literal must be valid"),
        FieldName::new(field).expect("schema field literal must be valid"),
        ScalarAspectType::String,
    )
}

fn relation(name: &str) -> SchemaRelationView {
    SchemaRelationView::new(
        RelationName::new(name).expect("schema relation literal must be valid"),
        2,
    )
}
