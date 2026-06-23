use crate::runtime::ForgeQueryReadGraph;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind {
    MissingFieldInFrozenSchemaView,
}

impl ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingFieldInFrozenSchemaView => "missing_field_in_frozen_schema_view",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadSchemaReferenceAdmissionError {
    kind: ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind,
    read_graph_digest: String,
    aspect: String,
    field: String,
}

impl ForgeQueryGraphReadSchemaReferenceAdmissionError {
    pub fn kind(&self) -> &ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind {
        &self.kind
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub(crate) fn missing_field(
        read_graph: &ForgeQueryReadGraph,
        aspect: &str,
        field: &str,
    ) -> Self {
        Self {
            kind:
                ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind::MissingFieldInFrozenSchemaView,
            read_graph_digest: read_graph.digest().to_string(),
            aspect: aspect.to_string(),
            field: field.to_string(),
        }
    }
}
