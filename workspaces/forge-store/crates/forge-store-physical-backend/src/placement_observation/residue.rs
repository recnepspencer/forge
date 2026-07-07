use forge_store_security::StoreSecurityScopeIdentity;

use crate::BackendCapabilityKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobBackendResidueObservationKind {
    MissingExternalChunk,
    StaleGenerationRow,
    OrphanedPlacementResidue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobBackendResidueObservation {
    kind: BlobBackendResidueObservationKind,
    observed_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestObservation {
    reachability_digest: String,
    reachability_generation_sequence: u64,
    placement_digest: String,
    placement_generation_sequence: u64,
    security_scope: StoreSecurityScopeIdentity,
    external_chunk_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestValidation {
    reachability_digest: String,
    reachability_generation_sequence: u64,
    placement_digest: String,
    placement_generation_sequence: u64,
    security_scope: StoreSecurityScopeIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPhysicalManifestObservationDenial {
    WrongBackendCapability { actual: BackendCapabilityKind },
    EmptyManifestDigest,
    ZeroGenerationSequence,
}

impl BlobBackendResidueObservation {
    pub(super) fn observed(
        kind: BlobBackendResidueObservationKind,
        token: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            observed_token: token.into(),
        }
    }

    pub const fn kind(&self) -> BlobBackendResidueObservationKind {
        self.kind
    }

    pub fn observed_token(&self) -> &str {
        &self.observed_token
    }
}

impl BlobPhysicalManifestObservation {
    pub(super) fn from_backend_manifest_traversal_unchecked(
        reachability_digest: impl Into<String>,
        reachability_generation_sequence: u64,
        placement_digest: impl Into<String>,
        placement_generation_sequence: u64,
        security_scope: StoreSecurityScopeIdentity,
        external_chunk_present: bool,
    ) -> Self {
        Self {
            reachability_digest: reachability_digest.into(),
            reachability_generation_sequence,
            placement_digest: placement_digest.into(),
            placement_generation_sequence,
            security_scope,
            external_chunk_present,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn for_certification_test_authority(
        reachability_digest: impl Into<String>,
        reachability_generation_sequence: u64,
        placement_digest: impl Into<String>,
        placement_generation_sequence: u64,
        security_scope: StoreSecurityScopeIdentity,
        external_chunk_present: bool,
    ) -> Result<Self, BlobPhysicalManifestObservationDenial> {
        let observation = Self::from_backend_manifest_traversal_unchecked(
            reachability_digest,
            reachability_generation_sequence,
            placement_digest,
            placement_generation_sequence,
            security_scope,
            external_chunk_present,
        );
        if observation.reachability_digest().is_empty() || observation.placement_digest().is_empty()
        {
            return Err(BlobPhysicalManifestObservationDenial::EmptyManifestDigest);
        }
        if observation.reachability_generation_sequence() == 0
            || observation.placement_generation_sequence() == 0
        {
            return Err(BlobPhysicalManifestObservationDenial::ZeroGenerationSequence);
        }
        Ok(observation)
    }

    pub fn reachability_digest(&self) -> &str {
        &self.reachability_digest
    }

    pub const fn reachability_generation_sequence(&self) -> u64 {
        self.reachability_generation_sequence
    }

    pub fn placement_digest(&self) -> &str {
        &self.placement_digest
    }

    pub const fn placement_generation_sequence(&self) -> u64 {
        self.placement_generation_sequence
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn external_chunk_present(&self) -> bool {
        self.external_chunk_present
    }
}

impl BlobPhysicalManifestValidation {
    pub fn validate_observation(
        observation: BlobPhysicalManifestObservation,
    ) -> Result<Self, BlobBackendResidueObservation> {
        if !observation.external_chunk_present() {
            return Err(BlobBackendResidueObservation::observed(
                BlobBackendResidueObservationKind::MissingExternalChunk,
                observation.placement_digest(),
            ));
        }
        if observation.reachability_generation_sequence()
            != observation.placement_generation_sequence()
        {
            return Err(BlobBackendResidueObservation::observed(
                BlobBackendResidueObservationKind::StaleGenerationRow,
                observation.placement_digest(),
            ));
        }
        Ok(Self {
            reachability_digest: observation.reachability_digest,
            reachability_generation_sequence: observation.reachability_generation_sequence,
            placement_digest: observation.placement_digest,
            placement_generation_sequence: observation.placement_generation_sequence,
            security_scope: observation.security_scope,
        })
    }

    pub fn reachability_digest(&self) -> &str {
        &self.reachability_digest
    }

    pub const fn reachability_generation_sequence(&self) -> u64 {
        self.reachability_generation_sequence
    }

    pub fn placement_digest(&self) -> &str {
        &self.placement_digest
    }

    pub const fn placement_generation_sequence(&self) -> u64 {
        self.placement_generation_sequence
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_manifest_validation_denies_missing_stale_and_orphaned_residue() {
        let missing = BlobPhysicalManifestValidation::validate_observation(manifest_observation(
            "missing-r",
            1,
            "missing-p",
            1,
            security_scope(),
            false,
        ))
        .expect_err("missing external chunk must deny");
        assert_eq!(
            missing.kind(),
            BlobBackendResidueObservationKind::MissingExternalChunk
        );

        let stale = BlobPhysicalManifestValidation::validate_observation(manifest_observation(
            "stale-r",
            1,
            "stale-p",
            2,
            security_scope(),
            true,
        ))
        .expect_err("stale generation row must deny");
        assert_eq!(
            stale.kind(),
            BlobBackendResidueObservationKind::StaleGenerationRow
        );

        let orphan = residue_observation(
            BlobBackendResidueObservationKind::OrphanedPlacementResidue,
            "orphan-p",
        );
        assert_eq!(
            orphan.kind(),
            BlobBackendResidueObservationKind::OrphanedPlacementResidue
        );
    }

    fn manifest_observation(
        reachability_digest: &str,
        reachability_generation_sequence: u64,
        placement_digest: &str,
        placement_generation_sequence: u64,
        security_scope: StoreSecurityScopeIdentity,
        external_chunk_present: bool,
    ) -> BlobPhysicalManifestObservation {
        let request = crate::BlobPhysicalManifestTraversalRequest::new(
            reachability_digest,
            reachability_generation_sequence,
        )
        .expect("manifest request admits");
        let mut backend = ManifestTraversalBackend {
            placement_digest: placement_digest.to_owned(),
            placement_generation_sequence,
            security_scope,
            external_chunk_present,
        };
        crate::BlobPhysicalManifestTraversalSession::for_store_backend(
            &mut backend,
            crate::StoreOwnedBlobPhysicalManifestTraversal::store_owned(),
        )
        .execute(request)
        .expect("manifest observation admits")
    }

    fn residue_observation(
        kind: BlobBackendResidueObservationKind,
        token: &str,
    ) -> BlobBackendResidueObservation {
        let mut backend = ResidueScanBackend {
            token: token.to_owned(),
        };
        crate::BlobBackendResidueScanSession::for_store_backend(
            &mut backend,
            crate::StoreOwnedBlobBackendResidueScan::store_owned(),
        )
        .execute(crate::BlobBackendResidueScanRequest::new(kind))
        .expect("residue scan should admit")
    }

    fn security_scope() -> StoreSecurityScopeIdentity {
        use forge_store_security::{
            StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
            StoreKeyScope, StoreKeyVersionPosture, StoreTenantScope,
        };

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
        placement_generation_sequence: u64,
        security_scope: StoreSecurityScopeIdentity,
        external_chunk_present: bool,
    }

    impl crate::PhysicalStoreBlobManifestTraverser for ManifestTraversalBackend {
        type Error = ();

        fn traverse_blob_manifest(
            &mut self,
            _: crate::BlobPhysicalManifestTraversalRequest,
        ) -> Result<crate::BlobPhysicalManifestTraversalObservation, Self::Error> {
            crate::BlobPhysicalManifestTraversalObservation::new(
                self.placement_digest.clone(),
                self.placement_generation_sequence,
                self.security_scope,
                self.external_chunk_present,
            )
            .map_err(|_| ())
        }
    }

    struct ResidueScanBackend {
        token: String,
    }

    impl crate::PhysicalStoreBlobResidueScanner for ResidueScanBackend {
        type Error = ();

        fn scan_blob_residue(
            &mut self,
            request: crate::BlobBackendResidueScanRequest,
        ) -> Result<crate::BlobBackendResidueScanObservation, Self::Error> {
            Ok(crate::BlobBackendResidueScanObservation::new(
                request.kind(),
                self.token.clone(),
            ))
        }
    }
}
