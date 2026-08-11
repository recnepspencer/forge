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

/// One Store-issued, consuming authorization for the exact cleanup plan that
/// was sampled against the current selector.
///
/// The authorization is deliberately not `Clone` or `Copy`. It is not enough
/// to authorize removal by itself: the Store cleanup command also binds it to
/// performed fresh-reopen evidence and independently verified checkpoint/WAL
/// facts before any physical effect is admitted.
pub struct PhysicalRecoveryCleanupAuthorization {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    media_generation: worth_store_physical_backend::PhysicalRecoveryMediaGeneration,
    session: [u8; 16],
    observed_published_generation: u64,
    cleanup_plan_identity: [u8; 32],
    sealed_publication_basis: [u8; 32],
    policy_identity: [u8; 32],
    wal: worth_store_wal::WalSegmentInspection,
}

/// Owner-sampled descriptive evidence plus the one consuming authorization
/// issued by that same sampling occurrence.
pub struct StoreRecoveryCleanupFreshnessAdmission {
    sample: StoreRecoveryCleanupFreshnessSample,
    authorization: PhysicalRecoveryCleanupAuthorization,
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
    wal: worth_store_wal::WalSegmentInspection,
) -> Result<StoreRecoveryCleanupFreshnessAdmission, StoreRecoveryCleanupFreshnessFailure> {
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
    let observed_published_generation = completed.selector().root_generation();
    #[cfg(feature = "certification-test-authority")]
    let observed_published_generation =
        if coordination.take_certification_cleanup_generation_shift() {
            observed_published_generation
                .checked_add(1)
                .expect("certification generation shift remains representable")
        } else {
            observed_published_generation
        };
    let sample = StoreRecoveryCleanupFreshnessSample {
        store: media.store_identity(),
        observed_published_generation,
        cleanup_plan_identity,
        sealed_publication_basis,
        policy_identity: cleanup_policy_identity(media.store_identity()),
        selector_read_operation: completed.physical().physical().operation(),
        work: completed.work(),
        signal: completed.signal(),
    };
    let authorization = PhysicalRecoveryCleanupAuthorization {
        store: media.store_identity(),
        media_generation: media.media_generation(),
        session: coordination.session_identity(),
        observed_published_generation,
        cleanup_plan_identity,
        sealed_publication_basis,
        policy_identity: sample.policy_identity,
        wal,
    };
    Ok(StoreRecoveryCleanupFreshnessAdmission {
        sample,
        authorization,
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

impl StoreRecoveryCleanupFreshnessAdmission {
    pub const fn sample(&self) -> &StoreRecoveryCleanupFreshnessSample {
        &self.sample
    }

    pub fn into_parts(
        self,
    ) -> (
        StoreRecoveryCleanupFreshnessSample,
        PhysicalRecoveryCleanupAuthorization,
    ) {
        (self.sample, self.authorization)
    }
}

impl PhysicalRecoveryCleanupAuthorization {
    pub(in crate::physical_runtime) fn matches(
        &self,
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        media_generation: worth_store_physical_backend::PhysicalRecoveryMediaGeneration,
        session: [u8; 16],
        published_generation: u64,
        publication_plan: [u8; 32],
        wal: worth_store_wal::WalSegmentInspection,
    ) -> bool {
        self.store == store
            && self.media_generation == media_generation
            && self.session == session
            && self.observed_published_generation == published_generation
            && self.sealed_publication_basis == publication_plan
            && self.cleanup_plan_identity != [0; 32]
            && self.policy_identity != [0; 32]
            && self.wal == wal
    }

    pub(in crate::physical_runtime) const fn cleanup_plan_identity(&self) -> [u8; 32] {
        self.cleanup_plan_identity
    }

    pub(in crate::physical_runtime) const fn published_generation(&self) -> u64 {
        self.observed_published_generation
    }

    pub(in crate::physical_runtime) const fn session(&self) -> [u8; 16] {
        self.session
    }

    pub(in crate::physical_runtime) const fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    pub(in crate::physical_runtime) const fn media_generation(
        &self,
    ) -> worth_store_physical_backend::PhysicalRecoveryMediaGeneration {
        self.media_generation
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
