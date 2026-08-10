use crate::authoring::AspectFieldKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareValue {
    source: AspectFieldKey,
    value: String,
}

impl DeclarativeBranchCompareValue {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            source: AspectFieldKey::from_authoring_parts(aspect, field).expect(
                "declarative branch compare values require non-empty aspect and field names",
            ),
            value: value.into(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareInputRow {
    identity: String,
    label: String,
    values: Vec<DeclarativeBranchCompareValue>,
}

impl DeclarativeBranchCompareInputRow {
    pub fn new(
        identity: impl Into<String>,
        label: impl Into<String>,
        values: impl IntoIterator<Item = DeclarativeBranchCompareValue>,
    ) -> Self {
        Self {
            identity: identity.into(),
            label: label.into(),
            values: values.into_iter().collect(),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn values(&self) -> &[DeclarativeBranchCompareValue] {
        &self.values
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeBranchCompareChangeFamily {
    Added,
    Removed,
    Modified,
}

impl DeclarativeBranchCompareChangeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeBranchCompareIdentityClass {
    AuthoritativeIdentity,
    BranchLocalAddition,
    BranchLocalRemoval,
}

impl DeclarativeBranchCompareIdentityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeIdentity => "authoritative_identity",
            Self::BranchLocalAddition => "branch_local_addition",
            Self::BranchLocalRemoval => "branch_local_removal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareFieldDelta {
    source: AspectFieldKey,
    left_value: Option<String>,
    right_value: Option<String>,
    family: DeclarativeBranchCompareChangeFamily,
}

impl DeclarativeBranchCompareFieldDelta {
    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn left_value(&self) -> Option<&str> {
        self.left_value.as_deref()
    }

    pub fn right_value(&self) -> Option<&str> {
        self.right_value.as_deref()
    }

    pub fn family(&self) -> &DeclarativeBranchCompareChangeFamily {
        &self.family
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareRow {
    left_identity: Option<String>,
    right_identity: Option<String>,
    label: String,
    identity_class: DeclarativeBranchCompareIdentityClass,
    field_deltas: Vec<DeclarativeBranchCompareFieldDelta>,
}

impl DeclarativeBranchCompareRow {
    pub fn left_identity(&self) -> Option<&str> {
        self.left_identity.as_deref()
    }

    pub fn right_identity(&self) -> Option<&str> {
        self.right_identity.as_deref()
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn identity_class(&self) -> &DeclarativeBranchCompareIdentityClass {
        &self.identity_class
    }

    pub fn field_deltas(&self) -> &[DeclarativeBranchCompareFieldDelta] {
        &self.field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareArtifact {
    left_live_view_digest: String,
    right_live_view_digest: String,
    left_basis_digest: String,
    right_basis_digest: String,
    query_digest: String,
    result_digest: String,
    rows: Vec<DeclarativeBranchCompareRow>,
}

impl DeclarativeBranchCompareArtifact {
    pub fn left_live_view_digest(&self) -> &str {
        &self.left_live_view_digest
    }

    pub fn right_live_view_digest(&self) -> &str {
        &self.right_live_view_digest
    }

    pub fn left_basis_digest(&self) -> &str {
        &self.left_basis_digest
    }

    pub fn right_basis_digest(&self) -> &str {
        &self.right_basis_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn rows(&self) -> &[DeclarativeBranchCompareRow] {
        &self.rows
    }
}
