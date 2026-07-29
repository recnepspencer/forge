use worth_relational::facade::identity::VersionId;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, SnapshotReadPacket,
    TruthBranchIdentity,
};

pub struct WorthQueryManagedTruthReadRequest {
    relational_version_id: VersionId,
    branch: TruthBranchIdentity,
    packet: SnapshotReadPacket,
    replay_mode: BridgeReplayMode,
    diagnostics_tier: BridgeDiagnosticsTier,
    delivery_intent: BridgeDeliveryIntent,
}

impl WorthQueryManagedTruthReadRequest {
    pub fn new(
        relational_version_id: VersionId,
        branch: TruthBranchIdentity,
        packet: SnapshotReadPacket,
    ) -> Self {
        Self {
            relational_version_id,
            branch,
            packet,
            replay_mode: BridgeReplayMode::Disabled,
            diagnostics_tier: BridgeDiagnosticsTier::Standard,
            delivery_intent: BridgeDeliveryIntent::PrepareSignalEvaluation,
        }
    }

    pub fn with_replay_mode(mut self, replay_mode: BridgeReplayMode) -> Self {
        self.replay_mode = replay_mode;
        self
    }

    pub fn with_diagnostics_tier(mut self, diagnostics_tier: BridgeDiagnosticsTier) -> Self {
        self.diagnostics_tier = diagnostics_tier;
        self
    }

    pub fn with_delivery_intent(mut self, delivery_intent: BridgeDeliveryIntent) -> Self {
        self.delivery_intent = delivery_intent;
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VersionId,
        TruthBranchIdentity,
        SnapshotReadPacket,
        BridgeReplayMode,
        BridgeDiagnosticsTier,
        BridgeDeliveryIntent,
    ) {
        (
            self.relational_version_id,
            self.branch,
            self.packet,
            self.replay_mode,
            self.diagnostics_tier,
            self.delivery_intent,
        )
    }
}

#[cfg(test)]
mod tests {
    use worth_runtime_bridge::facade::{BridgeReplayMode, SnapshotReadPacket, TruthBranchIdentity};

    use super::WorthQueryManagedTruthReadRequest;

    #[test]
    fn ordinary_managed_truth_requests_disable_replay_by_default() {
        let (_, _, _, replay, _, _) = WorthQueryManagedTruthReadRequest::new(
            worth_relational::facade::identity::VersionId(7),
            TruthBranchIdentity::from_relational_branch_id("main"),
            SnapshotReadPacket::new(Vec::new()),
        )
        .into_parts();

        assert_eq!(replay, BridgeReplayMode::Disabled);
    }
}
