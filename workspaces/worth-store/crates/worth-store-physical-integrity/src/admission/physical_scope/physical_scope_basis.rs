use crate::{ChecksumCoverageBasis, GenerationIntegrityReport, LogicalDecodeGateIdentity};
use worth_store_physical_format::{
    CheckpointAdjacencyPosture, ManifestMembershipProof, PhysicalReferenceScope,
    PhysicalScopeFamily, RootManifestIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScopeBasis {
    checked_identity: LogicalDecodeGateIdentity,
    scope: PhysicalReferenceScope,
    membership: ManifestMembershipProof,
    root_posture: RootManifestIntegrityPosture,
    checkpoint_adjacency: CheckpointAdjacencyPosture,
    checksum_scope: ChecksumCoverageBasis,
    generation_report: GenerationIntegrityReport,
}

impl PhysicalScopeBasis {
    pub(crate) const fn new(
        checked_identity: LogicalDecodeGateIdentity,
        scope: PhysicalReferenceScope,
        membership: ManifestMembershipProof,
        root_posture: RootManifestIntegrityPosture,
        checkpoint_adjacency: CheckpointAdjacencyPosture,
        checksum_scope: ChecksumCoverageBasis,
        generation_report: GenerationIntegrityReport,
    ) -> Self {
        Self {
            checked_identity,
            scope,
            membership,
            root_posture,
            checkpoint_adjacency,
            checksum_scope,
            generation_report,
        }
    }

    pub const fn checked_identity(&self) -> &LogicalDecodeGateIdentity {
        &self.checked_identity
    }

    pub const fn scope(&self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn family(&self) -> PhysicalScopeFamily {
        self.scope.family()
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

    pub const fn generation_report(&self) -> GenerationIntegrityReport {
        self.generation_report
    }
}
