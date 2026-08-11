use worth_store_physical_backend::AdmittedRecoveryFilesystemMedia;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, DurablePhysicalRootManifest,
};

use crate::physical_runtime::{CompletedPhysicalRecoveryFreshReopen, RuntimeIdentity};

pub struct RecoveredPhysicalRuntimeCore {
    pub(super) store: StableStoreIdentity,
    pub(super) runtime: RuntimeIdentity,
    pub(super) recovery_runtime: RuntimeIdentity,
    pub(super) root: DurablePhysicalRootManifest,
    pub(super) media: AdmittedRecoveryFilesystemMedia,
    pub(super) reopen: CompletedPhysicalRecoveryFreshReopen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveredPhysicalRuntimeConstructionDenial {
    BindingMismatch,
    ConstructionAuthorityMismatch,
    CoordinationNotQuiescent,
    RuntimeIdentityUnavailable,
}

impl RecoveredPhysicalRuntimeCore {
    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }
    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.runtime
    }
    pub const fn recovery_runtime_identity(&self) -> RuntimeIdentity {
        self.recovery_runtime
    }
    pub const fn root(&self) -> &DurablePhysicalRootManifest {
        &self.root
    }
    pub const fn reopen(&self) -> &CompletedPhysicalRecoveryFreshReopen {
        &self.reopen
    }
    pub fn recovery_effect_count(&self) -> u64 {
        self.media.recovery_effect_count()
    }
    pub const fn backend_profile(
        &self,
    ) -> &worth_store_physical_backend::QualifiedPhysicalBackendProfile {
        self.media.backend_profile()
    }
    pub const fn media_generation(
        &self,
    ) -> worth_store_physical_backend::PhysicalRecoveryMediaGeneration {
        self.media.media_generation()
    }
}
