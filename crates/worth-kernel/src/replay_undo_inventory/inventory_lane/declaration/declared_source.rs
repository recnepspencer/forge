use super::declared_input_role_set::ReplayUndoDeclaredInputRoleSet;
use super::declared_source_identity::ReplayUndoDeclaredSourceIdentity;
use super::declared_source_kind::ReplayUndoDeclaredSourceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoDeclaredSource {
    identity: ReplayUndoDeclaredSourceIdentity,
    source_path: String,
    source_kind: ReplayUndoDeclaredSourceKind,
    authority_roles: ReplayUndoDeclaredInputRoleSet,
    observability_roles: ReplayUndoDeclaredInputRoleSet,
}

impl ReplayUndoDeclaredSource {
    pub(crate) fn new(
        identity: ReplayUndoDeclaredSourceIdentity,
        source_path: impl Into<String>,
        source_kind: ReplayUndoDeclaredSourceKind,
        authority_roles: ReplayUndoDeclaredInputRoleSet,
        observability_roles: ReplayUndoDeclaredInputRoleSet,
    ) -> Self {
        Self {
            identity,
            source_path: source_path.into(),
            source_kind,
            authority_roles,
            observability_roles,
        }
    }

    pub const fn identity(&self) -> ReplayUndoDeclaredSourceIdentity {
        self.identity
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn source_kind(&self) -> ReplayUndoDeclaredSourceKind {
        self.source_kind
    }

    pub const fn authority_roles(&self) -> &ReplayUndoDeclaredInputRoleSet {
        &self.authority_roles
    }

    pub const fn observability_roles(&self) -> &ReplayUndoDeclaredInputRoleSet {
        &self.observability_roles
    }
}
