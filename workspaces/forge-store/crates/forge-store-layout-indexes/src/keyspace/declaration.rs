use crate::catalog::ArtifactFamilyDenial;
use crate::catalog::ArtifactScopePartitionWitness;
use forge_store_contracts::DurableArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalKeyDomain {
    RootManifestKey,
    PageAddressKey,
    SegmentAddressKey,
    ExtentAddressKey,
    PhysicalReferenceKey,
    WalRecordKey,
    BlobIdentityKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalKeyDomainWitness {
    scope: ArtifactScopePartitionWitness,
    domain: PhysicalKeyDomain,
}

impl PhysicalKeyDomainWitness {
    pub(crate) const fn new(
        scope: ArtifactScopePartitionWitness,
        domain: PhysicalKeyDomain,
    ) -> Self {
        Self { scope, domain }
    }

    pub const fn scope(self) -> ArtifactScopePartitionWitness {
        self.scope
    }

    pub const fn family_id(self) -> DurableArtifactFamilyId {
        self.scope.family_id()
    }

    pub const fn domain(self) -> PhysicalKeyDomain {
        self.domain
    }
}

pub(crate) const fn declare_physical_key_domain(
    scope: ArtifactScopePartitionWitness,
) -> Result<PhysicalKeyDomainWitness, ArtifactFamilyDenial> {
    use DurableArtifactFamilyId as Family;

    let domain = match scope.family_id() {
        Family::PhysicalRootManifest => PhysicalKeyDomain::RootManifestKey,
        Family::PhysicalPage => PhysicalKeyDomain::PageAddressKey,
        Family::PhysicalSegment => PhysicalKeyDomain::SegmentAddressKey,
        Family::PhysicalExtent => PhysicalKeyDomain::ExtentAddressKey,
        Family::WalDurableMutationIntent
        | Family::WalHostedRuntimeCommitResult
        | Family::WalBulkCheckpointPublicationIntent
        | Family::WalDurablePublicationProgress
        | Family::WalRecoveryDecision
        | Family::PublicationWalIntent
        | Family::PublicationWalCanonicalResult
        | Family::PublicationWalPublicationProgress => PhysicalKeyDomain::WalRecordKey,
        Family::ReachabilityEdge => PhysicalKeyDomain::PhysicalReferenceKey,
        Family::BlobChunk
        | Family::BlobManifest
        | Family::BlobStream
        | Family::ChunkTreeRoot
        | Family::DedupeIndex => PhysicalKeyDomain::BlobIdentityKey,
        _ => return Err(ArtifactFamilyDenial::PhysicalKeyDomainNotDeclaredForFamily),
    };

    Ok(PhysicalKeyDomainWitness::new(scope, domain))
}
