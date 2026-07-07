use forge_store_physical_backend::{
    BlobBackendResidueObservation, BlobBackendResidueObservationKind,
    BlobBackendResidueScanObservation, BlobBackendResidueScanRequest,
    BlobBackendResidueScanSession, BlobPhysicalManifestObservation,
    BlobPhysicalManifestTraversalObservation, BlobPhysicalManifestTraversalRequest,
    BlobPhysicalManifestTraversalSession, BlobPhysicalManifestValidation,
    ExternalPlacementCleanupObservation, ExternalPlacementCleanupRequest,
    ExternalPlacementCleanupSession, ExternalPlacementMissingDenial,
    ExternalPlacementOrphanScanReceipt, ExternalPlacementRecoveryProbeObservation,
    ExternalPlacementRecoveryProbeRequest, ExternalPlacementRecoveryProbeSession,
    PhysicalStoreBlobManifestTraverser, PhysicalStoreBlobResidueScanner,
    PhysicalStoreExternalPlacementCleanupExecutor, PhysicalStoreExternalPlacementRecoveryProber,
    StoreExternalPlacementRecoverabilityEvidence,
};
use forge_store_security::StoreSecurityScopeIdentity;

use crate::BlobChunkReachabilityProofSet;

pub(in crate::harness_execution) fn external_recovery(
    reachability: &BlobChunkReachabilityProofSet,
) -> StoreExternalPlacementRecoverabilityEvidence {
    let digest = reachability.stored_digest().digest().as_str();
    let observation = manifest_observation(
        digest,
        digest,
        reachability.security_metadata().identity(),
        true,
    );
    let manifest = BlobPhysicalManifestValidation::validate_observation(observation).expect("manifest");
    let probe = recovery_probe(manifest.clone());
    let missing = ExternalPlacementMissingDenial::from_missing_observation(
        residue_observation(BlobBackendResidueObservationKind::MissingExternalChunk, digest),
    )
    .expect("missing");
    let orphan = ExternalPlacementOrphanScanReceipt::from_orphan_scan_observation(
        residue_observation(BlobBackendResidueObservationKind::OrphanedPlacementResidue, digest),
    )
    .expect("orphan");
    let cleanup = cleanup_receipt(orphan.clone());
    StoreExternalPlacementRecoverabilityEvidence::admit(manifest, probe, missing, orphan, cleanup)
        .expect("recoverability")
}

fn manifest_observation(
    reachability_digest: &str,
    placement_digest: &str,
    security_scope: StoreSecurityScopeIdentity,
    external_chunk_present: bool,
) -> BlobPhysicalManifestObservation {
    let request = BlobPhysicalManifestTraversalRequest::new(reachability_digest, 1).expect("manifest request");
    let mut backend = ManifestTraversalBackend {
        placement_digest: placement_digest.to_owned(),
        security_scope,
        external_chunk_present,
    };
    BlobPhysicalManifestTraversalSession::for_owned_backend(&mut backend)
        .execute(request)
        .expect("manifest traversal")
}

fn residue_observation(
    kind: BlobBackendResidueObservationKind,
    token: &str,
) -> BlobBackendResidueObservation {
    let mut backend = ResidueScanBackend {
        token: token.to_owned(),
    };
    BlobBackendResidueScanSession::for_owned_backend(&mut backend)
        .execute(BlobBackendResidueScanRequest::new(kind))
        .expect("residue scan")
}

fn recovery_probe(
    manifest: BlobPhysicalManifestValidation,
) -> forge_store_physical_backend::ExternalPlacementRecoveryProbe {
    let mut backend = RecoveryProbeBackend {
        placement_digest: manifest.placement_digest().to_owned(),
    };
    ExternalPlacementRecoveryProbeSession::for_owned_backend(&mut backend)
        .execute(ExternalPlacementRecoveryProbeRequest::new(manifest))
        .expect("recovery probe")
}

fn cleanup_receipt(
    orphan_scan: ExternalPlacementOrphanScanReceipt,
) -> forge_store_physical_backend::ExternalPlacementCleanupReceipt {
    let mut backend = CleanupBackend {
        orphan_token: orphan_scan.token().to_owned(),
    };
    ExternalPlacementCleanupSession::for_owned_backend(&mut backend)
        .execute(ExternalPlacementCleanupRequest::new(orphan_scan))
        .expect("cleanup")
}

struct ManifestTraversalBackend {
    placement_digest: String,
    security_scope: StoreSecurityScopeIdentity,
    external_chunk_present: bool,
}

impl PhysicalStoreBlobManifestTraverser for ManifestTraversalBackend {
    type Error = ();

    fn traverse_blob_manifest(
        &mut self,
        _: BlobPhysicalManifestTraversalRequest,
    ) -> Result<BlobPhysicalManifestTraversalObservation, Self::Error> {
        BlobPhysicalManifestTraversalObservation::new(
            self.placement_digest.clone(),
            1,
            self.security_scope,
            self.external_chunk_present,
        )
        .map_err(|_| ())
    }
}

struct ResidueScanBackend {
    token: String,
}

impl PhysicalStoreBlobResidueScanner for ResidueScanBackend {
    type Error = ();

    fn scan_blob_residue(
        &mut self,
        request: BlobBackendResidueScanRequest,
    ) -> Result<BlobBackendResidueScanObservation, Self::Error> {
        Ok(BlobBackendResidueScanObservation::new(request.kind(), self.token.clone()))
    }
}

struct RecoveryProbeBackend {
    placement_digest: String,
}

impl PhysicalStoreExternalPlacementRecoveryProber for RecoveryProbeBackend {
    type Error = ();

    fn probe_external_placement_recovery(
        &mut self,
        _: ExternalPlacementRecoveryProbeRequest,
    ) -> Result<ExternalPlacementRecoveryProbeObservation, Self::Error> {
        Ok(ExternalPlacementRecoveryProbeObservation::new(
            self.placement_digest.clone(),
            true,
        ))
    }
}

struct CleanupBackend {
    orphan_token: String,
}

impl PhysicalStoreExternalPlacementCleanupExecutor for CleanupBackend {
    type Error = ();

    fn cleanup_external_placement_residue(
        &mut self,
        _: ExternalPlacementCleanupRequest,
    ) -> Result<ExternalPlacementCleanupObservation, Self::Error> {
        Ok(ExternalPlacementCleanupObservation::new(self.orphan_token.clone(), true))
    }
}
