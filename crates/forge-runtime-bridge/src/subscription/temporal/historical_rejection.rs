use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeSubscriptionCounters, BridgeSubscriptionHistoricalTemporalReplayBasisIdentity,
    BridgeSubscriptionTemporalAdmissionIdentity,
};

use super::family::BridgeTemporalSubscriptionFamilyKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeHistoricalTemporalReplayRejectionKind {
    TemporalTruthBasisNotHistorical,
    TemporalAdmissionFamilyNotHistoricalReplay,
    TemporalBasisIdentityMismatch,
    HistoricalTruthSnapshotIdentityMismatch,
    HistoricalTruthBranchIdentityMismatch,
    MissingPreviousValueEvidence,
    PreviousValueEvidenceBranchMismatch,
    PreviousValueEvidenceSnapshotMismatch,
}

impl BridgeHistoricalTemporalReplayRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemporalTruthBasisNotHistorical => "temporal_truth_basis_not_historical",
            Self::TemporalAdmissionFamilyNotHistoricalReplay => {
                "temporal_admission_family_not_historical_replay"
            }
            Self::TemporalBasisIdentityMismatch => "temporal_basis_identity_mismatch",
            Self::HistoricalTruthSnapshotIdentityMismatch => {
                "historical_truth_snapshot_identity_mismatch"
            }
            Self::HistoricalTruthBranchIdentityMismatch => {
                "historical_truth_branch_identity_mismatch"
            }
            Self::MissingPreviousValueEvidence => "missing_previous_value_evidence",
            Self::PreviousValueEvidenceBranchMismatch => "previous_value_evidence_branch_mismatch",
            Self::PreviousValueEvidenceSnapshotMismatch => {
                "previous_value_evidence_snapshot_mismatch"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHistoricalTemporalReplayRejection {
    rejection_kind: BridgeHistoricalTemporalReplayRejectionKind,
    family_kind: BridgeTemporalSubscriptionFamilyKind,
    temporal_admission_identity: Arc<str>,
    replay_basis_identity: Option<BridgeSubscriptionHistoricalTemporalReplayBasisIdentity>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeHistoricalTemporalReplayRejection {
    pub(crate) fn new(
        rejection_kind: BridgeHistoricalTemporalReplayRejectionKind,
        family_kind: BridgeTemporalSubscriptionFamilyKind,
        temporal_admission_identity: &BridgeSubscriptionTemporalAdmissionIdentity,
        replay_basis_identity: Option<&BridgeSubscriptionHistoricalTemporalReplayBasisIdentity>,
        basis_fingerprint: &str,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-historical-temporal-replay-rejection|temporal-admission={}|family={}|replay-basis={}|rejection-kind={}|basis={basis_fingerprint}",
            temporal_admission_identity.as_str(),
            family_kind.as_str(),
            replay_basis_identity.map(|it| it.as_str()).unwrap_or("-"),
            rejection_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            family_kind,
            temporal_admission_identity: Arc::from(temporal_admission_identity.as_str()),
            replay_basis_identity: replay_basis_identity.cloned(),
            counters: BridgeSubscriptionCounters::from_historical_temporal_replay_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-historical-temporal-replay-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeHistoricalTemporalReplayRejectionKind {
        self.rejection_kind
    }

    pub fn family_kind(&self) -> BridgeTemporalSubscriptionFamilyKind {
        self.family_kind
    }

    pub fn temporal_admission_identity(&self) -> &str {
        self.temporal_admission_identity.as_ref()
    }

    pub fn replay_basis_identity(
        &self,
    ) -> Option<&BridgeSubscriptionHistoricalTemporalReplayBasisIdentity> {
        self.replay_basis_identity.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
