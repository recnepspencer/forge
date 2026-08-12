use worth_store_physical_backend::{
    AdmittedRecoveryFilesystemMedia, QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::VerifiedCheckpointStream;
use worth_store_wal::VerifiedWalFrame;

use super::{
    binding, cleanup, PhysicalRecoveryFreshnessAuthority, StoreRecoveryBindingFreshnessSample,
    StoreRecoveryBindingSampleFailure, StoreRecoveryCleanupPlan,
    StoreRecoveryCleanupFreshnessAdmission, StoreRecoveryCleanupFreshnessFailure,
};

/// The sole Store-owned construction port for recovery freshness authority.
pub struct PhysicalRecoveryFreshnessPort {
    _private: (),
}

impl PhysicalRecoveryFreshnessPort {
    pub fn admit(
        media: &QualifiedRecoveryFilesystemMedia,
    ) -> Option<PhysicalRecoveryFreshnessAuthority> {
        PhysicalRecoveryFreshnessAuthority::issue(media.media_generation())
    }

    pub fn sample_binding<'frame>(
        coordination: &crate::physical_runtime::PhysicalRecoveryCoordination,
        media: &AdmittedRecoveryFilesystemMedia,
        checkpoint: &VerifiedCheckpointStream,
        wal_frames: impl IntoIterator<Item = &'frame VerifiedWalFrame>,
        maximum_operation_bindings: u64,
        maximum_redo_bytes: u64,
    ) -> Result<StoreRecoveryBindingFreshnessSample, StoreRecoveryBindingSampleFailure> {
        binding::sample_binding(
            coordination.freshness(),
            media,
            checkpoint,
            wal_frames,
            maximum_operation_bindings,
            maximum_redo_bytes,
        )
    }

    pub fn sample_cleanup(
        coordination: &crate::physical_runtime::PhysicalRecoveryCoordination,
        media: &AdmittedRecoveryFilesystemMedia,
        plan: &mut StoreRecoveryCleanupPlan<'_>,
        artifact: worth_store_wal::WalSegmentArtifactIdentity,
    ) -> Result<StoreRecoveryCleanupFreshnessAdmission, StoreRecoveryCleanupFreshnessFailure> {
        cleanup::sample(
            coordination.freshness(),
            coordination,
            media,
            plan,
            artifact,
        )
    }
}
