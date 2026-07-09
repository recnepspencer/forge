use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};

use crate::NativeStoreAspectFixture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S51SecurityScopeNativeHarnessFixture {
    current_authority: StoreCurrentAuthorityWitness,
    drifted_authority: StoreCurrentAuthorityWitness,
}

impl S51SecurityScopeNativeHarnessFixture {
    pub(crate) fn new() -> Self {
        let current = NativeStoreAspectFixture::scalar_string("s5.1-security-current");
        let drifted =
            NativeStoreAspectFixture::replay_boundary_scalar_string("s5.1-security-drifted");
        Self {
            current_authority: require_current_store_authority(current.boundary_fact().clone()),
            drifted_authority: require_current_store_authority(drifted.boundary_fact().clone()),
        }
    }

    pub(crate) const fn current_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.current_authority
    }

    pub(crate) const fn drifted_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.drifted_authority
    }
}
