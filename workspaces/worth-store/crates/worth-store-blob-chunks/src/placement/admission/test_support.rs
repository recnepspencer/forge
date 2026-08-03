#![cfg_attr(not(test), allow(dead_code, unused_imports))]

use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, BlobBackendResidueObservation,
    BlobBackendResidueObservationKind, BlobBackendResidueScanObservation,
    BlobBackendResidueScanRequest, BlobBackendResidueScanSession, BlobPhysicalManifestObservation,
    BlobPhysicalManifestTraversalObservation, BlobPhysicalManifestTraversalRequest,
    BlobPhysicalManifestTraversalSession, BlobPhysicalManifestValidation,
    ExternalPlacementCleanupObservation, ExternalPlacementCleanupRequest,
    ExternalPlacementCleanupSession, ExternalPlacementMissingDenial,
    ExternalPlacementOrphanScanReceipt, ExternalPlacementRecoveryProbeObservation,
    ExternalPlacementRecoveryProbeRequest, ExternalPlacementRecoveryProbeSession,
    PhysicalBackendCapabilityAdmissionAuthority, PhysicalStoreBlobManifestTraverser,
    PhysicalStoreBlobResidueScanner, PhysicalStoreExternalPlacementCleanupExecutor,
    PhysicalStoreExternalPlacementRecoveryProber, StoreExternalPlacementRecoverabilityEvidence,
    StoreOwnedBlobBackendResidueScan, StoreOwnedBlobPhysicalManifestTraversal,
    StoreOwnedExternalPlacementCleanup, StoreOwnedExternalPlacementRecoveryProbe,
};
use worth_store_tiering::{ColdPlacementState, ColdTierIoPosture};

use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobPlacementAdmissionAuthority,
    BlobPlacementIntent,
};

pub(crate) fn admit_inline_placement(
    reachability: &BlobChunkReachabilityProofSet,
) -> AdmittedBlobPlacement {
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    authority
        .admit(reachability, BlobPlacementIntent::inline())
        .expect("inline placement should admit")
}

pub(crate) fn admit_external_placement(
    reachability: &BlobChunkReachabilityProofSet,
) -> AdmittedBlobPlacement {
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    let recoverability = external_recovery(reachability);
    authority
        .admit(reachability, BlobPlacementIntent::external(&recoverability))
        .expect("external placement should admit")
}

pub(crate) fn admit_cold_placement(
    reachability: &BlobChunkReachabilityProofSet,
) -> AdmittedBlobPlacement {
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    let posture = cold_posture(reachability);
    authority
        .admit(
            reachability,
            BlobPlacementIntent::cold(&posture, ColdPlacementState::ColdAvailable),
        )
        .expect("cold placement should admit")
}

pub(crate) fn external_recovery(
    reachability: &BlobChunkReachabilityProofSet,
) -> StoreExternalPlacementRecoverabilityEvidence {
    let digest = reachability.stored_digest().digest().as_str();
    external_recovery_for_digest_and_scope(digest, reachability.security_metadata().identity())
}

pub(crate) fn external_recovery_for_digest(
    digest: &str,
) -> StoreExternalPlacementRecoverabilityEvidence {
    external_recovery_for_digest_and_scope(digest, default_security_scope())
}

pub(crate) fn external_recovery_for_digest_and_scope(
    digest: &str,
    security_scope: worth_store_security::StoreSecurityScopeIdentity,
) -> StoreExternalPlacementRecoverabilityEvidence {
    let observation = manifest_observation(digest, digest, security_scope, true);
    let manifest = BlobPhysicalManifestValidation::validate_observation(observation)
        .expect("manifest should validate");
    let probe = recovery_probe(manifest.clone());
    let missing = ExternalPlacementMissingDenial::from_missing_observation(residue_observation(
        BlobBackendResidueObservationKind::MissingExternalChunk,
        digest,
    ))
    .expect("missing denial should admit");
    let orphan =
        ExternalPlacementOrphanScanReceipt::from_orphan_scan_observation(residue_observation(
            BlobBackendResidueObservationKind::OrphanedPlacementResidue,
            digest,
        ))
        .expect("orphan scan should admit");
    let cleanup = cleanup_receipt(orphan.clone());
    StoreExternalPlacementRecoverabilityEvidence::admit(manifest, probe, missing, orphan, cleanup)
        .expect("external recoverability should admit")
}

