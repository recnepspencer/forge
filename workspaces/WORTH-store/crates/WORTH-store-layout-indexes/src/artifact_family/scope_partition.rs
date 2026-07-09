use super::{ArtifactDerivedAccuracyWitness, ArtifactFamilyDenial};
use worth_store_security::{
    admit_layout_access_security_boundary, StoreAuthenticityRequirement,
    StoreCurrentSecurityScopeWitnessSet, StoreCustodyPosture, StoreKeyScope,
    StoreLayoutAccessSecurityBoundaryWitness, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactTenantScopePartition {
    Single(StoreTenantScope),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKeyScopePartition {
    Single(StoreKeyScope),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactScopePartitionWitness {
    accuracy: ArtifactDerivedAccuracyWitness,
    tenant_partition: ArtifactTenantScopePartition,
    key_partition: ArtifactKeyScopePartition,
    required_authenticity: StoreAuthenticityRequirement,
    required_custody_posture: StoreCustodyPosture,
    security_boundary: StoreLayoutAccessSecurityBoundaryWitness,
}

impl ArtifactScopePartitionWitness {
    pub(crate) const fn new(
        accuracy: ArtifactDerivedAccuracyWitness,
        tenant_partition: ArtifactTenantScopePartition,
        key_partition: ArtifactKeyScopePartition,
        required_authenticity: StoreAuthenticityRequirement,
        required_custody_posture: StoreCustodyPosture,
        security_boundary: StoreLayoutAccessSecurityBoundaryWitness,
    ) -> Self {
        Self {
            accuracy,
            tenant_partition,
            key_partition,
            required_authenticity,
            required_custody_posture,
            security_boundary,
        }
    }

    pub const fn accuracy(self) -> ArtifactDerivedAccuracyWitness {
        self.accuracy
    }

    pub const fn family_id(self) -> worth_store_contracts::DurableArtifactFamilyId {
        self.accuracy.family_id()
    }

    pub const fn tenant_partition(self) -> ArtifactTenantScopePartition {
        self.tenant_partition
    }

    pub const fn key_partition(self) -> ArtifactKeyScopePartition {
        self.key_partition
    }

    pub const fn required_authenticity(self) -> StoreAuthenticityRequirement {
        self.required_authenticity
    }

    pub const fn required_custody_posture(self) -> StoreCustodyPosture {
        self.required_custody_posture
    }

    pub const fn security_boundary(self) -> StoreLayoutAccessSecurityBoundaryWitness {
        self.security_boundary
    }

    pub const fn admitted_tenant_scope(self) -> StoreTenantScope {
        self.security_boundary.tenant_scope()
    }

    pub const fn admitted_key_scope(self) -> StoreKeyScope {
        self.security_boundary.key_scope()
    }
}

pub(crate) fn require_scope_partition(
    accuracy: ArtifactDerivedAccuracyWitness,
    security_scope: &StoreCurrentSecurityScopeWitnessSet,
) -> Result<ArtifactScopePartitionWitness, ArtifactFamilyDenial> {
    let tenant_partition = declared_tenant_partition(accuracy);
    let key_partition = declared_key_partition(accuracy);
    let required_authenticity = declared_authenticity_requirement(accuracy);
    let required_custody_posture = declared_custody_posture(accuracy);
    let security_boundary = admit_layout_access_security_boundary(security_scope);

    if !tenant_partition_allows(tenant_partition, security_boundary.tenant_scope()) {
        return Err(ArtifactFamilyDenial::CrossTenantScopePartitionDenied);
    }

    if !key_partition_allows(key_partition, security_boundary.key_scope()) {
        return Err(ArtifactFamilyDenial::CrossKeyScopePartitionDenied);
    }

    if security_boundary.authenticity_requirement() != required_authenticity {
        return Err(ArtifactFamilyDenial::AuthenticityBoundaryDenied);
    }

    if security_boundary.custody_posture() != required_custody_posture {
        return Err(ArtifactFamilyDenial::CustodyBoundaryDenied);
    }

    Ok(ArtifactScopePartitionWitness::new(
        accuracy,
        tenant_partition,
        key_partition,
        required_authenticity,
        required_custody_posture,
        security_boundary,
    ))
}

const fn declared_tenant_partition(
    accuracy: ArtifactDerivedAccuracyWitness,
) -> ArtifactTenantScopePartition {
    use worth_store_contracts::DurableArtifactFamilyId as Family;

    match accuracy.family_id() {
        Family::PhysicalRootManifest
        | Family::WalDurableMutationIntent
        | Family::WalHostedRuntimeCommitResult
        | Family::WalBulkCheckpointPublicationIntent
        | Family::WalDurablePublicationProgress
        | Family::WalRecoveryDecision
        | Family::PublicationWalIntent
        | Family::PublicationWalCanonicalResult
        | Family::PublicationWalPublicationProgress => {
            ArtifactTenantScopePartition::Single(StoreTenantScope::StoreInternal)
        }
        Family::PhysicalPage
        | Family::PhysicalSegment
        | Family::PhysicalExtent
        | Family::BlobChunk
        | Family::BlobManifest
        | Family::BlobStream
        | Family::ChunkTreeRoot
        | Family::DedupeIndex
        | Family::ReachabilityEdge
        | Family::SecurityCustodyLookup => {
            ArtifactTenantScopePartition::Single(StoreTenantScope::TenantPhysicalBoundary)
        }
        Family::QuarantineRecord | Family::RepairRecord => {
            ArtifactTenantScopePartition::Single(StoreTenantScope::RepairBlastRadius)
        }
        Family::ReadmissionRecord | Family::ImportBundle => {
            ArtifactTenantScopePartition::Single(StoreTenantScope::ImportReadmissionBoundary)
        }
        Family::ExportBundle | Family::CapsuleArtifact => {
            ArtifactTenantScopePartition::Single(StoreTenantScope::BackupRestoreBoundary)
        }
        _ => ArtifactTenantScopePartition::Single(StoreTenantScope::StoreInternal),
    }
}

const fn declared_key_partition(
    accuracy: ArtifactDerivedAccuracyWitness,
) -> ArtifactKeyScopePartition {
    use worth_store_contracts::DurableArtifactFamilyId as Family;

    match accuracy.family_id() {
        Family::PhysicalRootManifest => {
            ArtifactKeyScopePartition::Single(StoreKeyScope::StoreManagedRoot)
        }
        Family::PhysicalPage | Family::PhysicalSegment | Family::PhysicalExtent => {
            ArtifactKeyScopePartition::Single(StoreKeyScope::PageEnvelope)
        }
        Family::WalDurableMutationIntent
        | Family::WalHostedRuntimeCommitResult
        | Family::WalBulkCheckpointPublicationIntent
        | Family::WalDurablePublicationProgress
        | Family::WalRecoveryDecision
        | Family::PublicationWalIntent
        | Family::PublicationWalCanonicalResult
        | Family::PublicationWalPublicationProgress => {
            ArtifactKeyScopePartition::Single(StoreKeyScope::WalCheckpointEnvelope)
        }
        Family::BlobChunk
        | Family::BlobManifest
        | Family::BlobStream
        | Family::ChunkTreeRoot
        | Family::DedupeIndex => {
            ArtifactKeyScopePartition::Single(StoreKeyScope::BlobChunkEnvelope)
        }
        Family::QuarantineRecord | Family::RepairRecord => {
            ArtifactKeyScopePartition::Single(StoreKeyScope::RepairScopeEnvelope)
        }
        Family::ExportBundle | Family::ImportBundle | Family::CapsuleArtifact => {
            ArtifactKeyScopePartition::Single(StoreKeyScope::BackupExportEnvelope)
        }
        _ => ArtifactKeyScopePartition::Single(StoreKeyScope::ArtifactEnvelope),
    }
}

const fn declared_authenticity_requirement(
    accuracy: ArtifactDerivedAccuracyWitness,
) -> StoreAuthenticityRequirement {
    use worth_store_contracts::DurableArtifactFamilyId as Family;
    use worth_store_security::StoreAuthenticityRequirementClass as Class;

    match accuracy.family_id() {
        Family::PhysicalPage | Family::PhysicalSegment | Family::PhysicalExtent => {
            StoreAuthenticityRequirement::required(Class::AuthenticatedFrame)
        }
        Family::WalDurableMutationIntent
        | Family::WalHostedRuntimeCommitResult
        | Family::WalBulkCheckpointPublicationIntent
        | Family::WalDurablePublicationProgress
        | Family::WalRecoveryDecision
        | Family::PublicationWalIntent
        | Family::PublicationWalCanonicalResult
        | Family::PublicationWalPublicationProgress => {
            StoreAuthenticityRequirement::required(Class::AuthenticatedWalRecord)
        }
        Family::BlobChunk
        | Family::BlobManifest
        | Family::BlobStream
        | Family::ChunkTreeRoot
        | Family::DedupeIndex => {
            StoreAuthenticityRequirement::required(Class::AuthenticatedBlobChunk)
        }
        Family::ExportBundle | Family::CapsuleArtifact => {
            StoreAuthenticityRequirement::required(Class::AuthenticatedBackupCapsule)
        }
        Family::QuarantineRecord | Family::RepairRecord => {
            StoreAuthenticityRequirement::required(Class::AuthenticatedRepairRead)
        }
        _ => StoreAuthenticityRequirement::not_required(),
    }
}

const fn declared_custody_posture(accuracy: ArtifactDerivedAccuracyWitness) -> StoreCustodyPosture {
    use worth_store_contracts::DurableArtifactFamilyId as Family;

    match accuracy.family_id() {
        Family::ExportBundle | Family::CapsuleArtifact => StoreCustodyPosture::ExportPrepared,
        Family::ImportBundle | Family::ReadmissionRecord => {
            StoreCustodyPosture::ImportedUnreadmitted
        }
        _ => StoreCustodyPosture::InternalStoreCustody,
    }
}

fn tenant_partition_allows(
    partition: ArtifactTenantScopePartition,
    actual: StoreTenantScope,
) -> bool {
    match partition {
        ArtifactTenantScopePartition::Single(expected) => expected == actual,
    }
}

fn key_partition_allows(partition: ArtifactKeyScopePartition, actual: StoreKeyScope) -> bool {
    match partition {
        ArtifactKeyScopePartition::Single(expected) => expected == actual,
    }
}
