#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadRelationAuthority {
    schema_basis_digest: String,
    relation_name: String,
}

impl ForgeQueryGraphReadRelationAuthority {
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
pub struct ForgeQueryGraphReadPredicateFieldAuthority {
    schema_basis_digest: String,
    aspect: String,
    field: String,
    field_kind: String,
}

impl ForgeQueryGraphReadPredicateFieldAuthority {
    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn field_kind(&self) -> &str {
        &self.field_kind
    }

    pub(crate) fn new(
        schema_basis_digest: impl Into<String>,
        aspect: impl Into<String>,
        field: impl Into<String>,
        field_kind: impl Into<String>,
    ) -> Self {
        Self {
            schema_basis_digest: schema_basis_digest.into(),
            aspect: aspect.into(),
            field: field.into(),
            field_kind: field_kind.into(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "predicate_authority:{}:{}:{}:{}",
            self.schema_basis_digest, self.aspect, self.field, self.field_kind
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadOrderingFieldAuthority {
    schema_basis_digest: String,
    aspect: String,
    field: String,
    direction: String,
    field_kind: String,
}

impl ForgeQueryGraphReadOrderingFieldAuthority {
    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
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
        aspect: impl Into<String>,
        field: impl Into<String>,
        direction: impl Into<String>,
        field_kind: impl Into<String>,
    ) -> Self {
        Self {
            schema_basis_digest: schema_basis_digest.into(),
            aspect: aspect.into(),
            field: field.into(),
            direction: direction.into(),
            field_kind: field_kind.into(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ordering_authority:{}:{}:{}:{}:{}",
            self.schema_basis_digest, self.aspect, self.field, self.direction, self.field_kind
        )
    }
}
