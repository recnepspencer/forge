use worth_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadRelationAuthority {
    schema_basis_digest: String,
    relation_name: String,
}

impl WorthQueryGraphReadRelationAuthority {
    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    pub(crate) fn new(
        schema_basis_digest: impl Into<String>,
        relation_name: impl Into<String>,
    ) -> Self {
        Self {
            schema_basis_digest: schema_basis_digest.into(),
            relation_name: relation_name.into(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "relation_authority:{}:{}",
            self.schema_basis_digest, self.relation_name
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadPredicateFieldAuthority {
    schema_basis_digest: String,
    aspect: AspectKey,
    field: FieldKey,
    field_kind: String,
}

impl WorthQueryGraphReadPredicateFieldAuthority {
    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn field_kind(&self) -> &str {
        &self.field_kind
    }

    pub(crate) fn new(
        schema_basis_digest: impl Into<String>,
        aspect: AspectKey,
        field: FieldKey,
        field_kind: impl Into<String>,
    ) -> Self {
        Self {
            schema_basis_digest: schema_basis_digest.into(),
            aspect,
            field,
            field_kind: field_kind.into(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "predicate_authority:{}:{}:{}:{}",
            self.schema_basis_digest,
            self.aspect.as_str(),
            self.field.as_str(),
            self.field_kind
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadOrderingFieldAuthority {
    schema_basis_digest: String,
    aspect: AspectKey,
    field: FieldKey,
    direction: String,
    field_kind: String,
}

impl WorthQueryGraphReadOrderingFieldAuthority {
    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn direction(&self) -> &str {
        &self.direction
    }

    pub fn field_kind(&self) -> &str {
        &self.field_kind
    }

    pub(crate) fn new(
        schema_basis_digest: impl Into<String>,
        aspect: AspectKey,
        field: FieldKey,
        direction: impl Into<String>,
        field_kind: impl Into<String>,
    ) -> Self {
        Self {
            schema_basis_digest: schema_basis_digest.into(),
            aspect,
            field,
            direction: direction.into(),
            field_kind: field_kind.into(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ordering_authority:{}:{}:{}:{}:{}",
            self.schema_basis_digest,
            self.aspect.as_str(),
            self.field.as_str(),
            self.direction,
            self.field_kind
        )
    }
}
