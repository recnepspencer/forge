use std::sync::Arc;

use serde_json::Value;

use crate::snapshot::TruthSnapshotIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityBindingProof {
    field_key: Arc<str>,
}

impl IdentityBindingProof {
    pub(crate) fn new(field_key: impl Into<Arc<str>>) -> Self {
        Self {
            field_key: field_key.into(),
        }
    }

    pub fn field_key(&self) -> &str {
        self.field_key.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupingBindingProof {
    field_key: Arc<str>,
}

impl GroupingBindingProof {
    pub(crate) fn new(field_key: impl Into<Arc<str>>) -> Self {
        Self {
            field_key: field_key.into(),
        }
    }

    pub fn field_key(&self) -> &str {
        self.field_key.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedProjectionContract {
    grouping_aspect: Arc<str>,
    identity_binding: IdentityBindingProof,
    grouping_binding: GroupingBindingProof,
}

impl GroupedProjectionContract {
    pub(crate) fn from_source(source: &impl GroupedProjectionSource) -> Self {
        Self {
            grouping_aspect: Arc::from(source.grouping_aspect().to_string()),
            identity_binding: IdentityBindingProof::new(
                source.identity_binding_field_key().to_string(),
            ),
            grouping_binding: GroupingBindingProof::new(
                source.grouping_binding_field_key().to_string(),
            ),
        }
    }

    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_ref()
    }

    pub fn identity_binding(&self) -> &IdentityBindingProof {
        &self.identity_binding
    }

    pub fn grouping_binding(&self) -> &GroupingBindingProof {
        &self.grouping_binding
    }
}

pub trait GroupedProjectionMemberSource {
    fn row_identity(&self) -> &str;
    fn identity_value(&self) -> &Value;
    fn grouping_value(&self) -> &Value;
}

pub trait GroupedProjectionSource {
    type Member: GroupedProjectionMemberSource;

    fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity;
    fn grouping_aspect(&self) -> &str;
    fn identity_binding_field_key(&self) -> &str;
    fn grouping_binding_field_key(&self) -> &str;
    fn members(&self) -> &[Self::Member];
}
