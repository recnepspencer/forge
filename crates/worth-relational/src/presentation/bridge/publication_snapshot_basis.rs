pub(super) enum RelationalBridgeSnapshotBasis {
    HistoricalCommitIdentity,
    ExactObservation(worth_runtime_bridge::facade::TruthSnapshotIdentity),
}

impl RelationalBridgeSnapshotBasis {
    pub(super) fn resolve(
        self,
        envelope: &crate::history::data::CanonicalCommitEnvelope,
    ) -> worth_runtime_bridge::facade::TruthSnapshotIdentity {
        match self {
            Self::HistoricalCommitIdentity => {
                super::identities::bridge_snapshot_identity_for_commit(
                    envelope.commit.commit_id,
                    envelope.commit.version_id,
                )
            }
            Self::ExactObservation(identity) => identity,
        }
    }
}
