use crate::snapshot::{AdmittedSnapshotContext, SnapshotReadPacket, TruthSnapshotReader};

pub struct BridgeSignalEvaluationRequest {
    artifact: crate::routing::BridgeInvalidationArtifact,
    read_packet: SnapshotReadPacket,
    snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>,
}

impl BridgeSignalEvaluationRequest {
    pub(crate) fn new(
        artifact: crate::routing::BridgeInvalidationArtifact,
        read_packet: SnapshotReadPacket,
        snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>,
    ) -> Self {
        Self {
            artifact,
            read_packet,
            snapshot,
        }
    }

    pub fn artifact(&self) -> &crate::routing::BridgeInvalidationArtifact {
        &self.artifact
    }

    pub fn read_packet(&self) -> &SnapshotReadPacket {
        &self.read_packet
    }

    pub fn snapshot(&self) -> &AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>> {
        &self.snapshot
    }
}

pub struct BridgePreparedDeliveryRequest {
    inner: crate::routing::planning::BridgePreparedDelivery,
}

impl BridgePreparedDeliveryRequest {
    pub(crate) fn new(prepared: crate::routing::planning::BridgePreparedDelivery) -> Self {
        Self { inner: prepared }
    }

    pub fn contract_proof(&self) -> &crate::routing::BridgeRouteContractProof {
        self.inner.contract_proof()
    }

    pub fn routing_summary(&self) -> &crate::routing::BridgeRoutingSummary {
        self.inner.routing_summary()
    }

    pub fn counters(&self) -> &crate::routing::BridgeRoutingCounters {
        self.inner.counters()
    }

    pub fn read_packet(&self) -> &SnapshotReadPacket {
        self.inner.read_packet()
    }

    pub(crate) fn validated_lowering_plan(
        &self,
    ) -> &crate::routing::lowering::ValidatedBridgeLoweringPlan {
        self.inner.validated_lowering_plan()
    }

    pub(crate) fn into_inner(self) -> crate::routing::planning::BridgePreparedDelivery {
        self.inner
    }

    pub(crate) fn failure_source(&self) -> crate::diagnostics::BridgeFailureSource {
        self.inner.failure_source()
    }
}
