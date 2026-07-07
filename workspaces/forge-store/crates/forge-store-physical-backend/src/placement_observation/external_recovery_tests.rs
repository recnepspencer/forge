use super::*;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[test]
fn recoverability_admission_rejects_unrelated_missing_denial() {
    let manifest = manifest_validation("placement-a");
    let probe =
        recovery_probe(manifest.clone(), "placement-a", true).expect("recovery probe should admit");
    let missing = missing_denial("placement-b");
    let orphan = orphan_scan("placement-a");
    let cleanup =
        cleanup_receipt(orphan.clone(), "placement-a", true).expect("cleanup should admit");

    assert_eq!(
        StoreExternalPlacementRecoverabilityEvidence::admit(
            manifest, probe, missing, orphan, cleanup
        ),
        Err(ExternalPlacementRecoverabilityDenial::MissingDenialManifestMismatch)
    );
}

#[test]
fn recoverability_admission_rejects_unrelated_orphan_scan() {
    let manifest = manifest_validation("placement-a");
    let probe =
        recovery_probe(manifest.clone(), "placement-a", true).expect("recovery probe should admit");
    let missing = missing_denial("placement-a");
    let orphan = orphan_scan("placement-b");
    let cleanup =
        cleanup_receipt(orphan.clone(), "placement-b", true).expect("cleanup should admit");

    assert_eq!(
        StoreExternalPlacementRecoverabilityEvidence::admit(
            manifest, probe, missing, orphan, cleanup
        ),
        Err(ExternalPlacementRecoverabilityDenial::OrphanScanManifestMismatch)
    );
}

#[test]
fn recovery_probe_must_be_executed_for_manifest_basis() {
    let manifest = manifest_validation("placement-a");

    assert_eq!(
        recovery_probe(manifest, "placement-a", false),
        Err(ExternalPlacementRecoverabilityDenial::RecoveryProbeIncomplete)
    );
}

#[test]
fn cleanup_receipt_must_be_executed_for_orphan_scan_basis() {
    let orphan = orphan_scan("placement-a");

    assert_eq!(
        cleanup_receipt(orphan, "placement-a", false),
        Err(ExternalPlacementRecoverabilityDenial::CleanupIncomplete)
    );
}

#[test]
fn recovery_probe_rejects_copied_manifest_digest() {
    let manifest = manifest_validation("placement-a");

    assert_eq!(
        recovery_probe(manifest, "placement-b", true),
        Err(ExternalPlacementRecoverabilityDenial::ManifestProbeMismatch)
    );
}

#[test]
fn cleanup_receipt_rejects_copied_orphan_token() {
    let orphan = orphan_scan("placement-a");

    assert_eq!(
        cleanup_receipt(orphan, "placement-b", true),
        Err(ExternalPlacementRecoverabilityDenial::CleanupScanMismatch)
    );
}

fn manifest_validation(placement_digest: &str) -> BlobPhysicalManifestValidation {
    let observation = manifest_observation(placement_digest, placement_digest, true);
    BlobPhysicalManifestValidation::validate_observation(observation)
        .expect("manifest validation should admit")
}

fn missing_denial(token: &str) -> ExternalPlacementMissingDenial {
    ExternalPlacementMissingDenial::from_missing_observation(residue_observation(
        BlobBackendResidueObservationKind::MissingExternalChunk,
        token,
    ))
    .expect("missing denial should admit")
}

fn orphan_scan(token: &str) -> ExternalPlacementOrphanScanReceipt {
    ExternalPlacementOrphanScanReceipt::from_orphan_scan_observation(residue_observation(
        BlobBackendResidueObservationKind::OrphanedPlacementResidue,
        token,
    ))
    .expect("orphan scan should admit")
}

