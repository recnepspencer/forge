use crate::identity::hash_parts;
use worth_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedProjectionIdentity(String);

impl AuthorizedProjectionIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyFieldInfluenceSet {
    digest: String,
    field_reference_count: usize,
}

impl PolicyFieldInfluenceSet {
    pub(crate) fn new(parts: &[String], field_reference_count: usize) -> Self {
        Self {
            digest: hash_parts(parts),
            field_reference_count,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn field_reference_count(&self) -> usize {
        self.field_reference_count
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AuthorizedProjectionFieldPath {
    aspect_key: AspectKey,
    field_key: FieldKey,
    terminal_projection: String,
}

impl AuthorizedProjectionFieldPath {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field_key
    }

    pub(crate) fn terminal_projection_for_boundary(&self) -> &str {
        &self.terminal_projection
    }

    pub fn from_native_keys(aspect_key: AspectKey, field_key: FieldKey) -> Self {
        Self {
            terminal_projection: format!("{}.{}", aspect_key.as_str(), field_key.as_str()),
            aspect_key,
            field_key,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskedProjectionArtifact {
    masked_fields: Vec<AuthorizedProjectionFieldPath>,
    non_disclosing_fields: Vec<AuthorizedProjectionFieldPath>,
    digest: String,
}

impl MaskedProjectionArtifact {
    pub(crate) fn new(
        masked_fields: Vec<AuthorizedProjectionFieldPath>,
        non_disclosing_fields: Vec<AuthorizedProjectionFieldPath>,
    ) -> Self {
        let mut parts = vec!["masked_projection".to_string()];
        parts.extend(
            masked_fields
                .iter()
                .map(|field| format!("masked:{}", field.terminal_projection_for_boundary())),
        );
        parts.extend(non_disclosing_fields.iter().map(|field| {
            format!(
                "non_disclosing:{}",
                field.terminal_projection_for_boundary()
            )
        }));
        Self {
            masked_fields,
            non_disclosing_fields,
            digest: hash_parts(&parts),
        }
    }

    pub fn masked_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.masked_fields
    }

    pub fn non_disclosing_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.non_disclosing_fields
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
