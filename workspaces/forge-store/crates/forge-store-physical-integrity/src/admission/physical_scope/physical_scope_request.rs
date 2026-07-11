use crate::ChecksumCoverageBasis;
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, ManifestMembershipProof, PhysicalReferenceScope,
    RootManifestIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScopeAdmissionRequest {
    scope: PhysicalReferenceScope,
    membership: ManifestMembershipProof,
    root_posture: RootManifestIntegrityPosture,
    checkpoint_adjacency: CheckpointAdjacencyPosture,
    checksum_scope: ChecksumCoverageBasis,
}

impl PhysicalScopeAdmissionRequest {
    pub fn new(
        scope: PhysicalReferenceScope,
        membership: ManifestMembershipProof,
        root_posture: RootManifestIntegrityPosture,
        checkpoint_adjacency: CheckpointAdjacencyPosture,
        checksum_scope: ChecksumCoverageBasis,
    ) -> Self {
        Self {
            scope,
            membership,
            root_posture,
            checkpoint_adjacency,
            checksum_scope,
        }
    }

    pub fn page(
        scope: PhysicalReferenceScope,
        membership: ManifestMembershipProof,
        root_posture: RootManifestIntegrityPosture,
        checksum_scope: ChecksumCoverageBasis,
    ) -> Self {
        Self::new(
            scope,
            membership,
            root_posture,
            CheckpointAdjacencyPosture::NotApplicable,
            checksum_scope,
        )
    }

    pub fn frame(
        scope: PhysicalReferenceScope,
        membership: ManifestMembershipProof,
        root_posture: RootManifestIntegrityPosture,
        checkpoint_adjacency: CheckpointAdjacencyPosture,
        checksum_scope: ChecksumCoverageBasis,
    ) -> Self {
        Self::new(
            scope,
            membership,
            root_posture,
            checkpoint_adjacency,
            checksum_scope,
        )
    }

    pub const fn scope(&self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn membership(&self) -> ManifestMembershipProof {
        self.membership
    }

    pub const fn root_posture(&self) -> RootManifestIntegrityPosture {
        self.root_posture
    }

    pub const fn checkpoint_adjacency(&self) -> CheckpointAdjacencyPosture {
        self.checkpoint_adjacency
    }

    pub const fn checksum_scope(&self) -> &ChecksumCoverageBasis {
        &self.checksum_scope
    }
}