fn recovery_probe(
    manifest: BlobPhysicalManifestValidation,
    placement_digest: &str,
    completed: bool,
) -> Result<ExternalPlacementRecoveryProbe, ExternalPlacementRecoverabilityDenial> {
    let mut backend = RecoveryProbeBackend {
        placement_digest: placement_digest.to_owned(),
        completed,
    };
    ExternalPlacementRecoveryProbeSession::for_store_backend(
        &mut backend,
        StoreOwnedExternalPlacementRecoveryProbe::store_owned(),
    )
    .execute(ExternalPlacementRecoveryProbeRequest::new(manifest))
    .map_err(|error| match error {
        ExternalPlacementRecoveryProbeExecutionError::Backend(()) => {
            panic!("recovery probe backend should not fail")
        }
        ExternalPlacementRecoveryProbeExecutionError::Denied(denial) => denial,
    })
}

fn cleanup_receipt(
    orphan_scan: ExternalPlacementOrphanScanReceipt,
    orphan_token: &str,
    completed: bool,
) -> Result<ExternalPlacementCleanupReceipt, ExternalPlacementRecoverabilityDenial> {
    let mut backend = CleanupBackend {
        orphan_token: orphan_token.to_owned(),
        completed,
    };
    ExternalPlacementCleanupSession::for_store_backend(
        &mut backend,
        StoreOwnedExternalPlacementCleanup::store_owned(),
    )
    .execute(ExternalPlacementCleanupRequest::new(orphan_scan))
    .map_err(|error| match error {
        ExternalPlacementCleanupExecutionError::Backend(()) => {
            panic!("cleanup backend should not fail")
        }
        ExternalPlacementCleanupExecutionError::Denied(denial) => denial,
    })
}

fn manifest_observation(
    reachability_digest: &str,
    placement_digest: &str,
    external_chunk_present: bool,
) -> BlobPhysicalManifestObservation {
    let request = BlobPhysicalManifestTraversalRequest::new(reachability_digest, 1)
        .expect("manifest request should admit");
    let mut backend = ManifestTraversalBackend {
        placement_digest: placement_digest.to_owned(),
        external_chunk_present,
    };
    BlobPhysicalManifestTraversalSession::for_store_backend(
        &mut backend,
        StoreOwnedBlobPhysicalManifestTraversal::store_owned(),
    )
    .execute(request)
    .expect("manifest traversal should admit")
}

fn residue_observation(
    kind: BlobBackendResidueObservationKind,
    token: &str,
) -> BlobBackendResidueObservation {
    let mut backend = ResidueScanBackend {
        token: token.to_owned(),
    };
    BlobBackendResidueScanSession::for_store_backend(
        &mut backend,
        StoreOwnedBlobBackendResidueScan::store_owned(),
    )
    .execute(BlobBackendResidueScanRequest::new(kind))
    .expect("residue scan should admit")
}

fn security_scope() -> StoreSecurityScopeIdentity {
    StoreSecurityScopeIdentity::from_physical_security_scope(
        physical_witness(),
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn physical_witness() -> forge_store_aspect_native::StorePhysicalBoundaryWitness {
    use forge_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };

    forge_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary")
}

struct ManifestTraversalBackend {
    placement_digest: String,
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
            security_scope(),
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
        Ok(BlobBackendResidueScanObservation::new(
            request.kind(),
            self.token.clone(),
        ))
    }
}

struct RecoveryProbeBackend {
    placement_digest: String,
    completed: bool,
}

impl PhysicalStoreExternalPlacementRecoveryProber for RecoveryProbeBackend {
    type Error = ();

    fn probe_external_placement_recovery(
        &mut self,
        _: ExternalPlacementRecoveryProbeRequest,
    ) -> Result<ExternalPlacementRecoveryProbeObservation, Self::Error> {
        Ok(ExternalPlacementRecoveryProbeObservation::new(
            self.placement_digest.clone(),
            self.completed,
        ))
    }
}

struct CleanupBackend {
    orphan_token: String,
    completed: bool,
}

impl PhysicalStoreExternalPlacementCleanupExecutor for CleanupBackend {
    type Error = ();

    fn cleanup_external_placement_residue(
        &mut self,
        _: ExternalPlacementCleanupRequest,
    ) -> Result<ExternalPlacementCleanupObservation, Self::Error> {
        Ok(ExternalPlacementCleanupObservation::new(
            self.orphan_token.clone(),
            self.completed,
        ))
    }
}
