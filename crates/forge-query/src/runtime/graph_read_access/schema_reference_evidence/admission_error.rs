use crate::runtime::ForgeQueryReadGraph;
use forge_foundational::facade::{AspectKey, FieldKey};

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
    aspect: AspectKey,
    field: FieldKey,
}

impl ForgeQueryGraphReadSchemaReferenceAdmissionError {
    pub fn kind(&self) -> &ForgeQueryGraphReadSchemaReferenceAdmissionErrorKind {
        &self.kind
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
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
            aspect: AspectKey::new(aspect).expect(
                "schema reference admission error aspect must be a foundational aspect key",
            ),
            field: FieldKey::new(field)
                .expect("schema reference admission error field must be a foundational field key"),
        }
    }
}
