use worth_store_physical_backend::AdmittedRecoveryFilesystemMedia;
use worth_store_physical_format::RecordArtifactFile;

use crate::physical_runtime::{
    CompletedPhysicalRecoveryFreshReopen, PhysicalRecoveryCoordination, RuntimeIdentity,
};

use super::{RecoveredPhysicalRuntimeConstructionDenial, RecoveredPhysicalRuntimeCore};

/// The sole Store-owned construction port for a recovered physical runtime.
pub struct PhysicalRecoveryConstructionPort {
    _private: (),
}

impl PhysicalRecoveryConstructionPort {
    pub fn construct(
        coordination: PhysicalRecoveryCoordination,
        media: AdmittedRecoveryFilesystemMedia,
        reopen: CompletedPhysicalRecoveryFreshReopen,
    ) -> Result<RecoveredPhysicalRuntimeCore, RecoveredPhysicalRuntimeConstructionDenial> {
        let occurrence = reopen.fresh_reopen_occurrence();
        let expected_root = RecordArtifactFile::RootManifest {
            generation: reopen.root().generation(),
        };
        if !coordination.construction_authority().matches(
            media.store_identity(),
            media.media_generation(),
            coordination.session_identity(),
        ) {
            let _ = coordination.shutdown_is_quiescent();
            return Err(RecoveredPhysicalRuntimeConstructionDenial::ConstructionAuthorityMismatch);
        }
        if occurrence.session() != coordination.session_identity()
            || occurrence.generation() != reopen.root().generation()
            || occurrence.selector().artifact() != RecordArtifactFile::CurrentRootSelector
            || occurrence.root().artifact() != expected_root
            || occurrence.selector().bytes().is_empty()
            || occurrence.root().bytes().is_empty()
        {
            let _ = coordination.shutdown_is_quiescent();
            return Err(RecoveredPhysicalRuntimeConstructionDenial::BindingMismatch);
        }
        let _ = coordination.reconcile_signal_settlements();
        if !coordination.is_ready() {
            let _ = coordination.shutdown_is_quiescent();
            return Err(RecoveredPhysicalRuntimeConstructionDenial::CoordinationNotQuiescent);
        }
        let recovery_runtime = coordination.runtime_identity();
        let Some(runtime) = fresh_runtime_identity(recovery_runtime) else {
            let _ = coordination.shutdown_is_quiescent();
            return Err(RecoveredPhysicalRuntimeConstructionDenial::RuntimeIdentityUnavailable);
        };
        if !coordination.shutdown_is_quiescent() {
            return Err(RecoveredPhysicalRuntimeConstructionDenial::CoordinationNotQuiescent);
        }
        Ok(RecoveredPhysicalRuntimeCore {
            store: media.store_identity(),
            runtime,
            recovery_runtime,
            root: reopen.root().clone(),
            media,
            reopen,
        })
    }
}

fn fresh_runtime_identity(previous: RuntimeIdentity) -> Option<RuntimeIdentity> {
    loop {
        let candidate = RuntimeIdentity::generate()?;
        if candidate != previous {
            return Some(candidate);
        }
    }
}