pub(crate) fn cold_posture(reachability: &BlobChunkReachabilityProofSet) -> ColdTierIoPosture {
    worth_store_tiering::certification_test_support::cold_tier_io_posture_for_certification_test(
        reachability.security_metadata().identity(),
    )
}

pub(crate) fn admitted_backend() -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_mmap_coherence()
                .with_async_ordering()
                .with_secure_frame_io()
                .with_flush_ordering()
                .with_fdatasync_durability()
                .with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend should admit")
}

fn manifest_observation(
    reachability_digest: &str,
    placement_digest: &str,
    security_scope: worth_store_security::StoreSecurityScopeIdentity,
    external_chunk_present: bool,
) -> BlobPhysicalManifestObservation {
    let request = BlobPhysicalManifestTraversalRequest::new(reachability_digest, 1)
        .expect("manifest request should admit");
    let mut backend = ManifestTraversalBackend {
        placement_digest: placement_digest.to_owned(),
        security_scope,
        external_chunk_present,
    };
    BlobPhysicalManifestTraversalSession::for_store_backend(
        &mut backend,
        StoreOwnedBlobPhysicalManifestTraversal::for_certification_test_authority(),
    )
    .execute(request)
    .expect("manifest traversal should admit")
}

pub(crate) fn residue_observation(
    kind: BlobBackendResidueObservationKind,
    token: &str,
) -> BlobBackendResidueObservation {
    let mut backend = ResidueScanBackend {
        token: token.to_owned(),
    };
    BlobBackendResidueScanSession::for_store_backend(
        &mut backend,
        StoreOwnedBlobBackendResidueScan::for_certification_test_authority(),
    )
    .execute(BlobBackendResidueScanRequest::new(kind))
    .expect("residue scan should admit")
}

fn recovery_probe(
    manifest: BlobPhysicalManifestValidation,
) -> worth_store_physical_backend::ExternalPlacementRecoveryProbe {
    let mut backend = RecoveryProbeBackend {
        placement_digest: manifest.placement_digest().to_owned(),
        completed: true,
    };
    ExternalPlacementRecoveryProbeSession::for_store_backend(
        &mut backend,
        StoreOwnedExternalPlacementRecoveryProbe::for_certification_test_authority(),
    )
    .execute(ExternalPlacementRecoveryProbeRequest::new(manifest))
    .expect("recovery probe should admit")
}

fn cleanup_receipt(
    orphan_scan: ExternalPlacementOrphanScanReceipt,
) -> worth_store_physical_backend::ExternalPlacementCleanupReceipt {
    let mut backend = CleanupBackend {
        orphan_token: orphan_scan.token().to_owned(),
        completed: true,
    };
    ExternalPlacementCleanupSession::for_store_backend(
        &mut backend,
        StoreOwnedExternalPlacementCleanup::for_certification_test_authority(),
    )
    .execute(ExternalPlacementCleanupRequest::new(orphan_scan))
    .expect("cleanup should admit")
}

fn default_security_scope() -> worth_store_security::StoreSecurityScopeIdentity {
    use worth_store_security::{
        StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
        StoreKeyScope, StoreKeyVersionPosture, StoreTenantScope,
    };

    worth_store_security::StoreSecurityScopeIdentity::from_physical_security_scope(
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

fn physical_witness() -> worth_store_aspect_native::StorePhysicalBoundaryWitness {
    use worth_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };

    worth_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary")
}

struct ManifestTraversalBackend {
    placement_digest: String,
    security_scope: worth_store_security::StoreSecurityScopeIdentity,
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
