use std::sync::Arc;

use crate::history::data::BranchId;

use super::reference::{RelationalBranchReferenceCell, RelationalBranchReferenceMutableState};

/// Stable, branch-local handle consumed by the publication port.
///
/// Clones address the same live reference state and coordination cell. Draft
/// branch cells remain detached until this handle installs their prepared
/// state after an exact-reference comparison succeeds.
#[derive(Clone, Debug)]
pub(crate) struct RelationalBranchPublicationCell {
    identity: super::RelationalBranchIdentity,
    state: Arc<std::sync::Mutex<RelationalBranchReferenceMutableState>>,
    basis_registry: super::RelationalBranchBasisRegistry,
    coordination: Arc<super::coordination::RelationalBranchCoordinationCell>,
}

pub(crate) struct RelationalBranchPublicationStateGuard<'cell> {
    cell: &'cell RelationalBranchPublicationCell,
    state: std::sync::MutexGuard<'cell, RelationalBranchReferenceMutableState>,
}

impl RelationalBranchReferenceCell {
    pub(crate) fn publication_cell(&self) -> RelationalBranchPublicationCell {
        RelationalBranchPublicationCell {
            identity: self.identity.clone(),
            state: Arc::clone(&self.state),
            basis_registry: self.basis_registry.clone(),
            coordination: Arc::clone(&self.coordination),
        }
    }
}

impl RelationalBranchPublicationCell {
    pub(crate) fn runtime_instance_id(&self) -> u64 {
        self.identity.runtime_instance_id()
    }

    pub(crate) fn branch_id(&self) -> &BranchId {
        self.identity.branch_id()
    }

    pub(crate) fn coordination(
        &self,
    ) -> &Arc<super::coordination::RelationalBranchCoordinationCell> {
        &self.coordination
    }

    pub(crate) fn enter_state(&self) -> RelationalBranchPublicationStateGuard<'_> {
        RelationalBranchPublicationStateGuard {
            cell: self,
            state: self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
        }
    }

    pub(crate) fn currently_selects_root(
        &self,
        expected: &Arc<super::RelationalBranchRoot>,
    ) -> bool {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .currently_selects_root(expected)
    }

    pub(crate) fn register_basis(
        &self,
        basis: super::AdmittedRelationalBranchBasis,
    ) -> Result<super::AdmittedRelationalBranchBasis, super::RelationalBranchBasisDenial> {
        self.basis_registry.register(basis)
    }
}

impl RelationalBranchPublicationStateGuard<'_> {
    pub(crate) fn snapshot_cell(&self) -> RelationalBranchReferenceCell {
        RelationalBranchReferenceCell {
            identity: self.cell.identity.clone(),
            state: Arc::new(std::sync::Mutex::new(self.state.clone())),
            basis_registry: self.cell.basis_registry.clone(),
            coordination: Arc::clone(&self.cell.coordination),
        }
    }

    pub(crate) fn replace_with(&mut self, prepared: RelationalBranchReferenceMutableState) {
        *self.state = prepared;
    }
}
