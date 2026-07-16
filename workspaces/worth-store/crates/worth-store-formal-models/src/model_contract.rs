use crate::ProtocolFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolLivenessContract {
    SafetyOnlyNoFairnessAssumed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FiniteAbstractionRule {
    WalBytesToDurabilityPosture,
    PagesToFlushAndGenerationPosture,
    CheckpointsToPublicationFrontier,
    RecoveryCandidatesToTypedApplicationRole,
    CompactionArtifactsToLifecycleAndVisibility,
    ReadersToLiveLeaseCardinality,
    QuarantinePayloadsToVerificationPosture,
    ImportPayloadsToReadmissionAndPublicationPosture,
    ReplicationFramesToFrontierAndLineagePosture,
    SharedIdentitiesToCrossProtocolFrontiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolModelContract {
    protocol: ProtocolFamily,
    finite_abstractions: &'static [FiniteAbstractionRule],
    liveness: ProtocolLivenessContract,
}

const DURABILITY_ABSTRACTIONS: &[FiniteAbstractionRule] = &[
    FiniteAbstractionRule::WalBytesToDurabilityPosture,
    FiniteAbstractionRule::PagesToFlushAndGenerationPosture,
    FiniteAbstractionRule::CheckpointsToPublicationFrontier,
];
const SOURCE_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::RecoveryCandidatesToTypedApplicationRole];
const COMPACTION_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::CompactionArtifactsToLifecycleAndVisibility];
const LEASE_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::ReadersToLiveLeaseCardinality];
const QUARANTINE_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::QuarantinePayloadsToVerificationPosture];
const IMPORT_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::ImportPayloadsToReadmissionAndPublicationPosture];
const REPLICATION_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::ReplicationFramesToFrontierAndLineagePosture];
const SHARED_ABSTRACTIONS: &[FiniteAbstractionRule] =
    &[FiniteAbstractionRule::SharedIdentitiesToCrossProtocolFrontiers];

pub const fn protocol_model_contract(protocol: ProtocolFamily) -> ProtocolModelContract {
    let finite_abstractions = match protocol {
        ProtocolFamily::DurabilityRecovery => DURABILITY_ABSTRACTIONS,
        ProtocolFamily::RecoverySourcePrecedence => SOURCE_ABSTRACTIONS,
        ProtocolFamily::CompactionVisibility => COMPACTION_ABSTRACTIONS,
        ProtocolFamily::LeaseReclaim => LEASE_ABSTRACTIONS,
        ProtocolFamily::QuarantineReadmission => QUARANTINE_ABSTRACTIONS,
        ProtocolFamily::ImportPublication => IMPORT_ABSTRACTIONS,
        ProtocolFamily::ReplicationAdmission => REPLICATION_ABSTRACTIONS,
        ProtocolFamily::SharedFrontiers => SHARED_ABSTRACTIONS,
    };
    ProtocolModelContract {
        protocol,
        finite_abstractions,
        liveness: ProtocolLivenessContract::SafetyOnlyNoFairnessAssumed,
    }
}

impl FiniteAbstractionRule {
    pub const fn collapsed_runtime_detail(self) -> &'static str {
        match self {
            Self::WalBytesToDurabilityPosture => "frame bytes, offsets, and segment lengths",
            Self::PagesToFlushAndGenerationPosture => "page payloads and concrete page identities",
            Self::CheckpointsToPublicationFrontier => "checkpoint artifact payloads and locators",
            Self::RecoveryCandidatesToTypedApplicationRole => {
                "candidate payloads while retaining discovery order and application role"
            }
            Self::CompactionArtifactsToLifecycleAndVisibility => {
                "run contents while retaining cutover, recovery, and reclaim posture"
            }
            Self::ReadersToLiveLeaseCardinality => {
                "reader payloads while retaining live-lease and generation identity"
            }
            Self::QuarantinePayloadsToVerificationPosture => {
                "damaged bytes while retaining scope, verification, and authority posture"
            }
            Self::ImportPayloadsToReadmissionAndPublicationPosture => {
                "import bytes while retaining readmission and durable-publication posture"
            }
            Self::ReplicationFramesToFrontierAndLineagePosture => {
                "replicated bytes while retaining frontier, epoch, lineage, and durability"
            }
            Self::SharedIdentitiesToCrossProtocolFrontiers => {
                "family-local payloads while retaining shared authority frontiers"
            }
        }
    }

    pub const fn preserved_protocol_truth(self) -> &'static str {
        match self {
            Self::WalBytesToDurabilityPosture => "acknowledgment never precedes the required fence",
            Self::PagesToFlushAndGenerationPosture => {
                "uncertain or stale pages never become recovered authority"
            }
            Self::CheckpointsToPublicationFrontier => {
                "selection requires durable namespace publication"
            }
            Self::RecoveryCandidatesToTypedApplicationRole => {
                "only admitted authority candidates may be selected"
            }
            Self::CompactionArtifactsToLifecycleAndVisibility => {
                "visibility and reclaim remain ordered behind cutover and readers"
            }
            Self::ReadersToLiveLeaseCardinality => {
                "live or expired-without-authority leases block reclaim and reuse"
            }
            Self::QuarantinePayloadsToVerificationPosture => {
                "readmission requires verification and current authority"
            }
            Self::ImportPayloadsToReadmissionAndPublicationPosture => {
                "raw or pending imports are never durable publications"
            }
            Self::ReplicationFramesToFrontierAndLineagePosture => {
                "divergence cannot advance or publish a peer frontier"
            }
            Self::SharedIdentitiesToCrossProtocolFrontiers => {
                "durability, visibility, reachability, quarantine, and admission stay ordered"
            }
        }
    }
}

impl ProtocolModelContract {
    pub const fn protocol(self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn finite_abstractions(self) -> &'static [FiniteAbstractionRule] {
        self.finite_abstractions
    }

    pub const fn liveness(self) -> ProtocolLivenessContract {
        self.liveness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_checked_family_declares_finite_collapses_and_no_liveness_claim() {
        for protocol in ProtocolFamily::all() {
            let contract = protocol_model_contract(protocol);
            assert_eq!(contract.protocol(), protocol);
            assert!(!contract.finite_abstractions().is_empty());
            assert_eq!(
                contract.liveness(),
                ProtocolLivenessContract::SafetyOnlyNoFairnessAssumed
            );
            for rule in contract.finite_abstractions() {
                assert!(!rule.collapsed_runtime_detail().is_empty());
                assert!(!rule.preserved_protocol_truth().is_empty());
            }
        }
    }
}
