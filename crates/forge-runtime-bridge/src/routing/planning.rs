use std::collections::BTreeSet;
use std::sync::Arc;

use crate::clone_budget::clone_cheap;
use crate::diagnostics::{BridgeFailureSource, BridgeReplaySummary, BridgeRouteRecord};
use crate::diagnostics::BridgeRouteRecordEntry;
use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind, BridgeRouteError};
use crate::facade::{BridgeRouteRequest, RuntimeBridge};
use crate::identity::{BridgeIdentity, RouteIdentityTag};
use crate::input::envelope::{
    BridgeCommittedPatchDigest, BridgeCommittedPatchEnvelope, BridgeProducerMetadata,
    TruthCommitIdentity, TruthPatchIdentity,
};
use crate::routing::canonicalization::{
    canonical_route_entry_order, canonical_snapshot_request_order, canonical_target_order,
    digest_string, lowering_provenance_digest_basis, planning_provenance_digest_basis,
    planning_summary_digest_basis, route_digest_basis, SnapshotReadRequestSetView,
};
use crate::routing::context::BridgeMappingContext;
use crate::routing::counters::BridgeRoutingCounters;
use crate::routing::eligibility::{validate_route_request, EligibleRouteEntry, EligibleRouteRequest};
use crate::routing::lowering::{
    BridgeLoweringPlan, BridgeLoweringPlanSummary, BridgeLoweringProvenance,
    BridgeLoweringSummary, BridgeSubscriptionSlice, ValidatedBridgeLoweringPlan,
};
use crate::routing::matching::FineGrainedMatchStatus;
use crate::routing::outcome::BridgeRouteOutcomeReference;
use crate::routing::proof::BridgeRouteContractProof;
use crate::routing::scope::RouteScope;
use crate::snapshot::{SnapshotReadPacket, SnapshotReadRequest, TruthSnapshotIdentity};

pub type BridgeRouteIdentity = BridgeIdentity<RouteIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IngestedBridgePatch {
    envelope: BridgeCommittedPatchEnvelope,
    mapping_context: BridgeMappingContext,
    route_scope: RouteScope,
}

impl IngestedBridgePatch {
    pub(crate) fn new(
        envelope: BridgeCommittedPatchEnvelope,
        mapping_context: BridgeMappingContext,
        route_scope: RouteScope,
    ) -> Self {
        Self {
            envelope,
            mapping_context,
            route_scope,
        }
    }

    pub(crate) fn envelope(&self) -> &BridgeCommittedPatchEnvelope {
        &self.envelope
    }

    pub(crate) fn mapping_context(&self) -> &BridgeMappingContext {
        &self.mapping_context
    }

    pub(crate) fn with_mapping_context(mut self, mapping_context: BridgeMappingContext) -> Self {
        self.mapping_context = mapping_context;
        self
    }

