use forge_store_security::StoreSecurityScopeIdentity;

use super::{BlobPhysicalManifestObservation, BlobPhysicalManifestObservationDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestTraversalRequest {
    reachability_digest: String,
    reachability_generation_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestTraversalObservation {
    placement_digest: String,
    placement_generation_sequence: u64,
    security_scope: StoreSecurityScopeIdentity,
    external_chunk_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedBlobPhysicalManifestTraversal {
    _private: (),
}

pub trait PhysicalStoreBlobManifestTraverser {
    type Error;

    fn traverse_blob_manifest(
        &mut self,
        request: BlobPhysicalManifestTraversalRequest,
    ) -> Result<BlobPhysicalManifestTraversalObservation, Self::Error>;
}

pub struct BlobPhysicalManifestTraversalSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedBlobPhysicalManifestTraversal,
}

impl BlobPhysicalManifestTraversalRequest {
    pub fn new(
        reachability_digest: impl Into<String>,
        reachability_generation_sequence: u64,
    ) -> Result<Self, BlobPhysicalManifestObservationDenial> {
        let reachability_digest = reachability_digest.into();
        if reachability_digest.is_empty() {
            return Err(BlobPhysicalManifestObservationDenial::EmptyManifestDigest);
        }
        if reachability_generation_sequence == 0 {
            return Err(BlobPhysicalManifestObservationDenial::ZeroGenerationSequence);
        }
        Ok(Self {
            reachability_digest,
            reachability_generation_sequence,
        })
    }

    pub fn reachability_digest(&self) -> &str {
        &self.reachability_digest
    }

    pub const fn reachability_generation_sequence(&self) -> u64 {
        self.reachability_generation_sequence
    }
}

impl BlobPhysicalManifestTraversalObservation {
    pub fn new(
        placement_digest: impl Into<String>,
        placement_generation_sequence: u64,
        security_scope: StoreSecurityScopeIdentity,
        external_chunk_present: bool,
    ) -> Result<Self, BlobPhysicalManifestObservationDenial> {
        let placement_digest = placement_digest.into();
        if placement_digest.is_empty() {
            return Err(BlobPhysicalManifestObservationDenial::EmptyManifestDigest);
        }
        if placement_generation_sequence == 0 {
            return Err(BlobPhysicalManifestObservationDenial::ZeroGenerationSequence);
        }
        Ok(Self {
            placement_digest,
            placement_generation_sequence,
            security_scope,
            external_chunk_present,
        })
    }
}

impl StoreOwnedBlobPhysicalManifestTraversal {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self { _private: () }
    }
}

impl<'backend, Backend> BlobPhysicalManifestTraversalSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedBlobPhysicalManifestTraversal,
    ) -> Self {
        Self { backend, authority }
    }

    pub fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(
            backend,
            StoreOwnedBlobPhysicalManifestTraversal::store_owned(),
        )
    }

    pub fn execute(
        &mut self,
        request: BlobPhysicalManifestTraversalRequest,
    ) -> Result<BlobPhysicalManifestObservation, Backend::Error>
    where
        Backend: PhysicalStoreBlobManifestTraverser,
    {
        let observation = self.backend.traverse_blob_manifest(request.clone())?;
        Ok(self.authority.complete(request, observation))
    }
}

impl StoreOwnedBlobPhysicalManifestTraversal {
    fn complete(
        self,
        request: BlobPhysicalManifestTraversalRequest,
        observation: BlobPhysicalManifestTraversalObservation,
    ) -> BlobPhysicalManifestObservation {
        BlobPhysicalManifestObservation::from_backend_manifest_traversal_unchecked(
            request.reachability_digest,
            request.reachability_generation_sequence,
            observation.placement_digest,
            observation.placement_generation_sequence,
            observation.security_scope,
            observation.external_chunk_present,
        )
    }
}
