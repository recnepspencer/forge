use super::WorthUiHandleArenaIdentity;
use crate::runtime::WorthUiRuntimeHandleAllocationBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocationReceipt {
    basis_digest: u64,
    arena_identity: WorthUiHandleArenaIdentity,
}

impl WorthUiRuntimeHandleAllocationReceipt {
    pub(crate) fn from_basis(
        basis: &WorthUiRuntimeHandleAllocationBasis,
        arena_identity: WorthUiHandleArenaIdentity,
    ) -> Self {
        Self {
            basis_digest: basis.digest(),
            arena_identity,
        }
    }

    pub fn certifies_basis(self, basis: &WorthUiRuntimeHandleAllocationBasis) -> bool {
        self.basis_digest == basis.digest()
    }

    pub fn basis_digest(self) -> u64 {
        self.basis_digest
    }

    pub fn arena_identity(self) -> WorthUiHandleArenaIdentity {
        self.arena_identity
    }
}
