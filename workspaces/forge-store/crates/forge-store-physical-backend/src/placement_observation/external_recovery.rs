use super::{
    BlobBackendResidueObservation, BlobBackendResidueObservationKind,
    BlobPhysicalManifestValidation,
};
use forge_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementRecoveryProbe {
    manifest: BlobPhysicalManifestValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementMissingDenial {
    token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementOrphanScanReceipt {
    token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementCleanupReceipt {
    orphan_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreExternalPlacementRecoverabilityEvidence {
    manifest: BlobPhysicalManifestValidation,
    probe: ExternalPlacementRecoveryProbe,
    missing_denial: ExternalPlacementMissingDenial,
    orphan_scan: ExternalPlacementOrphanScanReceipt,
    cleanup: ExternalPlacementCleanupReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalPlacementRecoverabilityDenial {
    MissingDenialObservationKind {
        actual: BlobBackendResidueObservationKind,
    },
    OrphanScanObservationKind {
        actual: BlobBackendResidueObservationKind,
    },
    ManifestProbeMismatch,
    MissingDenialManifestMismatch,
    OrphanScanManifestMismatch,
    CleanupScanMismatch,
    RecoveryProbeIncomplete,
    CleanupIncomplete,
}

impl ExternalPlacementRecoveryProbe {
    pub(super) fn from_store_recovery_probe(manifest: BlobPhysicalManifestValidation) -> Self {
        Self { manifest }
    }

    pub const fn manifest(&self) -> &BlobPhysicalManifestValidation {
        &self.manifest
    }
}

impl ExternalPlacementMissingDenial {
    pub fn from_missing_observation(
        observation: BlobBackendResidueObservation,
    ) -> Result<Self, ExternalPlacementRecoverabilityDenial> {
        if observation.kind() != BlobBackendResidueObservationKind::MissingExternalChunk {
            return Err(
                ExternalPlacementRecoverabilityDenial::MissingDenialObservationKind {
                    actual: observation.kind(),
                },
            );
        }
        Ok(Self {
            token: observation.observed_token().to_owned(),
        })
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl ExternalPlacementOrphanScanReceipt {
    pub fn from_orphan_scan_observation(
        observation: BlobBackendResidueObservation,
    ) -> Result<Self, ExternalPlacementRecoverabilityDenial> {
        if observation.kind() != BlobBackendResidueObservationKind::OrphanedPlacementResidue {
            return Err(
                ExternalPlacementRecoverabilityDenial::OrphanScanObservationKind {
                    actual: observation.kind(),
                },
            );
        }
        Ok(Self {
            token: observation.observed_token().to_owned(),
        })
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl ExternalPlacementCleanupReceipt {
    pub(super) fn from_store_cleanup(orphan_token: String) -> Self {
        Self { orphan_token }
    }

    pub fn orphan_token(&self) -> &str {
        &self.orphan_token
    }
}

impl StoreExternalPlacementRecoverabilityEvidence {
    pub fn admit(
        manifest: BlobPhysicalManifestValidation,
        probe: ExternalPlacementRecoveryProbe,
        missing_denial: ExternalPlacementMissingDenial,
        orphan_scan: ExternalPlacementOrphanScanReceipt,
        cleanup: ExternalPlacementCleanupReceipt,
    ) -> Result<Self, ExternalPlacementRecoverabilityDenial> {
        if manifest != *probe.manifest() {
            return Err(ExternalPlacementRecoverabilityDenial::ManifestProbeMismatch);
        }
        if missing_denial.token() != manifest.placement_digest() {
            return Err(ExternalPlacementRecoverabilityDenial::MissingDenialManifestMismatch);
        }
        if orphan_scan.token() != manifest.placement_digest() {
            return Err(ExternalPlacementRecoverabilityDenial::OrphanScanManifestMismatch);
        }
        if orphan_scan.token() != cleanup.orphan_token() {
            return Err(ExternalPlacementRecoverabilityDenial::CleanupScanMismatch);
        }
        Ok(Self {
            manifest,
            probe,
            missing_denial,
            orphan_scan,
            cleanup,
        })
    }

    pub const fn manifest(&self) -> &BlobPhysicalManifestValidation {
        &self.manifest
    }

    pub fn matches_placement_manifest_basis(
        &self,
        reachability_digest: &str,
        security_scope: StoreSecurityScopeIdentity,
    ) -> bool {
        self.manifest.reachability_digest() == reachability_digest
            && self.manifest.placement_digest() == reachability_digest
            && self.manifest.security_scope() == security_scope
    }

    pub const fn probe(&self) -> &ExternalPlacementRecoveryProbe {
        &self.probe
    }

    pub const fn missing_denial(&self) -> &ExternalPlacementMissingDenial {
        &self.missing_denial
    }

    pub const fn orphan_scan(&self) -> &ExternalPlacementOrphanScanReceipt {
        &self.orphan_scan
    }

    pub const fn cleanup(&self) -> &ExternalPlacementCleanupReceipt {
        &self.cleanup
    }
}
