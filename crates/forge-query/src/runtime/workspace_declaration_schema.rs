use crate::authoring::RelationName;
use crate::runtime::ForgeQueryRuntimeError;
use crate::schema_view::SchemaRelationView;

use super::workspace_error;

pub(super) fn schema_relation_view(
    relation: String,
    max_depth: u8,
) -> Result<SchemaRelationView, ForgeQueryRuntimeError> {
    Ok(SchemaRelationView::new(
        schema_relation_name(relation)?,
        max_depth,
    ))
}

fn schema_relation_name(value: String) -> Result<RelationName, ForgeQueryRuntimeError> {
    RelationName::new(value).map_err(|error| workspace_error(format!("{error:?}")))
}
