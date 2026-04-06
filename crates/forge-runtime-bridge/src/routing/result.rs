use crate::delivery::BridgeDeliveryReceipt;
use crate::routing::counters::BridgeRoutingCounters;
use crate::routing::lowering::{
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeSubscriptionSliceIdentity,
};
use crate::routing::outcome::BridgeRouteOutcomeReference;
use crate::routing::planning::BridgeRoutingSummary;
use crate::routing::planning::BridgeExecutionCounts;
use crate::routing::proof::BridgeRouteContractProof;
use crate::input::envelope::{TruthCommitIdentity, TruthPatchIdentity};
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteResultSummary {
    outcome: BridgeRouteOutcomeReference,
    contract_proof: BridgeRouteContractProof,
    routing_entry_count: usize,
    execution_counts: BridgeExecutionCounts,
    delivered_target_count: usize,
}

impl BridgeRouteResultSummary {
    pub(crate) fn new(
        outcome: BridgeRouteOutcomeReference,
        contract_proof: BridgeRouteContractProof,
        routing_entry_count: usize,
        execution_counts: BridgeExecutionCounts,
        delivered_target_count: usize,
    ) -> Self {
        Self {
            outcome,
            contract_proof,
            routing_entry_count,
            execution_counts,
            delivered_target_count,
        }
    }

    pub fn outcome(&self) -> &BridgeRouteOutcomeReference {
        &self.outcome
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        self.outcome.invalidation_identity()
    }

    pub fn source(&self) -> &crate::routing::BridgeRouteSourceSummary {
        self.outcome.source()
    }

    pub fn route_identity(&self) -> &crate::routing::BridgeRouteIdentity {
        self.outcome.route_identity()
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.outcome.source_commit()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.outcome.source_patch()
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.outcome.source_snapshot()
    }

    pub fn contract_proof(&self) -> &BridgeRouteContractProof {
        &self.contract_proof
    }

    pub fn producer_metadata(&self) -> &crate::input::envelope::BridgeProducerMetadata {
        self.contract_proof.producer_metadata()
    }

    pub fn mapping_context_digest(&self) -> &str {
        self.contract_proof.mapping_context_digest()
    }

    pub fn planning_provenance_digest(&self) -> &str {
        self.contract_proof.planning_provenance_digest()
    }

    pub fn planning_summary_digest(&self) -> &str {
        self.contract_proof.planning_summary_digest()
    }

    pub fn lowering_provenance_digest(&self) -> &str {
        self.contract_proof.lowering_provenance_digest()
    }

    pub fn lowering_summary_digest(&self) -> &str {
        self.contract_proof.lowering_summary_digest()
    }

    pub fn routing_entry_count(&self) -> usize {
        self.routing_entry_count
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        self.outcome.subscription_slice_identity()
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.execution_counts.snapshot_read_count()
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn delivered_target_count(&self) -> usize {
        self.delivered_target_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteResult {
    routing_summary: BridgeRoutingSummary,
    result_summary: BridgeRouteResultSummary,
    counters: BridgeRoutingCounters,
    artifact: BridgeInvalidationArtifact,
    receipt: BridgeDeliveryReceipt,
}

impl BridgeRouteResult {
    pub(crate) fn new(
        routing_summary: BridgeRoutingSummary,
        result_summary: BridgeRouteResultSummary,
        counters: BridgeRoutingCounters,
        artifact: BridgeInvalidationArtifact,
        receipt: BridgeDeliveryReceipt,
    ) -> Self {
        Self {
            routing_summary,
            result_summary,
            counters,
            artifact,
            receipt,
        }
    }

    pub fn routing_summary(&self) -> &BridgeRoutingSummary {
        &self.routing_summary
    }

    pub fn result_summary(&self) -> &BridgeRouteResultSummary {
        &self.result_summary
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }

    pub fn artifact(&self) -> &BridgeInvalidationArtifact {
        &self.artifact
    }

    pub fn receipt(&self) -> &BridgeDeliveryReceipt {
        &self.receipt
    }
}
