use std::sync::Arc;

use crate::clone_budget::clone_cheap;
use crate::diagnostics::{BridgeFailureSource, BridgeRouteRecordEntry};
use crate::input::envelope::{
    BridgeCommittedPatchDigest, BridgeProducerMetadata, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity,
};
use crate::routing::context::BridgeMappingContext;
use crate::routing::counters::BridgeRoutingCounters;
use crate::routing::lowering::{
    lower_validated_route, BridgeLoweringPlan, BridgeLoweringPlanSummary, BridgeLoweringProvenance,
    BridgeLoweringSummary, CanonicalInvalidationTargets, CanonicalSubscriptionSlices,
    ValidatedBridgeLoweringPlan,
};
use crate::routing::proof::BridgeRouteContractProof;
use crate::routing::scope::RouteScope;
use crate::routing::BridgeInvalidationArtifact;
use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity};

use super::super::summaries::{
    BridgePlanningProvenance, BridgePlanningSummary, BridgeRouteSourceSummary, BridgeRoutingSummary,
};
use super::super::BridgeRouteIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BridgePlanningArtifacts {
    planning_provenance: BridgePlanningProvenance,
    planning_summary: BridgePlanningSummary,
}

impl BridgePlanningArtifacts {
    pub(crate) fn new(
        planning_provenance: BridgePlanningProvenance,
        planning_summary: BridgePlanningSummary,
    ) -> Self {
        Self {
            planning_provenance,
            planning_summary,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BridgePlannedExecution {
    routing_summary: BridgeRoutingSummary,
    read_packet: SnapshotReadPacket,
    counters: BridgeRoutingCounters,
    lowering_plan: BridgeLoweringPlan,
    validated_lowering_plan: ValidatedBridgeLoweringPlan,
    route_record_entries: Arc<[BridgeRouteRecordEntry]>,
}

impl BridgePlannedExecution {
    pub(crate) fn new(
        routing_summary: BridgeRoutingSummary,
        read_packet: SnapshotReadPacket,
        counters: BridgeRoutingCounters,
        lowering_plan: BridgeLoweringPlan,
        validated_lowering_plan: ValidatedBridgeLoweringPlan,
        route_record_entries: Arc<[BridgeRouteRecordEntry]>,
    ) -> Self {
        Self {
            routing_summary,
            read_packet,
            counters,
            lowering_plan,
            validated_lowering_plan,
            route_record_entries,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlannedRoute {
    route_scope: RouteScope,
    mapping_context: BridgeMappingContext,
    route_identity: BridgeRouteIdentity,
    source: BridgeRouteSourceSummary,
    source_digest: BridgeCommittedPatchDigest,
    producer_metadata: BridgeProducerMetadata,
    planning: BridgePlanningArtifacts,
    execution: BridgePlannedExecution,
}

pub(crate) struct BridgePreparedDelivery {
    route_scope: RouteScope,
    contract_proof: BridgeRouteContractProof,
    validated_lowering_plan: ValidatedBridgeLoweringPlan,
    routing_summary: BridgeRoutingSummary,
    counters: BridgeRoutingCounters,
    read_packet: SnapshotReadPacket,
    route_record_entries: Arc<[BridgeRouteRecordEntry]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BridgeLoweredExecution {
    route_scope: RouteScope,
    contract_proof: BridgeRouteContractProof,
    routing_summary: BridgeRoutingSummary,
    counters: BridgeRoutingCounters,
    artifact: BridgeInvalidationArtifact,
    route_record_entries: Arc<[BridgeRouteRecordEntry]>,
}

impl BridgePlannedRoute {
    pub(crate) fn new(
        route_scope: RouteScope,
        mapping_context: BridgeMappingContext,
        route_identity: BridgeRouteIdentity,
        source: BridgeRouteSourceSummary,
        producer_metadata: BridgeProducerMetadata,
        source_digest: BridgeCommittedPatchDigest,
        planning: BridgePlanningArtifacts,
        execution: BridgePlannedExecution,
    ) -> Self {
        Self {
            route_scope,
            mapping_context,
            route_identity,
            source,
            producer_metadata,
            source_digest,
            planning,
            execution,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        &self.mapping_context
    }

    pub fn route_planning_policy_digest(&self) -> Option<&str> {
        self.route_scope.route_planning_policy_digest()
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
    }

    pub fn source_branch(&self) -> &TruthBranchIdentity {
        self.source.source_branch()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.source.source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.source.source_snapshot()
    }

    pub fn producer_metadata(&self) -> &BridgeProducerMetadata {
        &self.producer_metadata
    }

    pub fn routing_summary(&self) -> &BridgeRoutingSummary {
        &self.execution.routing_summary
    }

    pub fn source_digest(&self) -> &BridgeCommittedPatchDigest {
        &self.source_digest
    }

    pub fn read_packet(&self) -> &SnapshotReadPacket {
        &self.execution.read_packet
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.execution.counters
    }

    pub fn planning_provenance(&self) -> &BridgePlanningProvenance {
        &self.planning.planning_provenance
    }

    pub fn planning_summary(&self) -> &BridgePlanningSummary {
        &self.planning.planning_summary
    }

    pub fn lowering_summary(&self) -> &BridgeLoweringPlanSummary {
        self.execution.lowering_plan.summary()
    }

    pub fn subscription_slices(&self) -> &CanonicalSubscriptionSlices {
        self.execution.lowering_plan.subscription_slices()
    }

    pub fn invalidation_targets(&self) -> &CanonicalInvalidationTargets {
        self.execution.lowering_plan.invalidation_targets()
    }

    pub fn lowering_provenance(&self) -> &BridgeLoweringProvenance {
        self.execution.lowering_plan.provenance()
    }

    pub fn validated_lowering_summary(&self) -> &BridgeLoweringSummary {
        self.execution.validated_lowering_plan.summary()
    }

    pub fn route_record_entries(&self) -> &[BridgeRouteRecordEntry] {
        &self.execution.route_record_entries
    }

    pub(crate) fn into_prepared_delivery(self) -> BridgePreparedDelivery {
        let route_planning_policy = self.route_scope.route_planning_policy().cloned();
        let route_planning_policy_digest = self
            .route_scope
            .route_planning_policy_digest()
            .map(str::to_owned);
        BridgePreparedDelivery {
            route_scope: self.route_scope,
            contract_proof: BridgeRouteContractProof::new(
                self.producer_metadata,
                self.mapping_context,
                self.source_digest,
                route_planning_policy,
                route_planning_policy_digest,
                self.planning.planning_provenance.digest(),
                self.planning.planning_summary.digest(),
                self.execution.validated_lowering_plan.provenance().digest(),
                self.execution.validated_lowering_plan.summary().digest(),
            ),
            validated_lowering_plan: self.execution.validated_lowering_plan,
            routing_summary: self.execution.routing_summary,
            counters: self.execution.counters,
            read_packet: self.execution.read_packet,
            route_record_entries: self.execution.route_record_entries,
        }
    }
}

impl BridgePreparedDelivery {
    pub(crate) fn contract_proof(&self) -> &BridgeRouteContractProof {
        &self.contract_proof
    }

    pub(crate) fn validated_lowering_plan(&self) -> &ValidatedBridgeLoweringPlan {
        &self.validated_lowering_plan
    }

    pub(crate) fn routing_summary(&self) -> &BridgeRoutingSummary {
        &self.routing_summary
    }

    pub(crate) fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }

    pub(crate) fn read_packet(&self) -> &SnapshotReadPacket {
        &self.read_packet
    }

    pub(crate) fn failure_source(&self) -> BridgeFailureSource {
        BridgeFailureSource::new(
            self.routing_summary.source_commit().clone(),
            self.routing_summary.source_patch().clone(),
            self.routing_summary.source_snapshot().clone(),
            self.counters,
        )
        .with_route_identity(clone_cheap(self.routing_summary.route_identity()))
        .with_contract_proof(clone_cheap(&self.contract_proof))
    }

    pub(crate) fn into_lowered_execution(
        self,
        counters: BridgeRoutingCounters,
    ) -> BridgeLoweredExecution {
        let BridgePreparedDelivery {
            route_scope,
            contract_proof,
            validated_lowering_plan,
            routing_summary,
            counters: prepared_counters,
            read_packet: _read_packet,
            route_record_entries,
        } = self;
        let artifact = lower_validated_route(validated_lowering_plan, counters);
        BridgeLoweredExecution::new(
            route_scope,
            contract_proof,
            routing_summary,
            prepared_counters,
            artifact,
            route_record_entries,
        )
    }
}

impl BridgeLoweredExecution {
    pub(crate) fn route_scope(&self) -> &RouteScope {
        &self.route_scope
    }

    pub(crate) fn new(
        route_scope: RouteScope,
        contract_proof: BridgeRouteContractProof,
        routing_summary: BridgeRoutingSummary,
        counters: BridgeRoutingCounters,
        artifact: BridgeInvalidationArtifact,
        route_record_entries: Arc<[BridgeRouteRecordEntry]>,
    ) -> Self {
        Self {
            route_scope,
            contract_proof,
            routing_summary,
            counters,
            artifact,
            route_record_entries,
        }
    }

    pub(crate) fn contract_proof(&self) -> &BridgeRouteContractProof {
        &self.contract_proof
    }

    pub(crate) fn routing_summary(&self) -> &BridgeRoutingSummary {
        &self.routing_summary
    }

    pub(crate) fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }

    pub(crate) fn artifact(&self) -> &BridgeInvalidationArtifact {
        &self.artifact
    }

    pub(crate) fn route_record_entries(&self) -> &Arc<[BridgeRouteRecordEntry]> {
        &self.route_record_entries
    }

    pub(crate) fn failure_source(&self) -> BridgeFailureSource {
        BridgeFailureSource::new(
            self.artifact.source_commit().clone(),
            self.artifact.source_patch().clone(),
            self.artifact.source_snapshot().clone(),
            *self.counters(),
        )
        .with_route_identity(clone_cheap(self.artifact.route_identity()))
        .with_invalidation_identity(clone_cheap(self.artifact.invalidation_identity()))
        .with_subscription_slice_identity(clone_cheap(self.artifact.subscription_slice_identity()))
        .with_contract_proof(clone_cheap(&self.contract_proof))
    }
}
