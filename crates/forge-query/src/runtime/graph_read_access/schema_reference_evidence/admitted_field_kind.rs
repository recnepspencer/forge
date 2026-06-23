use crate::schema_view::SchemaFieldKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAdmittedSchemaFieldKind {
    String,
    Integer,
    Boolean,
    StructuredContent,
    WorkflowState,
}

impl ForgeQueryGraphReadAdmittedSchemaFieldKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::StructuredContent => "structured_content",
            Self::WorkflowState => "workflow_state",
        }
    }

    pub(crate) fn from_schema_field_kind(kind: &SchemaFieldKind) -> Self {
        match kind {
            SchemaFieldKind::String => Self::String,
            SchemaFieldKind::Integer => Self::Integer,
            SchemaFieldKind::Boolean => Self::Boolean,
            SchemaFieldKind::StructuredContent => Self::StructuredContent,
            SchemaFieldKind::WorkflowState => Self::WorkflowState,
        }
    }
}
