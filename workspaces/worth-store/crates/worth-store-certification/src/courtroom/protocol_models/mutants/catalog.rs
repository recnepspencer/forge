use std::path::{Path, PathBuf};

use worth_store_formal_models::runner::{ProtocolCheckBounds, ProtocolCheckInvocation};
use worth_store_formal_models::ProtocolFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlledProtocolMutant {
    DurabilityAcknowledgmentBeforeFence,
    RecoveryQuarantinedSourceSelected,
    CompactionPublicationBeforeCutover,
    LeaseIdentityReuseWithLiveLease,
    QuarantineReleaseWithoutVerification,
    ImportPublicationWithoutDurability,
    ReplicationDivergenceAcceptedAsResume,
    SharedReachableAuthorityReclaimed,
}

impl ControlledProtocolMutant {
    pub const fn all() -> [Self; 8] {
        [
            Self::DurabilityAcknowledgmentBeforeFence,
            Self::RecoveryQuarantinedSourceSelected,
            Self::CompactionPublicationBeforeCutover,
            Self::LeaseIdentityReuseWithLiveLease,
            Self::QuarantineReleaseWithoutVerification,
            Self::ImportPublicationWithoutDurability,
            Self::ReplicationDivergenceAcceptedAsResume,
            Self::SharedReachableAuthorityReclaimed,
        ]
    }

    pub const fn protocol(self) -> ProtocolFamily {
        match self {
            Self::DurabilityAcknowledgmentBeforeFence => ProtocolFamily::DurabilityRecovery,
            Self::RecoveryQuarantinedSourceSelected => ProtocolFamily::RecoverySourcePrecedence,
            Self::CompactionPublicationBeforeCutover => ProtocolFamily::CompactionVisibility,
            Self::LeaseIdentityReuseWithLiveLease => ProtocolFamily::LeaseReclaim,
            Self::QuarantineReleaseWithoutVerification => ProtocolFamily::QuarantineReadmission,
            Self::ImportPublicationWithoutDurability => ProtocolFamily::ImportPublication,
            Self::ReplicationDivergenceAcceptedAsResume => ProtocolFamily::ReplicationAdmission,
            Self::SharedReachableAuthorityReclaimed => ProtocolFamily::SharedFrontiers,
        }
    }

    pub const fn certification_lane(self) -> &'static str {
        match self {
            Self::DurabilityAcknowledgmentBeforeFence => {
                "protocol.durability.acknowledgment.mutant"
            }
            Self::RecoveryQuarantinedSourceSelected => "protocol.recovery.source-precedence.mutant",
            Self::CompactionPublicationBeforeCutover => "protocol.compaction.visibility.mutant",
            Self::LeaseIdentityReuseWithLiveLease => "protocol.lease.reuse.mutant",
            Self::QuarantineReleaseWithoutVerification => "protocol.quarantine.readmission.mutant",
            Self::ImportPublicationWithoutDurability => "protocol.import.durability.mutant",
            Self::ReplicationDivergenceAcceptedAsResume => "protocol.replication.divergence.mutant",
            Self::SharedReachableAuthorityReclaimed => "protocol.shared-frontiers.reclaim.mutant",
        }
    }

    pub fn invocation(self, bounds: ProtocolCheckBounds) -> ProtocolCheckInvocation {
        let (model, configuration) = self.artifact_paths();
        ProtocolCheckInvocation::for_controlled_defect(
            self.protocol(),
            model,
            configuration,
            bounds,
        )
    }

    fn artifact_paths(self) -> (PathBuf, PathBuf) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("certification crate lives under workspace crates")
            .join("worth-store-formal-models")
            .join("src")
            .join("protocols")
            .join(self.artifact_directory());
        (
            root.join(format!("{}.tla", self.artifact_stem())),
            root.join(format!("{}.cfg", self.artifact_stem())),
        )
    }

    const fn artifact_directory(self) -> &'static str {
        match self {
            Self::DurabilityAcknowledgmentBeforeFence => "durability_recovery",
            Self::RecoveryQuarantinedSourceSelected => "source_precedence",
            Self::CompactionPublicationBeforeCutover => "compaction_visibility",
            Self::LeaseIdentityReuseWithLiveLease => "lease_reclaim",
            Self::QuarantineReleaseWithoutVerification => "quarantine_readmission",
            Self::ImportPublicationWithoutDurability => "import_publication",
            Self::ReplicationDivergenceAcceptedAsResume => "replication_admission",
            Self::SharedReachableAuthorityReclaimed => "shared_frontiers",
        }
    }

    const fn artifact_stem(self) -> &'static str {
        match self {
            Self::DurabilityAcknowledgmentBeforeFence => "DurabilityAcknowledgmentMutant",
            Self::RecoveryQuarantinedSourceSelected => "SourcePrecedenceMutant",
            Self::CompactionPublicationBeforeCutover => "CompactionPublicationMutant",
            Self::LeaseIdentityReuseWithLiveLease => "LeaseReuseMutant",
            Self::QuarantineReleaseWithoutVerification => "QuarantineReleaseMutant",
            Self::ImportPublicationWithoutDurability => "ImportDurabilityMutant",
            Self::ReplicationDivergenceAcceptedAsResume => "ReplicationDivergenceMutant",
            Self::SharedReachableAuthorityReclaimed => "SharedReclaimMutant",
        }
    }
}
