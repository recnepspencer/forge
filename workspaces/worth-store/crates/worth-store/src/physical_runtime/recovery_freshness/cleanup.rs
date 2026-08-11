use sha2::{Digest, Sha256};
use worth_store_physical_backend::AdmittedRecoveryFilesystemMedia;

use crate::physical_runtime::{
    PhysicalRecoveryCleanupFreshnessReadDenial, PhysicalRecoveryCleanupFreshnessReadOutcome,
    PhysicalRecoveryCoordination,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreRecoveryCleanupFreshnessSample {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    observed_published_generation: u64,
    cleanup_plan_identity: [u8; 32],
    sealed_publication_basis: [u8; 32],
    policy_identity: [u8; 32],
    selector_read_operation: worth_store_physical_backend::MediaOperationIdentity,
    work: crate::physical_runtime::PhysicalWorkIdentity,
    signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
}

pub struct StoreRecoveryCleanupFreshnessFailure {
    denial: StoreRecoveryCleanupFreshnessDenial,
    read: Option<PhysicalRecoveryCleanupFreshnessReadDenial>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreRecoveryCleanupFreshnessDenial {
    FreshnessMediaMismatch,
    CurrentSelectorRead,
}

pub(super) fn sample(
    authority: &super::PhysicalRecoveryFreshnessAuthority,
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    cleanup_plan_identity: [u8; 32],
    sealed_publication_basis: [u8; 32],
) -> Result<StoreRecoveryCleanupFreshnessSample, StoreRecoveryCleanupFreshnessFailure> {
    if !authority.matches_media_generation(media.media_generation()) {
        return Err(StoreRecoveryCleanupFreshnessFailure {
            denial: StoreRecoveryCleanupFreshnessDenial::FreshnessMediaMismatch,
            read: None,
        });
    }
    let completed = match coordination.read_cleanup_current_selector(media) {
        PhysicalRecoveryCleanupFreshnessReadOutcome::Completed(completed) => completed,
        PhysicalRecoveryCleanupFreshnessReadOutcome::Denied(denial) => {
            return Err(StoreRecoveryCleanupFreshnessFailure {
                denial: StoreRecoveryCleanupFreshnessDenial::CurrentSelectorRead,
                read: Some(denial),
            })
        }
    };
    let mut observed_published_generation = completed.selector().root_generation();
    #[cfg(feature = "certification-test-authority")]
    if coordination.take_certification_cleanup_generation_shift() {
        observed_published_generation = observed_published_generation
            .checked_add(1)
            .expect("certification generation shift remains representable");
    }
    Ok(StoreRecoveryCleanupFreshnessSample {
        store: media.store_identity(),
        observed_published_generation,
        cleanup_plan_identity,
        sealed_publication_basis,
        policy_identity: cleanup_policy_identity(media.store_identity()),
        selector_read_operation: completed.physical().physical().operation(),
        work: completed.work(),
        signal: completed.signal(),
    })
}

fn cleanup_policy_identity(
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.cleanup-freshness-policy.v1");
    digest.update(store.bytes());
    digest.finalize().into()
}

impl StoreRecoveryCleanupFreshnessSample {
    pub const fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }
    pub const fn observed_published_generation(&self) -> u64 {
        self.observed_published_generation
    }
    pub const fn cleanup_plan_identity(&self) -> [u8; 32] {
        self.cleanup_plan_identity
    }
    pub const fn sealed_publication_basis(&self) -> [u8; 32] {
        self.sealed_publication_basis
    }
    pub const fn policy_identity(&self) -> [u8; 32] {
        self.policy_identity
    }
    pub const fn selector_read_operation(
        &self,
    ) -> worth_store_physical_backend::MediaOperationIdentity {
        self.selector_read_operation
    }
    pub const fn work(&self) -> crate::physical_runtime::PhysicalWorkIdentity {
        self.work
    }
    pub const fn signal(&self) -> crate::physical_runtime::PhysicalSignalSettlementOutcome {
        self.signal
    }
}

impl StoreRecoveryCleanupFreshnessFailure {
    pub const fn denial(&self) -> StoreRecoveryCleanupFreshnessDenial {
        self.denial
    }
    pub const fn read(&self) -> Option<&PhysicalRecoveryCleanupFreshnessReadDenial> {
        self.read.as_ref()
    }
}
