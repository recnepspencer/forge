use forge_store_recovery_physics::RecoveryCompletion;

#[derive(Debug, Clone, Copy)]
pub struct PhysicalIsolationEntryRequest<'a> {
    recovery_completion: &'a RecoveryCompletion,
    store_authority_identity: forge_store_authority::StoreCurrentAuthorityIdentity,
}

impl<'a> PhysicalIsolationEntryRequest<'a> {
    pub fn from_recovery_completion(recovery_completion: &'a RecoveryCompletion) -> Self {
        Self::for_store(
            recovery_completion,
            &forge_store_physical_format::PhysicalStoreIdentity::physical_format_default(),
        )
    }

    pub fn for_store(
        recovery_completion: &'a RecoveryCompletion,
        store_identity: &forge_store_physical_format::PhysicalStoreIdentity,
    ) -> Self {
        Self {
            recovery_completion,
            store_authority_identity: store_identity.authority_identity(),
        }
    }

    pub const fn recovery_completion(&self) -> &'a RecoveryCompletion {
        self.recovery_completion
    }

    pub const fn store_authority_identity(
        &self,
    ) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        self.store_authority_identity
    }
}
