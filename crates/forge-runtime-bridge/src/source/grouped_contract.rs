use std::sync::Arc;

use forge_foundational::facade::AspectValue;

use crate::snapshot::TruthSnapshotIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityBindingProof {
    aspect_key: Arc<str>,
}

impl IdentityBindingProof {
    pub(crate) fn new(aspect_key: impl Into<Arc<str>>) -> Self {
        Self {
            aspect_key: aspect_key.into(),
        }
    }

    pub fn aspect_key(&self) -> &str {
        self.aspect_key.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupingBindingProof {
    aspect_key: Arc<str>,
}

impl GroupingBindingProof {
    pub(crate) fn new(aspect_key: impl Into<Arc<str>>) -> Self {
        Self {
            aspect_key: aspect_key.into(),
        }
    }

    pub fn aspect_key(&self) -> &str {
        self.aspect_key.as_ref()
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
                source.identity_binding_aspect_key().to_string(),
            ),
            grouping_binding: GroupingBindingProof::new(
                source.grouping_binding_aspect_key().to_string(),
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
    fn identity_value(&self) -> &AspectValue;
    fn grouping_value(&self) -> &AspectValue;
}

pub trait GroupedProjectionSource {
    type Member: GroupedProjectionMemberSource;

    fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity;
    fn grouping_aspect(&self) -> &str;
    fn identity_binding_aspect_key(&self) -> &str;
    fn grouping_binding_aspect_key(&self) -> &str;
    fn members(&self) -> &[Self::Member];
}
