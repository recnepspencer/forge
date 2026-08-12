use std::path::Path;
use worth_store_physical_backend::{
    AdmittedRecoveryFilesystemMedia, QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::VerifiedCheckpointStream;
use worth_store_wal::VerifiedWalFrame;

use super::{
    binding, PhysicalRecoveryFreshnessAuthority, StoreRecoveryBindingFreshnessSample,
    StoreRecoveryBindingSampleFailure,
};

/// The sole Store-owned construction port for recovery freshness authority.
pub struct PhysicalRecoveryFreshnessPort {
    _private: (),
}

impl PhysicalRecoveryFreshnessPort {
    pub fn admit(
        media: &QualifiedRecoveryFilesystemMedia,
        root: &Path,
    ) -> Option<PhysicalRecoveryFreshnessAuthority> {
        let cleanup_media =
            crate::physical_runtime::media_ownership::RecoveryCleanupMediaOwner::open(root)?;
        PhysicalRecoveryFreshnessAuthority::issue(media.media_generation(), cleanup_media)
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn admit_for_certification(
        media: &QualifiedRecoveryFilesystemMedia,
        root: &Path,
        schedule: worth_store_physical_backend::MediaFaultSchedule,
    ) -> Option<PhysicalRecoveryFreshnessAuthority> {
        let cleanup_media = crate::physical_runtime::media_ownership::RecoveryCleanupMediaOwner::open_for_certification(
            root,
            schedule,
        )?;
        PhysicalRecoveryFreshnessAuthority::issue(media.media_generation(), cleanup_media)
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
}
