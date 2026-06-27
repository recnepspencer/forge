use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredInputRoleSet, ReplayUndoDeclaredSourceIdentity, ReplayUndoDeclaredSourceKind,
};

use super::inventory_category::ReplayUndoInventoryCategory;
use super::inventory_disposition::ReplayUndoInventoryDisposition;
use super::inventory_owner::ReplayUndoInventoryOwner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoInventoryReportRow {
    source_identity: ReplayUndoDeclaredSourceIdentity,
    source_path: String,
    source_kind: ReplayUndoDeclaredSourceKind,
    owner: ReplayUndoInventoryOwner,
    category: ReplayUndoInventoryCategory,
    disposition: ReplayUndoInventoryDisposition,
    authority_roles: ReplayUndoDeclaredInputRoleSet,
    observability_roles: ReplayUndoDeclaredInputRoleSet,
    removal_trigger: Option<String>,
}

impl ReplayUndoInventoryReportRow {
    pub(crate) fn new(
        source_identity: ReplayUndoDeclaredSourceIdentity,
        source_path: impl Into<String>,
        source_kind: ReplayUndoDeclaredSourceKind,
        owner: ReplayUndoInventoryOwner,
        category: ReplayUndoInventoryCategory,
        disposition: ReplayUndoInventoryDisposition,
        authority_roles: ReplayUndoDeclaredInputRoleSet,
        observability_roles: ReplayUndoDeclaredInputRoleSet,
        removal_trigger: Option<&str>,
    ) -> Self {
        Self {
            source_identity,
            source_path: source_path.into(),
            source_kind,
            owner,
            category,
            disposition,
            authority_roles,
            observability_roles,
            removal_trigger: removal_trigger.map(str::to_string),
        }
    }

    pub const fn source_identity(&self) -> ReplayUndoDeclaredSourceIdentity {
        self.source_identity
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn source_kind(&self) -> ReplayUndoDeclaredSourceKind {
        self.source_kind
    }

    pub const fn owner(&self) -> ReplayUndoInventoryOwner {
        self.owner
    }

    pub const fn category(&self) -> ReplayUndoInventoryCategory {
        self.category
    }

    pub const fn disposition(&self) -> ReplayUndoInventoryDisposition {
        self.disposition
    }

    pub const fn authority_roles(&self) -> &ReplayUndoDeclaredInputRoleSet {
        &self.authority_roles
    }

    pub const fn observability_roles(&self) -> &ReplayUndoDeclaredInputRoleSet {
        &self.observability_roles
    }

    pub fn removal_trigger(&self) -> Option<&str> {
        self.removal_trigger.as_deref()
    }
}
