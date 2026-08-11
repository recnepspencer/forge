use sha2::{Digest, Sha256};
use worth_store_physical_backend::AdmittedRecoveryFilesystemMedia;

use crate::physical_runtime::{
    CompletedPhysicalRecoveryFreshReopen, PhysicalRecoveryCleanupFreshnessReadDenial,
    PhysicalRecoveryCleanupFreshnessReadOutcome, PhysicalRecoveryCleanupRemovalCommand,
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

/// Owner-sampled descriptive evidence plus the only Store-admitted command
/// that may follow that same sampling occurrence.
pub struct StoreRecoveryCleanupFreshnessAdmission {
    sample: StoreRecoveryCleanupFreshnessSample,
    command: Option<PhysicalRecoveryCleanupRemovalCommand>,
}

pub struct StoreRecoveryCleanupFreshnessFailure {
    denial: StoreRecoveryCleanupFreshnessDenial,
    read: Option<PhysicalRecoveryCleanupFreshnessReadDenial>,
    binding: Option<super::StoreRecoveryBindingSampleFailure>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreRecoveryCleanupFreshnessDenial {
    FreshnessMediaMismatch,
    CurrentSelectorRead,
    InvalidCleanupEligibility,
}

pub(super) fn sample(
    authority: &super::PhysicalRecoveryFreshnessAuthority,
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    cleanup_plan_identity: [u8; 32],
    reopened: &CompletedPhysicalRecoveryFreshReopen,
    checkpoint: &worth_store_physical_format::VerifiedCheckpointStream,
    wal: worth_store_wal::VerifiedWalArtifact,
) -> Result<StoreRecoveryCleanupFreshnessAdmission, StoreRecoveryCleanupFreshnessFailure> {
    if !authority.matches_media_generation(media.media_generation()) {
        return Err(StoreRecoveryCleanupFreshnessFailure {
            denial: StoreRecoveryCleanupFreshnessDenial::FreshnessMediaMismatch,
            read: None,
            binding: None,
        });
    }
    let completed = match coordination.read_cleanup_current_selector(media) {
        PhysicalRecoveryCleanupFreshnessReadOutcome::Completed(completed) => completed,
        PhysicalRecoveryCleanupFreshnessReadOutcome::Denied(denial) => {
            return Err(StoreRecoveryCleanupFreshnessFailure {
                denial: StoreRecoveryCleanupFreshnessDenial::CurrentSelectorRead,
                read: Some(denial),
                binding: None,
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
        sealed_publication_basis: reopened.fresh_reopen_occurrence().plan(),
        policy_identity: cleanup_policy_identity(media.store_identity()),
        selector_read_operation: completed.physical().physical().operation(),
        work: completed.work(),
        signal: completed.signal(),
    };
    let command = if observed_published_generation == reopened.root().generation() {
        Some(admit_cleanup_command(
            authority,
            coordination,
            media,
            cleanup_plan_identity,
            reopened,
            checkpoint,
            wal,
        )?)
    } else {
        None
    };
    Ok(StoreRecoveryCleanupFreshnessAdmission { sample, command })
}

fn admit_cleanup_command(
    authority: &super::PhysicalRecoveryFreshnessAuthority,
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    cleanup_plan_identity: [u8; 32],
    reopened: &CompletedPhysicalRecoveryFreshReopen,
    checkpoint: &worth_store_physical_format::VerifiedCheckpointStream,
    wal: worth_store_wal::VerifiedWalArtifact,
) -> Result<PhysicalRecoveryCleanupRemovalCommand, StoreRecoveryCleanupFreshnessFailure> {
    let maximum_operations = checkpoint
        .footer()
        .binding_record_count()
        .saturating_add(wal.frames().len() as u64);
    let binding = super::binding::sample_binding(
        authority,
        media,
        checkpoint,
        wal.frames(),
        maximum_operations,
        wal.inspection().byte_count(),
    )
    .map_err(|binding| StoreRecoveryCleanupFreshnessFailure {
        denial: StoreRecoveryCleanupFreshnessDenial::InvalidCleanupEligibility,
        read: None,
        binding: Some(binding),
    })?;
    if !wal_members_are_terminal(&binding) {
        return Err(StoreRecoveryCleanupFreshnessFailure {
            denial: StoreRecoveryCleanupFreshnessDenial::InvalidCleanupEligibility,
            read: None,
            binding: None,
        });
    }
    PhysicalRecoveryCleanupRemovalCommand::admit(
        media,
        coordination,
        cleanup_plan_identity,
        reopened,
        checkpoint,
        wal.inspection(),
    )
    .ok_or(StoreRecoveryCleanupFreshnessFailure {
        denial: StoreRecoveryCleanupFreshnessDenial::InvalidCleanupEligibility,
        read: None,
        binding: None,
    })
}

fn wal_members_are_terminal(binding: &super::StoreRecoveryBindingFreshnessSample) -> bool {
    !binding.wal_members().is_empty()
        && binding.wal_members().iter().all(|member| {
            binding
                .operations()
                .binary_search_by_key(&member.operation_identity(), |operation| {
                    operation.idempotency_identity()
                })
                .ok()
                .and_then(|index| binding.operations().get(index))
                .is_some_and(|operation| {
                    operation.fate() != super::StoreRecoveryOperationFate::Indeterminate
                })
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
        Option<PhysicalRecoveryCleanupRemovalCommand>,
    ) {
        (self.sample, self.command)
    }
}

impl StoreRecoveryCleanupFreshnessFailure {
    pub const fn denial(&self) -> StoreRecoveryCleanupFreshnessDenial {
        self.denial
    }
    pub const fn read(&self) -> Option<&PhysicalRecoveryCleanupFreshnessReadDenial> {
        self.read.as_ref()
    }
    pub const fn binding(&self) -> Option<&super::StoreRecoveryBindingSampleFailure> {
        self.binding.as_ref()
    }
}
