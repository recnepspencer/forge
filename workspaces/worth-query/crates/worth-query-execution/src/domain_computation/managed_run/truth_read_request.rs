use worth_relational::facade::branch::RelationalBranchBasisDescriptor;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, SnapshotReadPacket,
};

pub struct WorthQueryManagedTruthReadRequest {
    relational_basis: RelationalBranchBasisDescriptor,
    packet: SnapshotReadPacket,
    replay_mode: BridgeReplayMode,
    diagnostics_tier: BridgeDiagnosticsTier,
    delivery_intent: BridgeDeliveryIntent,
}

impl WorthQueryManagedTruthReadRequest {
    pub fn new(
        relational_basis: RelationalBranchBasisDescriptor,
        packet: SnapshotReadPacket,
    ) -> Self {
        Self {
            relational_basis,
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
        RelationalBranchBasisDescriptor,
        SnapshotReadPacket,
        BridgeReplayMode,
        BridgeDiagnosticsTier,
        BridgeDeliveryIntent,
    ) {
        (
            self.relational_basis,
            self.packet,
            self.replay_mode,
            self.diagnostics_tier,
            self.delivery_intent,
        )
    }
}

#[cfg(test)]
mod tests {
    use worth_runtime_bridge::facade::{BridgeReplayMode, SnapshotReadPacket};

    use super::WorthQueryManagedTruthReadRequest;

    #[test]
    fn ordinary_managed_truth_requests_disable_replay_by_default() {
        let runtime = worth_relational::facade::runtime::RelationalRuntimeApi::builder().build();
        let identity = runtime.main_branch_identity();
        let (descriptor, _) = runtime.observe_branch(&identity).unwrap();
        let (_, _, replay, _, _) =
            WorthQueryManagedTruthReadRequest::new(descriptor, SnapshotReadPacket::new(Vec::new()))
                .into_parts();

        assert_eq!(replay, BridgeReplayMode::Disabled);
    }
}
