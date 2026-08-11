use worth_proof::AuthorityWitness;
use worth_store_physical_backend::PhysicalRecoveryMediaGeneration;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

worth_proof::authority_marker!(pub PhysicalRecoveryConstructionMarker);

/// Store-owned, move-only authority for constructing one recovered runtime.
pub struct PhysicalRecoveryConstructionAuthority {
    _witness: AuthorityWitness<PhysicalRecoveryConstructionMarker>,
    store: StableStoreIdentity,
    media: PhysicalRecoveryMediaGeneration,
    session: [u8; 16],
}

impl PhysicalRecoveryConstructionAuthority {
    pub(in crate::physical_runtime) fn issue(
        store: StableStoreIdentity,
        media: PhysicalRecoveryMediaGeneration,
        session: [u8; 16],
    ) -> Self {
        Self {
            _witness: PhysicalRecoveryConstructionMarker::witness(),
            store,
            media,
            session,
        }
    }

    pub(super) fn matches(
        &self,
        store: StableStoreIdentity,
        media: PhysicalRecoveryMediaGeneration,
        session: [u8; 16],
    ) -> bool {
        self.store == store && self.media == media && self.session == session
    }
}
