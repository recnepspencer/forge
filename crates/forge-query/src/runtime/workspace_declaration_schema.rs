use crate::authoring::{AspectName, FieldName, RelationName};
use crate::runtime::ForgeQueryRuntimeError;
use crate::schema_view::{SchemaFieldKind, SchemaFieldView, SchemaRelationView};

use super::workspace_error;

pub(super) fn schema_field_view(
    aspect: String,
    field: String,
    kind: SchemaFieldKind,
) -> Result<SchemaFieldView, ForgeQueryRuntimeError> {
    Ok(SchemaFieldView::new(
        schema_aspect_name(aspect)?,
        schema_field_name(field)?,
        kind,
    ))
}

pub(super) fn schema_relation_view(
    relation: String,
    max_depth: u8,
) -> Result<SchemaRelationView, ForgeQueryRuntimeError> {
    Ok(SchemaRelationView::new(
        schema_relation_name(relation)?,
        max_depth,
    ))
}

fn schema_aspect_name(value: String) -> Result<AspectName, ForgeQueryRuntimeError> {
    AspectName::new(value).map_err(|error| workspace_error(format!("{error:?}")))
}

fn schema_field_name(value: String) -> Result<FieldName, ForgeQueryRuntimeError> {
    FieldName::new(value).map_err(|error| workspace_error(format!("{error:?}")))
}

fn schema_relation_name(value: String) -> Result<RelationName, ForgeQueryRuntimeError> {
    RelationName::new(value).map_err(|error| workspace_error(format!("{error:?}")))
}
