use super::{
    ArtifactFamilyAuthorityDisposition, ArtifactFamilyClassification,
    ArtifactFamilyLifecycleDisposition,
};
use forge_store_contracts::DurableArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityRole {
    SemanticAuthorityConsumer,
    PhysicalDiscoveryAuthority,
    AllocationAuthority,
    RecoveryAuthority,
    CustodyEvidenceAuthority,
    PerformanceEvidenceAuthority,
    TerminalTransportEvidence,
    CertificationEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactAuthorityRoleWitness {
    classification: ArtifactFamilyClassification,
    role: AuthorityRole,
}

impl ArtifactAuthorityRoleWitness {
    pub(crate) const fn new(
        classification: ArtifactFamilyClassification,
        role: AuthorityRole,
    ) -> Self {
        Self {
            classification,
            role,
        }
    }

    pub const fn classification(self) -> ArtifactFamilyClassification {
        self.classification
    }

    pub const fn family_id(self) -> DurableArtifactFamilyId {
        self.classification.family_id()
    }

    pub const fn role(self) -> AuthorityRole {
        self.role
    }
}

pub(crate) fn declare_authority_role(
    classification: ArtifactFamilyClassification,
) -> ArtifactAuthorityRoleWitness {
    let role = if classification.authority() == ArtifactFamilyAuthorityDisposition::Certification {
        AuthorityRole::CertificationEvidence
    } else {
        match classification.family_id() {
            DurableArtifactFamilyId::PhysicalRootManifest => {
                AuthorityRole::PhysicalDiscoveryAuthority
            }
            DurableArtifactFamilyId::PhysicalPage
            | DurableArtifactFamilyId::PhysicalSegment
            | DurableArtifactFamilyId::PhysicalExtent
            | DurableArtifactFamilyId::BlobChunk
            | DurableArtifactFamilyId::BlobManifest
            | DurableArtifactFamilyId::BlobStream => AuthorityRole::AllocationAuthority,
            DurableArtifactFamilyId::WalDurableMutationIntent
            | DurableArtifactFamilyId::WalHostedRuntimeCommitResult
            | DurableArtifactFamilyId::WalBulkCheckpointPublicationIntent
            | DurableArtifactFamilyId::WalDurablePublicationProgress
            | DurableArtifactFamilyId::WalRecoveryDecision
            | DurableArtifactFamilyId::QuarantineRecord
            | DurableArtifactFamilyId::RepairRecord
            | DurableArtifactFamilyId::ReadmissionRecord
            | DurableArtifactFamilyId::PublicationWalIntent
            | DurableArtifactFamilyId::PublicationWalCanonicalResult
            | DurableArtifactFamilyId::PublicationWalPublicationProgress
            | DurableArtifactFamilyId::PublicationAuthoritativeCommitAppendUnit
            | DurableArtifactFamilyId::PublicationBranchHeadPublication
            | DurableArtifactFamilyId::SnapshotArtifact
            | DurableArtifactFamilyId::BranchDeltaArtifact => AuthorityRole::RecoveryAuthority,
            DurableArtifactFamilyId::SecurityCustodyLookup
            | DurableArtifactFamilyId::ImportBundle => AuthorityRole::CustodyEvidenceAuthority,
            DurableArtifactFamilyId::ReclaimReceipt
            | DurableArtifactFamilyId::ResidencyRecord
            | DurableArtifactFamilyId::BackgroundPacingRecord
            | DurableArtifactFamilyId::ForegroundInterferenceRecord => {
                AuthorityRole::PerformanceEvidenceAuthority
            }
            DurableArtifactFamilyId::ExportBundle | DurableArtifactFamilyId::CapsuleArtifact => {
                AuthorityRole::TerminalTransportEvidence
            }
            _ if classification.lifecycle()
                == ArtifactFamilyLifecycleDisposition::TransferBoundaryOnly =>
            {
                AuthorityRole::TerminalTransportEvidence
            }
            _ => AuthorityRole::SemanticAuthorityConsumer,
        }
    };

    ArtifactAuthorityRoleWitness::new(classification, role)
}
