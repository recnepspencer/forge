use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::input::envelope::{TruthBranchIdentity, TruthCommitIdentity};
use crate::snapshot::{
    BridgeTruthViewSelector, HistoricalEvaluationDeclaration, ResolvedTruthViewPolicy,
    SnapshotReadPacket, TruthSnapshotIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTruthViewAuthorityBasis {
    branch_identity: TruthBranchIdentity,
    commit_identity: Option<TruthCommitIdentity>,
    snapshot_identity: Option<TruthSnapshotIdentity>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTruthViewAuthorityBasis {
    pub(crate) fn from_selector(selector: &BridgeTruthViewSelector) -> Self {
        Self::from_resolved(
            selector,
            selector.commit_identity().cloned(),
            selector.snapshot_identity().cloned(),
        )
    }

    pub(crate) fn from_resolved_envelope(
        selector: &BridgeTruthViewSelector,
        commit_identity: TruthCommitIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self::from_resolved(selector, Some(commit_identity), Some(snapshot_identity))
    }

    fn from_resolved(
        selector: &BridgeTruthViewSelector,
        commit_identity: Option<TruthCommitIdentity>,
        snapshot_identity: Option<TruthSnapshotIdentity>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "truth-view-authority|selector={}|selector-basis={}|branch={}|commit={}|snapshot={}",
            selector.selector_identity().as_str(),
            selector.canonical_basis(),
            selector.branch_identity().as_str(),
            commit_identity
                .as_ref()
                .map(TruthCommitIdentity::as_str)
                .unwrap_or("-"),
            snapshot_identity
                .as_ref()
                .map(TruthSnapshotIdentity::as_str)
                .unwrap_or("-"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            branch_identity: selector.branch_identity().clone(),
            commit_identity,
            snapshot_identity,
            canonical_basis,
            digest: Arc::from(format!("truth-view-authority:sha256:{digest:x}")),
        }
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.branch_identity
    }

    pub fn commit_identity(&self) -> Option<&TruthCommitIdentity> {
        self.commit_identity.as_ref()
    }

    pub fn snapshot_identity(&self) -> Option<&TruthSnapshotIdentity> {
        self.snapshot_identity.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedTruthViewPacket {
    declaration: HistoricalEvaluationDeclaration,
    resolved_policy: ResolvedTruthViewPolicy,
    authority_basis: BridgeTruthViewAuthorityBasis,
    read_packet: SnapshotReadPacket,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PlannedTruthViewPacket {
    pub(crate) fn new(
        declaration: HistoricalEvaluationDeclaration,
        resolved_policy: ResolvedTruthViewPolicy,
        authority_basis: BridgeTruthViewAuthorityBasis,
        read_packet: SnapshotReadPacket,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "planned-truth-view-packet|declaration={}|validated-selectors={}|policy={}|authority={}|read-packet={}",
            declaration.declaration_identity().as_str(),
            declaration.validated_selector_set().digest(),
            resolved_policy.digest(),
            authority_basis.digest(),
            read_packet.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration,
            resolved_policy,
            authority_basis,
            read_packet,
            canonical_basis,
            digest: Arc::from(format!("planned-truth-view-packet:sha256:{digest:x}")),
        }
    }

    pub fn declaration(&self) -> &HistoricalEvaluationDeclaration {
        &self.declaration
    }

    pub fn resolved_policy(&self) -> &ResolvedTruthViewPolicy {
        &self.resolved_policy
    }

    pub fn authority_basis(&self) -> &BridgeTruthViewAuthorityBasis {
        &self.authority_basis
    }

    pub fn read_packet(&self) -> &SnapshotReadPacket {
        &self.read_packet
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{BridgeTruthViewAuthorityBasis, PlannedTruthViewPacket};
    use crate::input::envelope::TruthBranchIdentity;
    use crate::policy::BridgeDiagnosticsTier;
    use crate::snapshot::{
        BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewSelector,
        HistoricalEvaluationDeclaration, ResolvedTruthViewPolicy, SnapshotReadPacket,
        TruthSnapshotIdentity, TruthViewReplayCompatibility, TruthViewRetentionAdmission,
        TruthViewSourceCapability,
    };

    #[test]
    fn authority_basis_is_canonical_for_same_selector() {
        let selector = BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        );

        let left = BridgeTruthViewAuthorityBasis::from_selector(&selector);
        let right = BridgeTruthViewAuthorityBasis::from_selector(&selector);

        assert_eq!(left, right);
        assert!(left.canonical_basis().contains("branch=analysis"));
    }

    #[test]
    fn planned_truth_view_packet_is_canonical_for_same_inputs() {
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let policy = ResolvedTruthViewPolicy::admitted(
            &declaration,
            TruthViewRetentionAdmission::SnapshotResident,
            TruthViewSourceCapability::DirectSnapshotRead,
            TruthViewReplayCompatibility::ReplayPermitted,
        );
        let authority = BridgeTruthViewAuthorityBasis::from_selector(declaration.selector());
        let left = PlannedTruthViewPacket::new(
            declaration.clone(),
            policy.clone(),
            authority.clone(),
            SnapshotReadPacket::new(vec![]),
        );
        let right = PlannedTruthViewPacket::new(
            declaration,
            policy,
            authority,
            SnapshotReadPacket::new(vec![]),
        );

        assert_eq!(left, right);
        assert!(left
            .canonical_basis()
            .contains("read-packet=snapshot-read-packet:sha256:"));
    }
}