    pub(crate) fn route_scope(&self) -> RouteScope {
        self.route_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EligibleBridgeRouting {
    ingested_patch: IngestedBridgePatch,
    entries: Vec<EligibleRouteEntry>,
    counters: BridgeRoutingCounters,
}

impl EligibleBridgeRouting {
    pub(crate) fn new(
        ingested_patch: IngestedBridgePatch,
        entries: Vec<EligibleRouteEntry>,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            ingested_patch,
            entries,
            counters,
        }
    }

    pub(crate) fn envelope(&self) -> &BridgeCommittedPatchEnvelope {
        self.ingested_patch.envelope()
    }

    pub(crate) fn mapping_context(&self) -> &BridgeMappingContext {
        self.ingested_patch.mapping_context()
    }

    pub(crate) fn entries(&self) -> &[EligibleRouteEntry] {
        &self.entries
    }

    pub(crate) fn counters(&self) -> BridgeRoutingCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteSourceSummary {
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
}

impl BridgeRouteSourceSummary {
    pub(crate) fn new(
        source_commit: TruthCommitIdentity,
        source_patch: TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
    ) -> Self {
        Self {
            source_commit,
            source_patch,
            source_snapshot,
        }
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeExecutionCounts {
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
}

impl BridgeExecutionCounts {
    pub(crate) fn new(
        invalidation_target_count: usize,
        subscription_slice_count: usize,
        snapshot_read_count: usize,
    ) -> Self {
        Self {
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
        }
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.subscription_slice_count
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.snapshot_read_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlanningProvenance {
    route_identity: BridgeRouteIdentity,
    source_digest: BridgeCommittedPatchDigest,
    digest: Arc<str>,
}

impl BridgePlanningProvenance {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source_digest: BridgeCommittedPatchDigest,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            source_digest,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn source_digest(&self) -> &BridgeCommittedPatchDigest {
        &self.source_digest
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlanningSummary {
    route_identity: BridgeRouteIdentity,
    routing_entry_count: usize,
    execution_counts: BridgeExecutionCounts,
    digest: Arc<str>,
}

impl BridgePlanningSummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        routing_entry_count: usize,
        execution_counts: BridgeExecutionCounts,
        digest: Arc<str>,
    ) -> Self {
        Self {
            route_identity,
            routing_entry_count,
            execution_counts,
            digest,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn routing_entry_count(&self) -> usize {
        self.routing_entry_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.execution_counts.invalidation_target_count()
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.execution_counts.subscription_slice_count()
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.execution_counts.snapshot_read_count()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoutingSummary {
    route_identity: BridgeRouteIdentity,
    source: BridgeRouteSourceSummary,
    producer_metadata: BridgeProducerMetadata,
    routing_entry_count: usize,
    invalidation_target_count: usize,
}

impl BridgeRoutingSummary {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        source: BridgeRouteSourceSummary,
        producer_metadata: BridgeProducerMetadata,
        routing_entry_count: usize,
        invalidation_target_count: usize,
    ) -> Self {
        Self {
            route_identity,
            source,
            producer_metadata,
            routing_entry_count,
            invalidation_target_count,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
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

    pub fn routing_entry_count(&self) -> usize {
        self.routing_entry_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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
    artifact: crate::routing::BridgeInvalidationArtifact,
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

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
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

    pub fn lowering_provenance(&self) -> &BridgeLoweringProvenance {
        self.execution.lowering_plan.provenance()
    }

    pub fn validated_lowering_summary(&self) -> &BridgeLoweringSummary {
        self.execution.validated_lowering_plan.summary()
    }

    pub(crate) fn into_prepared_delivery(self) -> BridgePreparedDelivery {
        BridgePreparedDelivery {
            route_scope: self.route_scope,
            contract_proof: BridgeRouteContractProof::new(
                self.producer_metadata,
                self.mapping_context,
                self.source_digest,
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
        let artifact = crate::routing::lowering::lower_validated_route(validated_lowering_plan, counters);
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
    pub(crate) fn new(
        route_scope: RouteScope,
        contract_proof: BridgeRouteContractProof,
        routing_summary: BridgeRoutingSummary,
        counters: BridgeRoutingCounters,
        artifact: crate::routing::BridgeInvalidationArtifact,
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

    pub(crate) fn artifact(&self) -> &crate::routing::BridgeInvalidationArtifact {
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

pub(crate) fn plan_ingested_patch(
    runtime: &RuntimeBridge,
    ingested: IngestedBridgePatch,
) -> Result<BridgePlannedRoute, BridgeRouteError> {
    let eligible = into_eligible_bridge_routing(runtime, ingested)?;
    let envelope = eligible.envelope();
    let mapping_context = eligible.mapping_context();

    let mut entries = eligible.entries().to_vec();
    entries.sort_by(canonical_route_entry_order);
    let route_basis = route_digest_basis(envelope, mapping_context, &entries);
    let route_identity = BridgeRouteIdentity::new(digest_string("route", &route_basis));

    let invalidation_targets = canonical_invalidation_targets(&entries);
    let subscription_slices = canonical_subscription_slices(&entries);
    let invalidation_target_count = invalidation_targets.len();
    let subscription_slice_count = subscription_slices.len();
    let read_packet = canonical_read_packet(&subscription_slices, &entries);
    let route_record_entries = canonical_route_record_entries(&entries);
    let read_packet_view = SnapshotReadRequestSetView::new(read_packet.reads());
    let planning_provenance_basis = planning_provenance_digest_basis(
        &route_identity,
        envelope,
        mapping_context,
        &entries,
        &read_packet_view,
    );
    let planning_provenance = BridgePlanningProvenance::new(
        route_identity.clone(),
        envelope.digest().clone(),
        digest_string(
            "planning-provenance",
            &planning_provenance_basis,
        ),
    );
    let planning_summary_basis = planning_summary_digest_basis(
        &route_identity,
        entries.len(),
        invalidation_target_count,
        subscription_slice_count,
        read_packet.reads().len(),
    );
    let planning_summary = BridgePlanningSummary::new(
        route_identity.clone(),
        entries.len(),
        BridgeExecutionCounts::new(
            invalidation_target_count,
            subscription_slice_count,
            read_packet.reads().len(),
        ),
        digest_string(
            "planning-summary",
            &planning_summary_basis,
        ),
    );

    let source_summary = BridgeRouteSourceSummary::new(
        envelope.commit_identity().clone(),
        envelope.patch_identity().clone(),
        envelope.snapshot_identity().clone(),
    );
    let routing_summary = BridgeRoutingSummary::new(
        route_identity.clone(),
        source_summary,
        envelope.producer_metadata().clone(),
        entries.len(),
        invalidation_target_count,
    );
    let lowering_plan = BridgeLoweringPlan::new(
        route_identity.clone(),
        envelope.commit_identity().clone(),
        envelope.patch_identity().clone(),
        envelope.snapshot_identity().clone(),
        invalidation_targets,
        subscription_slices,
        read_packet.reads().len(),
        BridgeLoweringProvenance::new(
            route_identity.clone(),
            planning_provenance.clone(),
            digest_string(
                "lowering-provenance",
                &lowering_provenance_digest_basis(
                    &route_identity,
                    planning_provenance.digest(),
                    envelope.commit_identity().as_str(),
                    envelope.patch_identity().as_str(),
                    envelope.snapshot_identity().as_str(),
                ),
            ),
        ),
    );
    let validated_lowering_plan = ValidatedBridgeLoweringPlan::from_plan(&lowering_plan)?;
    let counters = eligible
        .counters()
        .with_routing_entry_count(entries.len())
        .with_invalidation_target_count(invalidation_target_count)
        .with_snapshot_packet(read_packet.reads().len())
        .with_sort_input_width(
            entries.len()
                + invalidation_target_count
                + subscription_slice_count
                + read_packet.reads().len(),
        )
        .with_digest_computations(4 + lowering_plan.digest_computation_count())
        .with_digest_input_bytes(
            route_basis.len()
                + planning_provenance_basis.len()
                + planning_summary_basis.len()
                + lowering_provenance_digest_basis(
                    &route_identity,
                    planning_provenance.digest(),
                    envelope.commit_identity().as_str(),
                    envelope.patch_identity().as_str(),
                    envelope.snapshot_identity().as_str(),
                )
                .len()
                + lowering_plan.digest_input_bytes(),
        );

    Ok(BridgePlannedRoute::new(
        eligible.ingested_patch.route_scope(),
        mapping_context.clone(),
        route_identity,
        BridgeRouteSourceSummary::new(
            envelope.commit_identity().clone(),
            envelope.patch_identity().clone(),
            envelope.snapshot_identity().clone(),
        ),
        envelope.producer_metadata().clone(),
        envelope.digest().clone(),
        BridgePlanningArtifacts::new(planning_provenance, planning_summary),
        BridgePlannedExecution::new(
            routing_summary,
            read_packet,
            counters,
            lowering_plan,
            validated_lowering_plan,
            route_record_entries,
        ),
    ))
}

fn into_eligible_bridge_routing(
    runtime: &RuntimeBridge,
    ingested: IngestedBridgePatch,
) -> Result<EligibleBridgeRouting, BridgeRouteError> {
    let eligible = validate_route_request(
        ingested.envelope.clone(),
        &runtime.mapping_registry,
        &runtime.aspect_registry,
    )?;
    Ok(EligibleBridgeRouting::from((ingested, eligible)))
}

impl From<(IngestedBridgePatch, EligibleRouteRequest)> for EligibleBridgeRouting {
    fn from(value: (IngestedBridgePatch, EligibleRouteRequest)) -> Self {
        let (ingested_patch, eligible) = value;
        Self::new(
            ingested_patch,
            eligible.entries().to_vec(),
            eligible.counters(),
        )
    }
}

pub(crate) fn replay_route_record(
    runtime: &RuntimeBridge,
    record: &BridgeRouteRecord,
) -> Result<BridgeReplaySummary, BridgeReplayError> {
    if !runtime.policy.allow_replay_artifacts() {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::ReplayArtifactsDisabled,
            "Bridge replay artifacts are disabled by runtime policy.",
        )
        .with_context(replay_context(record)));
    }

    let planned = match runtime.plan_committed_patch_with_mapping_context(
        BridgeRouteRequest::for_commit(record.source_commit().as_str()),
        record.mapping_context().clone(),
    ) {
        Ok(planned) => planned,
        Err(error) => {
            let replay_error = BridgeReplayError::new(
                BridgeReplayErrorKind::RouteMismatch,
                format!("Bridge replay failed to reconstruct the planned route: {error}"),
            )
            .with_context(replay_context(record));
            return Err(reject_replay(
                runtime,
                record,
                crate::routing::counters::BridgeRoutingCounters::default().with_route_replay_mismatch(),
                replay_error,
            ));
        }
    };
    let prepared = planned.into_prepared_delivery();
    let contract_proof = prepared.contract_proof().clone();
    let validated_lowering_plan = prepared.validated_lowering_plan().clone();
    let routing_summary = prepared.routing_summary().clone();
    let counters = *prepared.counters();
    let source_digest = contract_proof.source_digest().clone();
    let replay_counters = counters.with_route_replay_mismatch();
    if routing_summary.route_identity() != record.route_identity() {
        let error = BridgeReplayError::new(
            BridgeReplayErrorKind::RouteMismatch,
            format!(
                "Bridge replay reconstructed route `{}` but original route was `{}`.",
                routing_summary.route_identity().as_str(),
                record.route_identity().as_str()
            ),
        )
        .with_context(replay_context(record).with_invalidation_identity(record.invalidation_identity().clone()));
        return Err(reject_replay(runtime, record, replay_counters, error));
    }
    if source_digest != *record.source_digest() {
        let error = BridgeReplayError::new(
            BridgeReplayErrorKind::DigestMismatch,
            format!(
                "Bridge replay reconstructed digest `{}` but original digest was `{}`.",
                source_digest.as_str(),
                record.source_digest().as_str()
            ),
        )
        .with_context(replay_context(record).with_invalidation_identity(record.invalidation_identity().clone()));
        return Err(reject_replay(runtime, record, replay_counters, error));
    }

    let lowering_provenance_digest = contract_proof.lowering_provenance_digest().to_owned();
    let lowering_summary_digest = contract_proof.lowering_summary_digest().to_owned();
    let artifact =
        crate::routing::lowering::lower_validated_route(validated_lowering_plan, counters);
    if artifact.invalidation_identity() != record.invalidation_identity() {
        let error = BridgeReplayError::new(
            BridgeReplayErrorKind::InvalidationMismatch,
            format!(
                "Bridge replay reconstructed invalidation `{}` but original invalidation was `{}`.",
                artifact.invalidation_identity().as_str(),
                record.invalidation_identity().as_str()
            ),
        )
        .with_context(replay_context(record).with_invalidation_identity(record.invalidation_identity().clone()));
        return Err(reject_replay(runtime, record, replay_counters, error));
    }
    if artifact.subscription_slice_identity() != record.subscription_slice_identity() {
        let error = BridgeReplayError::new(
            BridgeReplayErrorKind::SubscriptionSliceMismatch,
            format!(
                "Bridge replay reconstructed subscription slices `{}` but original slices were `{}`.",
                artifact.subscription_slice_identity().as_str(),
                record.subscription_slice_identity().as_str()
            ),
        )
        .with_context(
            replay_context(record)
                .with_invalidation_identity(record.invalidation_identity().clone())
                .with_subscription_slice_identity(record.subscription_slice_identity().clone()),
        );
        return Err(reject_replay(runtime, record, replay_counters, error));
    }

    if contract_proof.planning_provenance_digest()
        != record.contract_proof().planning_provenance_digest()
        || contract_proof.planning_summary_digest()
            != record.contract_proof().planning_summary_digest()
    {
        let error = BridgeReplayError::new(
            BridgeReplayErrorKind::PlanningContractMismatch,
            format!(
                "Bridge replay reconstructed planning contract `{}` / `{}` but original planning contract was `{}` / `{}`.",
                contract_proof.planning_provenance_digest(),
                contract_proof.planning_summary_digest(),
                record.contract_proof().planning_provenance_digest(),
                record.contract_proof().planning_summary_digest()
            ),
        )
        .with_context(replay_context(record).with_invalidation_identity(record.invalidation_identity().clone()));
        return Err(reject_replay(runtime, record, replay_counters, error));
    }

    if artifact.subscription_slice_identity() == record.subscription_slice_identity()
        && (artifact.invalidation_identity() == record.invalidation_identity())
        && (lowering_provenance_digest != record.contract_proof().lowering_provenance_digest()
            || lowering_summary_digest != record.contract_proof().lowering_summary_digest())
    {
        let error = BridgeReplayError::new(
            BridgeReplayErrorKind::LoweringContractMismatch,
            format!(
                "Bridge replay reconstructed lowering contract `{}` / `{}` but original lowering contract was `{}` / `{}`.",
                lowering_provenance_digest,
                lowering_summary_digest,
                record.contract_proof().lowering_provenance_digest(),
                record.contract_proof().lowering_summary_digest()
            ),
        )
        .with_context(replay_context(record).with_invalidation_identity(record.invalidation_identity().clone()));
        return Err(reject_replay(runtime, record, replay_counters, error));
    }

    Ok(BridgeReplaySummary::new(BridgeRouteOutcomeReference::new(
        routing_summary.route_identity().clone(),
        artifact.invalidation_identity().clone(),
        BridgeRouteSourceSummary::new(
            routing_summary.source_commit().clone(),
            routing_summary.source_patch().clone(),
            routing_summary.source_snapshot().clone(),
        ),
        artifact.subscription_slice_identity().clone(),
    )))
}

fn replay_context(record: &BridgeRouteRecord) -> BridgeErrorContext {
    BridgeErrorContext::replay(
        record.route_identity().clone(),
        record.source_snapshot().clone(),
    )
}

fn replay_failure_source(
    record: &BridgeRouteRecord,
    counters: BridgeRoutingCounters,
) -> BridgeFailureSource {
    BridgeFailureSource::new(
        record.source_commit().clone(),
        record.source_patch().clone(),
        record.source_snapshot().clone(),
        counters,
    )
    .with_route_identity(record.route_identity().clone())
    .with_invalidation_identity(record.invalidation_identity().clone())
    .with_subscription_slice_identity(record.subscription_slice_identity().clone())
    .with_contract_proof(record.contract_proof().clone())
}

fn reject_replay(
    runtime: &RuntimeBridge,
    record: &BridgeRouteRecord,
    counters: BridgeRoutingCounters,
    error: BridgeReplayError,
) -> BridgeReplayError {
    runtime
        .diagnostic_sink
        .record_replay_failure(replay_failure_source(record, counters), &error);
    error
}

fn canonical_invalidation_targets(
    entries: &[EligibleRouteEntry],
) -> Vec<(Arc<str>, crate::mapping::CoarseRoutingMode)> {
    let mut deduped = BTreeSet::new();
    for entry in entries {
        deduped.insert((
            Arc::<str>::from(entry.registration().signal_scope().as_str()),
            entry.registration().routing_mode(),
        ));
    }

    let mut targets = deduped.into_iter().collect::<Vec<_>>();
    targets.sort_by(|left, right| canonical_target_order(left, right));
    targets
}

fn canonical_read_packet(
    subscription_slices: &[BridgeSubscriptionSlice],
    entries: &[EligibleRouteEntry],
) -> SnapshotReadPacket {
    if subscription_slices.is_empty() {
        return canonical_coarse_read_packet(entries);
    }

    let mut deduped = BTreeSet::new();
    for slice in subscription_slices {
        deduped.insert((
            slice.entity_identity().to_owned(),
            slice.aspect_label().to_owned(),
            slice.surface_label().to_owned(),
            slice.slice_kind().clone(),
        ));
    }

    let mut reads = deduped
        .into_iter()
        .map(|(entity_identity, aspect_label, surface_label, slice_kind)| {
            SnapshotReadRequest::for_subscription_slice(
                entity_identity,
                aspect_label,
                surface_label,
                slice_kind,
            )
        })
        .collect::<Vec<_>>();
    reads.sort_by(canonical_snapshot_request_order);
    SnapshotReadPacket::new(reads)
}

fn canonical_coarse_read_packet(entries: &[EligibleRouteEntry]) -> SnapshotReadPacket {
    let mut deduped = BTreeSet::new();
    for entry in entries {
        deduped.insert((
            entry.item().entity_identity().to_owned(),
            entry.item().aspect_label().to_owned(),
        ));
    }

    let mut reads = deduped
        .into_iter()
        .map(|(entity_identity, aspect_label)| {
            SnapshotReadRequest::for_coarse(entity_identity, aspect_label)
        })
        .collect::<Vec<_>>();
    reads.sort_by(canonical_snapshot_request_order);
    SnapshotReadPacket::new(reads)
}

fn canonical_subscription_slices(entries: &[EligibleRouteEntry]) -> Vec<BridgeSubscriptionSlice> {
    let mut deduped = BTreeSet::new();
    for entry in entries {
        match entry.fine_grained_match().status() {
            FineGrainedMatchStatus::Matched | FineGrainedMatchStatus::FallbackAdmitted => {
                let Some(slice_kind) = entry.fine_grained_match().subscription_slice_kind() else {
                    continue;
                };

                deduped.insert(BridgeSubscriptionSlice::new(
                    entry.normalized_surface().entity_identity(),
                    entry.normalized_surface().aspect_label(),
                    entry.normalized_surface().surface_label(),
                    slice_kind.clone(),
                    entry.fine_grained_match().status(),
                ));
            }
            FineGrainedMatchStatus::SuppressedByRegistrationPolicy
            | FineGrainedMatchStatus::UnsupportedSurfaceCategory
            | FineGrainedMatchStatus::AmbiguousRegistration => {}
        }
    }

    let mut slices = deduped.into_iter().collect::<Vec<_>>();
    slices.sort();
    slices
}

fn canonical_route_record_entries(entries: &[EligibleRouteEntry]) -> Arc<[BridgeRouteRecordEntry]> {
    Arc::from(
        entries
        .iter()
        .map(|entry| {
            BridgeRouteRecordEntry::new(
                entry.normalized_surface().entity_identity(),
                entry.normalized_surface().aspect_label(),
                entry.normalized_surface().surface_label(),
                entry.item().surface_label(),
                entry.normalized_surface().surface_identity().as_str(),
                entry.registration().mapping_id().clone(),
                entry.registration().signal_scope().as_str(),
                entry.registration().routing_mode(),
                entry.fallback_class(),
                entry.fine_grained_match().clone(),
            )
        })
        .collect::<Vec<_>>(),
    )
}
