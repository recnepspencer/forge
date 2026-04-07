//! Public API boundary for `forge-runtime-bridge`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

use std::sync::Arc;

use crate::mapping::{FrozenAspectMappingRegistry, FrozenMappingRegistry};
pub use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, RelationalBridgeSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, SignalBridgeSink, SignalBridgeSinkError, SnapshotReadSource,
    SnapshotReaderPool, TruthBranchHeadSource,
};
use crate::diagnostics::DiagnosticSink;
pub use crate::builder::RuntimeBridgeBuilder;
pub use crate::delivery::{
    BridgeDeliveryReceipt, BridgePreparedDeliveryRequest, BridgeSignalEvaluationRequest,
};
pub use crate::diagnostics::{
    BridgeBulkPlanExplanation, BridgeCanonicalContinuityRecord, BridgeCanonicalRouteRecord,
    BridgeContinuityExplanation,
    BridgeContinuityReplaySummary,
    BridgeDiagnosticsFacade, BridgeDiagnosticsHandle,
    BridgeContractDiagnosticsRecord, BridgeFailureClass, BridgeFailureRecord,
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationCounters,
    BridgeHistoricalEvaluationDecisionLog, BridgeHistoricalEvaluationExplanation,
    BridgeHistoricalEvaluationFailureClass, BridgeHistoricalEvaluationFailureRecord,
    BridgeHistoricalEvaluationRecord,
    BridgeHistoricalEvaluationReplaySummary, BridgeHistoricalMaterializationPath,
    BridgeLoweringDiagnosticsRecord, BridgeReplayRecord, BridgeReplaySummary,
    BridgeRouteExplanation, BridgeRouteExplanationEntry, BridgeRouteRecord, BridgeRouteRecordMatch,
    BridgeRouteRecordEntry, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
    BRIDGE_CANONICAL_HISTORICAL_EVALUATION_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3,
};
pub use crate::error::{
    BridgeBuildError, BridgeBuildErrorKind, BridgeContinuityError, BridgeContinuityErrorKind,
    BridgeDeliveryError, BridgeDeliveryErrorKind, BridgeErrorContext, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, BridgePatchCoordinate, BridgeReplayError,
    BridgeReplayErrorKind, BridgeRouteError, BridgeRouteErrorKind, BridgeSnapshotReadCoordinate,
};
pub use crate::input::envelope::{
    BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchItem, BridgeCommittedPatchSummary, BridgeProducerAuthorityKind,
    BridgeProducerMetadata, RawCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, BRIDGE_PRODUCER_EXPORT_SCHEMA_V1,
};
pub use crate::mapping::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingFallbackClass,
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    SignalInvalidationScope, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
    TruthPatchScope,
};
pub use crate::continuity::{
    BridgeContinuityAuthorityBasis, BridgeContinuityAuthorityKind, BridgeContinuityClass,
    BridgeContinuityArtifact, BridgeContinuityCounters, BridgeContinuityIdentity,
    BridgeContinuityDigestBasisKind, BridgeContinuityOutcomeClass,
    BridgeContinuityRejectionClass, BridgeEligibleContinuityRequestSet,
    BridgeHistoricalLineagePacket, BridgeHistoricalLineagePacketEntry,
    BridgePlannedContinuityRequest, BridgePlannedContinuityRequestSet,
    BridgeUnsupportedContinuityClass, PriorSubscriptionSlice, ResolvedLineageContinuity,
    ResolvedLineageContinuitySet,
};
pub use crate::policy::{BridgeDiagnosticsRetentionBudget, BridgeDiagnosticsTier, BridgeRuntimePolicy};
pub use crate::routing::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkPlanningCounters, BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind, BridgeBulkWorkloadSegment,
    BridgeCanonicalBulkPlanRecord, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
    BridgeCanonicalPlanningIdentity, BridgeExecutionCounts, BridgeLocalityFootprint,
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeInvalidationTarget,
    BridgeLoweringPlanSummary, BridgeLoweringProvenance, BridgeLoweringSummary,
    BridgeParallelAdmission, BridgeParallelAdmissionClass, BridgeParallelAdmissionReason,
    BridgeParallelLegalityClass, BridgeParallelLegalityDecision, BridgeParallelLegalityReason,
    BridgeParallelProfitabilityClass, BridgeParallelProfitabilityDecision,
    BridgeParallelProfitabilityReason,
    BridgePlannedRoute, BridgePlanningProvenance, BridgePlanningSummary,
    BridgePreparationMode, BridgeRouteIdentity, BridgeRouteContractProof, BridgeRouteOutcomeReference,
    BridgeBulkResultSummary, BridgeBulkWorkloadResult, BridgeRouteResult, BridgeRouteResultSummary,
    BridgeRoutingCounters, BridgeRoutingSummary,
    BridgeLineageContext, BridgeMappingContext, BridgeRouteSourceSummary, BridgeWorkloadIdentity,
    CanonicalBridgeWorkloadRequest, ContinuityPacketIdentity, ContinuityRemapPacket, DisjointPacketRegionSet,
    FallbackAggregationPacket, FallbackPacketIdentity, NormalizedBridgeWorkloadSummary,
    InvalidationReductionPacket, ParallelPreparationLegalityProof, PlannedBridgePacketSet,
    ReducedBridgePublication, ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity,
    ReducedContinuityRemap, ReducedPublicationIdentity, ReducedTruthViewIdentity,
    ReducedTruthViewMaterialization, ReductionPacketIdentity, RoutingPacketIdentity,
    TruthDeltaRoutingPacket, TruthViewMaterializationPacket, TruthViewPacketIdentity,
    FineGrainedMatchOutcome, FineGrainedMatchStatus, BridgeSignalInvalidationDelivery, BridgeSubscriptionSlice,
    BridgeSubscriptionSliceIdentity, CanonicalInvalidationTargets, CanonicalSubscriptionSlices,
};
pub use crate::snapshot::{
    AdmittedSnapshotContext, BridgeDeliveryIntent, BridgeReplayMode, BridgeSnapshotContext,
    BridgeSnapshotReadError, BridgeSnapshotToken, BridgeTruthViewKind,
    BridgeTruthViewPolicyRejection, BridgeTruthViewPolicyResolution, BridgeTruthViewSelector,
    BridgeTruthViewSelectorIdentity, HistoricalEvaluationDeclaration,
    HistoricalEvaluationDeclarationIdentity, LoweredHistoricalEvaluationArtifact,
    LoweredHistoricalEvaluationArtifactIdentity, ResolvedTruthViewPolicy, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest, TruthSnapshotIdentity,
    TruthSnapshotReader, TruthViewPolicyRejectionKind, TruthViewReplayCompatibility,
    TruthViewRetentionAdmission, TruthViewSourceCapability, ValidatedSnapshotReadPacketResult,
};

#[derive(Clone)]
pub struct RuntimeBridge {
    pub(crate) policy: BridgeRuntimePolicy,
    pub(crate) diagnostics: BridgeDiagnosticsFacade,
    pub(crate) diagnostic_sink: Arc<dyn DiagnosticSink>,
    pub(crate) committed_patch_source: Arc<dyn CommittedPatchSource>,
    pub(crate) snapshot_read_source: Arc<dyn SnapshotReadSource>,
    pub(crate) snapshot_reader_pool: Option<Arc<dyn SnapshotReaderPool>>,
    pub(crate) signal_sink: Arc<dyn InvalidationSink>,
    pub(crate) truth_branch_head_source: Option<Arc<dyn TruthBranchHeadSource>>,
    pub(crate) continuity_lineage_source: Option<Arc<dyn ContinuityLineageSource>>,
    pub(crate) mapping_registry: FrozenMappingRegistry,
    pub(crate) aspect_registry: FrozenAspectMappingRegistry,
}

impl RuntimeBridge {
    pub fn builder() -> RuntimeBridgeBuilder {
        RuntimeBridgeBuilder::new()
    }

    pub fn policy(&self) -> &BridgeRuntimePolicy {
        &self.policy
    }

    pub fn ingest_committed_patch(
        &self,
        request: BridgeRouteRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, BridgeRouteError> {
        Ok(crate::input::ingress::ingest_committed_patch(self, request)?
            .envelope()
            .clone())
    }

    pub fn plan_envelope(
        &self,
        envelope: BridgeCommittedPatchEnvelope,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_envelope_with_mapping_context(envelope, BridgeMappingContext::default())
    }

    pub fn plan_envelope_with_mapping_context(
        &self,
        envelope: BridgeCommittedPatchEnvelope,
        mapping_context: BridgeMappingContext,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        crate::routing::planning::plan_ingested_patch(
            self,
            crate::routing::IngestedBridgePatch::new(
                envelope,
                mapping_context,
                crate::routing::scope::RouteScope::begin(),
            ),
        )
    }

    pub fn plan_committed_patch(
        &self,
        request: BridgeRouteRequest,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_committed_patch_with_mapping_context(request, BridgeMappingContext::default())
    }

    pub fn plan_committed_patch_with_mapping_context(
        &self,
        request: BridgeRouteRequest,
        mapping_context: BridgeMappingContext,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        let ingested = crate::input::ingress::ingest_committed_patch(self, request)?;
        crate::routing::planning::plan_ingested_patch(
            self,
            ingested.with_mapping_context(mapping_context),
        )
    }

    pub fn plan_bulk_workload(
        &self,
        request: BridgeBulkWorkloadRequest,
    ) -> Result<BridgeBulkWorkloadPlan, BridgeRouteError> {
        crate::routing::planning::plan_bulk_workload(self, request)
    }

    pub fn canonicalize_bulk_workload_plan(
        &self,
        plan: &BridgeBulkWorkloadPlan,
    ) -> BridgeCanonicalBulkPlanRecord {
        let record = crate::routing::BridgeCanonicalBulkPlanRecord::from_bulk_workload_plan(plan);
        self.diagnostics.record_bulk(record.clone());
        record
    }

    pub fn deliver_invalidation(
        &self,
        route: BridgePlannedRoute,
    ) -> Result<BridgeRouteResult, BridgeDeliveryError> {
        crate::delivery::deliver_planned_route(self, route)
    }

    pub fn prepare_delivery(
        &self,
        route: BridgePlannedRoute,
    ) -> BridgePreparedDeliveryRequest {
        crate::delivery::prepare_planned_route_for_delivery(route)
    }

    pub fn deliver_prepared(
        &self,
        prepared: BridgePreparedDeliveryRequest,
    ) -> Result<BridgeRouteResult, BridgeDeliveryError> {
        crate::delivery::deliver_prepared_route(self, prepared)
    }

    pub fn deliver_bulk_workload_plan(
        &self,
        plan: BridgeBulkWorkloadPlan,
    ) -> Result<BridgeBulkWorkloadResult, BridgeDeliveryError> {
        crate::delivery::deliver_bulk_workload_plan(self, plan)
    }

    pub fn prepare_signal_evaluation(
        &self,
        route: BridgePlannedRoute,
    ) -> Result<BridgeSignalEvaluationRequest, BridgeDeliveryError> {
        crate::delivery::prepare_signal_evaluation(self, route)
    }

    pub fn replay_canonical_record(
        &self,
        record: &BridgeCanonicalRouteRecord,
    ) -> Result<BridgeReplaySummary, BridgeReplayError> {
        let route_record = record.decode()?;
        crate::routing::replay_route_record(self, &route_record)
    }

    pub fn replay_canonical_bulk_plan_record(
        &self,
        record: &BridgeCanonicalBulkPlanRecord,
    ) -> Result<BridgeBulkWorkloadPlan, BridgeReplayError> {
        if !self.policy.allow_replay_artifacts() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::ReplayArtifactsDisabled,
                "Bridge replay artifacts are disabled by runtime policy.",
            ));
        }

        let record = record.decode()?;
        let replayed = self.plan_bulk_workload(record.request().clone()).map_err(|error| {
            BridgeReplayError::new(
                BridgeReplayErrorKind::BulkPlanReplayMismatch,
                format!("Bridge bulk replay failed to reconstruct the planned workload: {error}"),
            )
        })?;

        if replayed.workload_identity() != record.workload_identity()
            || replayed.canonical_request().digest()
                != record.canonical_request_digest()
            || replayed.normalized_summary().digest()
                != record.normalized_summary_digest()
            || replayed.canonical_planning_identity() != record.canonical_planning_identity()
            || replayed.admission_profile_identity() != record.admission_profile_identity()
            || replayed.packet_set().digest() != record.packet_set_digest()
            || replayed.execution_plan().digest() != record.execution_plan_digest()
            || replayed.execution_plan().reduced_artifact().digest()
                != record.reduced_artifact_digest()
        {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::BulkPlanReplayMismatch,
                format!(
                    "Bridge bulk replay reconstructed workload `{}` / plan `{}` / execution `{}` but the canonical record expected `{}` / `{}` / `{}`.",
                    replayed.workload_identity().as_str(),
                    replayed.canonical_planning_identity().as_str(),
                    replayed.execution_plan().digest(),
                    record.workload_identity().as_str(),
                    record.canonical_planning_identity().as_str(),
                    record.execution_plan_digest()
                ),
            ));
        }

        Ok(replayed)
    }

    pub fn diagnostics(&self) -> &BridgeDiagnosticsFacade {
        &self.diagnostics
    }

    pub fn continuity_lineage_source(&self) -> Option<&dyn ContinuityLineageSource> {
        self.continuity_lineage_source.as_deref()
    }

    pub fn plan_continuity_requests(
        &self,
        prior_route_record: &BridgeRouteRecord,
    ) -> Result<BridgeEligibleContinuityRequestSet, BridgeContinuityError> {
        let planned =
            crate::continuity::BridgePlannedContinuityRequestSet::from_route_record(prior_route_record)?;
        crate::continuity::BridgeEligibleContinuityRequestSet::from_planned(planned)
    }

    pub fn plan_historical_lineage_packet(
        &self,
        requests: &BridgeEligibleContinuityRequestSet,
    ) -> Result<BridgeHistoricalLineagePacket, BridgeContinuityError> {
        let source = self.continuity_lineage_source().ok_or_else(|| {
            BridgeContinuityError::new(
                BridgeContinuityErrorKind::MissingLineageSource,
                "Bridge historical lineage planning requires a configured continuity lineage source.",
            )
        })?;
        let mut entries = Vec::with_capacity(requests.requests().len());
        for request in requests.requests() {
            let lineage_authority = source
                .historical_lineage(BridgeHistoricalLineageRequest::new(
                    requests.authority_basis().clone(),
                    request.prior_slice().clone(),
                ))
                .map_err(|error| match error.kind() {
                    BridgeLineageSourceErrorKind::UnsupportedContinuityClass => {
                        BridgeContinuityError::new(
                            BridgeContinuityErrorKind::UnsupportedContinuityClass,
                            format!(
                                "Bridge continuity request `{}` targeted an unsupported continuity class: {error}",
                                request.request_key()
                            ),
                        )
                    }
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure => {
                        BridgeContinuityError::new(
                            BridgeContinuityErrorKind::HistoricalResolutionFailure,
                            format!(
                                "Bridge failed to resolve historical lineage for continuity request `{}`: {error}",
                                request.request_key()
                            ),
                        )
                    }
                })?;
            if lineage_authority.authority_basis() != requests.authority_basis() {
                return Err(BridgeContinuityError::new(
                    BridgeContinuityErrorKind::LineageAuthorityMismatch,
                    format!(
                        "Bridge historical lineage authority for continuity request `{}` did not match the planned branch/snapshot authority basis.",
                        request.request_key()
                    ),
                ));
            }
            entries.push(crate::continuity::BridgeHistoricalLineagePacketEntry::new(
                request.request_key(),
                request.prior_slice().clone(),
                lineage_authority,
            ));
        }
        Ok(crate::continuity::BridgeHistoricalLineagePacket::from_entries(
            requests, entries,
        ))
    }

    pub fn resolve_lineage_continuity(
        &self,
        packet: &BridgeHistoricalLineagePacket,
    ) -> Result<ResolvedLineageContinuitySet, BridgeContinuityError> {
        crate::continuity::ResolvedLineageContinuitySet::from_historical_packet(packet)
    }

    pub fn lower_continuity_artifact(
        &self,
        resolved: &ResolvedLineageContinuitySet,
    ) -> BridgeContinuityArtifact {
        crate::continuity::BridgeContinuityArtifact::from_resolved(resolved)
    }

    pub fn canonicalize_continuity_record(
        &self,
        route_record: &BridgeRouteRecord,
        requests: &BridgeEligibleContinuityRequestSet,
        artifact: &BridgeContinuityArtifact,
    ) -> BridgeCanonicalContinuityRecord {
        let record = crate::diagnostics::BridgeCanonicalContinuityRecord::new(
            route_record.clone(),
            requests.digest(),
            artifact.continuity_resolution_digest(),
            artifact.continuity_identity().clone(),
            artifact.remapped_subscription_slice_identity().clone(),
            artifact.remapped_slices().clone(),
            std::sync::Arc::from(artifact.continuity_outcomes().to_vec()),
            *artifact.counters(),
        );
        self.diagnostics.record_continuity(record.clone());
        record
    }

    pub fn replay_canonical_continuity_record(
        &self,
        record: &BridgeCanonicalContinuityRecord,
    ) -> Result<BridgeContinuityReplaySummary, BridgeReplayError> {
        let record = record.decode()?;
        let _continuity_replay_mismatch_counters =
            record.counters().with_continuity_replay_mismatch();
        let replay_context = BridgeErrorContext::replay(
            record.route_identity().clone(),
            record.source_snapshot().clone(),
        )
        .with_subscription_slice_identity(record.remapped_subscription_slice_identity().clone());

        let requests = self
            .plan_continuity_requests(record.route_record())
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::ContinuityRequestMismatch,
                    format!("Bridge continuity replay failed to reconstruct the planned continuity requests: {error}"),
                )
                .with_context(replay_context.clone())
            })?;
        if requests.digest() != record.continuity_request_digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::ContinuityRequestMismatch,
                format!(
                    "Bridge continuity replay reconstructed request digest `{}` but original digest was `{}`.",
                    requests.digest(),
                    record.continuity_request_digest()
                ),
            )
            .with_context(replay_context.clone()));
        }

        let packet = self
            .plan_historical_lineage_packet(&requests)
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::ContinuityResolutionMismatch,
                    format!("Bridge continuity replay failed to reconstruct the historical lineage packet: {error}"),
                )
                .with_context(replay_context.clone())
            })?;
        let resolved = self
            .resolve_lineage_continuity(&packet)
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::ContinuityResolutionMismatch,
                    format!("Bridge continuity replay failed to resolve continuity canonically: {error}"),
                )
                .with_context(replay_context.clone())
            })?;
        let artifact = self.lower_continuity_artifact(&resolved);
        if artifact.continuity_resolution_digest() != record.continuity_resolution_digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::ContinuityResolutionMismatch,
                format!(
                    "Bridge continuity replay reconstructed resolution digest `{}` but original digest was `{}`.",
                    artifact.continuity_resolution_digest(),
                    record.continuity_resolution_digest()
                ),
            )
            .with_context(replay_context.clone()));
        }
        if artifact.continuity_identity() != record.continuity_artifact_identity()
            || artifact.remapped_subscription_slice_identity()
                != record.remapped_subscription_slice_identity()
        {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::ContinuityArtifactMismatch,
                format!(
                    "Bridge continuity replay reconstructed artifact `{}` / `{}` but original artifact was `{}` / `{}`.",
                    artifact.continuity_identity().as_str(),
                    artifact.remapped_subscription_slice_identity().as_str(),
                    record.continuity_artifact_identity().as_str(),
                    record.remapped_subscription_slice_identity().as_str()
                ),
            )
            .with_context(replay_context));
        }

        Ok(artifact)
    }

    pub(crate) fn new(
        policy: BridgeRuntimePolicy,
        committed_patch_source: Arc<dyn CommittedPatchSource>,
        snapshot_read_source: Arc<dyn SnapshotReadSource>,
        signal_sink: Arc<dyn InvalidationSink>,
        truth_branch_head_source: Option<Arc<dyn TruthBranchHeadSource>>,
        continuity_lineage_source: Option<Arc<dyn ContinuityLineageSource>>,
        snapshot_reader_pool: Option<Arc<dyn SnapshotReaderPool>>,
        diagnostic_sink: Option<Arc<dyn DiagnosticSink>>,
        mapping_registry: FrozenMappingRegistry,
        aspect_registry: FrozenAspectMappingRegistry,
    ) -> Self {
        let diagnostics = BridgeDiagnosticsFacade::new(policy);
        let diagnostic_sink =
            diagnostic_sink.unwrap_or_else(|| Arc::new(diagnostics.clone()));
        Self {
            diagnostic_sink,
            diagnostics,
            policy,
            committed_patch_source,
            snapshot_read_source,
            snapshot_reader_pool,
            signal_sink,
            truth_branch_head_source,
            continuity_lineage_source,
            mapping_registry,
            aspect_registry,
        }
    }
}

impl std::fmt::Debug for RuntimeBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeBridge")
            .field("policy", &self.policy)
            .field("diagnostics", &self.diagnostics)
            .field(
                "mapping_registration_count",
                &self.mapping_registry.registrations().len(),
            )
            .field(
                "aspect_registration_count",
                &self.aspect_registry.registrations().len(),
            )
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeBridge;
    use crate::builder::RuntimeBridgeBuilder;
    use crate::input::envelope::TruthBranchIdentity;
    use crate::mapping::{
        BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
        SignalInvalidationScope, TruthPatchScope,
    };
    use crate::policy::{BridgeDiagnosticsTier, BridgeRuntimePolicy};
    use crate::snapshot::{
        BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewPolicyResolution,
        BridgeTruthViewSelector, HistoricalEvaluationDeclaration, SnapshotReadPacket,
        TruthSnapshotIdentity,
    };
    use crate::facade::BridgeHistoricalMaterializationPath;

    #[derive(Clone)]
    struct StaticSource;
    impl crate::adapter::CommittedPatchSource for StaticSource {
        fn load_committed_patch(
            &self,
            request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                crate::input::envelope::TruthPatchIdentity::new(format!(
                    "patch-for-{}",
                    request.commit_identity()
                )),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
                vec![],
            ))
        }
    }

    #[derive(Clone)]
    struct StaticSnapshotReader;
    impl crate::snapshot::TruthSnapshotReader for StaticSnapshotReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-a")
        }

        fn read_packet(
            &self,
            _request: &SnapshotReadPacket,
        ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError> {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![],
            ))
        }
    }

    impl crate::adapter::SnapshotReadSource for StaticSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn crate::snapshot::TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
            if identity.as_str() == "snapshot-a" {
                Ok(Box::new(StaticSnapshotReader))
            } else {
                Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "unknown snapshot `{}`",
                    identity.as_str()
                )))
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for StaticSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                crate::input::envelope::TruthPatchIdentity::new(format!(
                    "patch-{}",
                    branch_identity.as_str()
                )),
                TruthSnapshotIdentity::new("snapshot-a"),
                branch_identity.clone(),
                vec![],
            ))
        }
    }

    struct StaticSink;
    impl crate::adapter::InvalidationSink for StaticSink {
        fn deliver_invalidation(
            &self,
            _delivery: crate::routing::BridgeSignalInvalidationDelivery,
        ) -> Result<crate::delivery::BridgeDeliveryReceipt, crate::adapter::SignalBridgeSinkError> {
            unreachable!("policy-resolution tests do not deliver invalidations")
        }
    }

    fn runtime(policy: BridgeRuntimePolicy) -> RuntimeBridge {
        RuntimeBridgeBuilder::new()
            .with_policy(policy)
            .with_relational_source(StaticSource)
            .with_truth_branch_head_source(StaticSource)
            .with_signal_sink(StaticSink)
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("mapping"),
                TruthPatchScope::new(
                    MappingSelector::exact("profile"),
                    MappingSelector::any(),
                    MappingSelector::any(),
                ),
                SignalInvalidationScope::new("signal:profile"),
                CoarseRoutingMode::Direct,
            ))
            .build()
            .expect("runtime should build for policy-resolution tests")
    }

    #[test]
    fn runtime_admits_snapshot_bound_truth_view_policy() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );

        let resolution = runtime.resolve_truth_view_policy(&declaration);
        match resolution {
            BridgeTruthViewPolicyResolution::Admitted(policy) => {
                assert_eq!(
                    policy.retention_admission(),
                    crate::snapshot::TruthViewRetentionAdmission::SnapshotResident
                );
                assert_eq!(
                    policy.source_capability(),
                    crate::snapshot::TruthViewSourceCapability::DirectSnapshotRead
                );
            }
            BridgeTruthViewPolicyResolution::Rejected(rejection) => {
                panic!("expected admitted policy, got rejection: {}", rejection.detail())
            }
        }
    }

    #[test]
    fn runtime_rejects_required_replay_when_runtime_policy_disallows_replay_artifacts() {
        let runtime = runtime(
            BridgeRuntimePolicy::operational()
                .with_route_record_limit(8)
                .with_replay_artifacts(false),
        );
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("main"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Required,
            BridgeDiagnosticsTier::Exhaustive,
            BridgeDeliveryIntent::PrepareOnly,
        );

        let resolution = runtime.resolve_truth_view_policy(&declaration);
        match resolution {
            BridgeTruthViewPolicyResolution::Rejected(rejection) => {
                assert_eq!(
                    rejection.kind(),
                    crate::snapshot::TruthViewPolicyRejectionKind::ReplayNotPermitted
                );
            }
            BridgeTruthViewPolicyResolution::Admitted(_) => {
                panic!("expected replay policy rejection")
            }
        }
    }

    #[test]
    fn runtime_plans_truth_view_packet_from_admitted_policy() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );

        let planned = runtime
            .plan_truth_view_packet(declaration.clone(), SnapshotReadPacket::new(vec![]))
            .expect("snapshot-bound declaration should plan");

        assert_eq!(
            planned.declaration().declaration_identity(),
            declaration.declaration_identity()
        );
        assert_eq!(
            planned
                .authority_basis()
                .snapshot_identity()
                .map(|id: &TruthSnapshotIdentity| id.as_str()),
            Some("snapshot-a")
        );
    }

    #[test]
    fn runtime_admits_commit_bound_truth_view_policy() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );

        let resolution = runtime.resolve_truth_view_policy(&declaration);
        match resolution {
            BridgeTruthViewPolicyResolution::Admitted(policy) => {
                assert_eq!(
                    policy.retention_admission(),
                    crate::snapshot::TruthViewRetentionAdmission::HistoricalLookupRequired
                );
            }
            BridgeTruthViewPolicyResolution::Rejected(rejection) => {
                panic!(
                    "expected commit-bound selector admission, got rejection: {}",
                    rejection.detail()
                )
            }
        }
    }

    #[test]
    fn runtime_materializes_commit_bound_truth_view_observation() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let planned = runtime
            .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
            .expect("commit-bound declaration should plan");

        let observation = runtime
            .materialize_truth_view_observation(planned)
            .expect("commit-bound declaration should materialize");

        assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
        assert_eq!(
            observation
                .authority_basis()
                .commit_identity()
                .map(crate::input::envelope::TruthCommitIdentity::as_str),
            Some("commit-a")
        );
    }

    #[test]
    fn runtime_materializes_branch_head_truth_view_observation() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let planned = runtime
            .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
            .expect("branch-head declaration should plan");

        let observation = runtime
            .materialize_truth_view_observation(planned)
            .expect("branch-head declaration should materialize");

        assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
        assert_eq!(
            observation
                .authority_basis()
                .commit_identity()
                .map(crate::input::envelope::TruthCommitIdentity::as_str),
            Some("head-analysis")
        );
    }

    #[test]
    fn runtime_materializes_snapshot_bound_truth_view_observation() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let planned = runtime
            .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
            .expect("snapshot-bound declaration should plan");

        let observation = runtime
            .materialize_truth_view_observation(planned)
            .expect("snapshot-bound declaration should materialize");
        let validated_reads = observation
            .read_planned_packet()
            .expect("materialized observation should execute its planned packet");

        assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
        assert_eq!(observation.snapshot_token().snapshot_identity().as_str(), "snapshot-a");
        assert_eq!(validated_reads.snapshot_identity().as_str(), "snapshot-a");
    }

    #[test]
    fn runtime_canonicalizes_historical_evaluation_record() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let planned = runtime
            .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
            .expect("historical declaration should plan");
        let observation = runtime
            .materialize_truth_view_observation(planned)
            .expect("historical declaration should materialize");

        let record = runtime.canonicalize_historical_evaluation_record(&observation);

        assert_eq!(record.decision_log().snapshot_identity().as_str(), "snapshot-a");
        assert_eq!(
            record.decision_log().materialization_path(),
            BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot
        );
        assert_eq!(
            runtime
                .diagnostics()
                .last_historical_evaluation_record()
                .expect("historical record should be retained")
                .record_identity(),
            record.record_identity()
        );
    }

    #[test]
    fn runtime_lowers_identical_historical_requests_to_identical_artifacts() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let left_observation = runtime
            .materialize_truth_view_observation(
                runtime
                    .plan_truth_view_packet(declaration.clone(), SnapshotReadPacket::new(vec![]))
                    .expect("left historical declaration should plan"),
            )
            .expect("left historical declaration should materialize");
        let right_observation = runtime
            .materialize_truth_view_observation(
                runtime
                    .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                    .expect("right historical declaration should plan"),
            )
            .expect("right historical declaration should materialize");

        let left = runtime.lower_historical_evaluation_artifact(&left_observation);
        let right = runtime.lower_historical_evaluation_artifact(&right_observation);

        assert_eq!(left, right);
        assert_eq!(left.snapshot_identity().as_str(), "snapshot-a");
    }

    #[test]
    fn runtime_replays_canonical_historical_evaluation_record() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let observation = runtime
            .materialize_truth_view_observation(
                runtime
                    .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                    .expect("branch-head declaration should plan"),
            )
            .expect("branch-head declaration should materialize");
        let record = runtime.canonicalize_historical_evaluation_record(&observation);

        let replay = runtime
            .replay_canonical_historical_evaluation_record(&record)
            .expect("historical record replay should succeed");

        assert_eq!(replay.record_identity(), record.record_identity());
        assert_eq!(replay.snapshot_identity().as_str(), "snapshot-a");
    }

    #[test]
    fn runtime_replay_rejects_historical_authority_drift() {
        #[derive(Clone)]
        struct DriftSource;

        impl crate::adapter::CommittedPatchSource for DriftSource {
            fn load_committed_patch(
                &self,
                request: crate::adapter::RelationalCommittedPatchRequest,
            ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
                let snapshot = if request.commit_identity() == "commit-a" {
                    "snapshot-b"
                } else {
                    "snapshot-a"
                };
                Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                    crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                    crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                    TruthSnapshotIdentity::new(snapshot),
                    TruthBranchIdentity::new("analysis"),
                    vec![],
                ))
            }
        }

        impl crate::adapter::SnapshotReadSource for DriftSource {
            fn open_snapshot(
                &self,
                identity: &TruthSnapshotIdentity,
            ) -> Result<Box<dyn crate::snapshot::TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
                if identity.as_str() == "snapshot-b" {
                    Ok(Box::new(StaticSnapshotReader))
                } else {
                    Err(crate::adapter::RelationalBridgeSourceError::new("missing snapshot"))
                }
            }
        }

        impl crate::adapter::TruthBranchHeadSource for DriftSource {
            fn load_branch_head_patch(
                &self,
                branch_identity: &TruthBranchIdentity,
            ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
                Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                    crate::input::envelope::TruthCommitIdentity::new("head-analysis"),
                    crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                    TruthSnapshotIdentity::new("snapshot-b"),
                    branch_identity.clone(),
                    vec![],
                ))
            }
        }

        let original = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let record = original.canonicalize_historical_evaluation_record(
            &original
                .materialize_truth_view_observation(
                    original
                        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                        .expect("original historical declaration should plan"),
                )
                .expect("original historical declaration should materialize"),
        );
        let drifted = RuntimeBridgeBuilder::new()
            .with_policy(BridgeRuntimePolicy::default())
            .with_relational_source(DriftSource)
            .with_truth_branch_head_source(DriftSource)
            .with_signal_sink(StaticSink)
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("mapping"),
                TruthPatchScope::new(
                    MappingSelector::exact("profile"),
                    MappingSelector::any(),
                    MappingSelector::any(),
                ),
                SignalInvalidationScope::new("signal:profile"),
                CoarseRoutingMode::Direct,
            ))
            .build()
            .expect("drifted runtime should build");

        let error = drifted
            .replay_canonical_historical_evaluation_record(&record)
            .expect_err("historical replay should reject authority drift");

        assert_eq!(
            error.kind(),
            crate::error::BridgeReplayErrorKind::HistoricalEvaluationAuthorityMismatch
        );
        assert_eq!(
            drifted
                .diagnostics()
                .last_historical_evaluation_failure()
                .expect("historical replay mismatch should be recorded")
                .failure_class(),
            crate::facade::BridgeHistoricalEvaluationFailureClass::HistoricalReplayMismatch
        );
    }

    #[test]
    fn runtime_replay_rejects_incompatible_historical_record_version() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let record = runtime
            .canonicalize_historical_evaluation_record(
                &runtime
                    .materialize_truth_view_observation(
                        runtime
                            .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                            .expect("branch-head declaration should plan"),
                    )
                    .expect("branch-head declaration should materialize"),
            )
            .with_schema_version_for_test("forge-runtime-bridge.historical-evaluation-record.v0");

        let error = runtime
            .replay_canonical_historical_evaluation_record(&record)
            .expect_err("historical replay should reject unsupported schema versions");

        assert_eq!(
            error.kind(),
            crate::error::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRequest {
    committed_patch: crate::adapter::RelationalCommittedPatchRequest,
}

impl BridgeRouteRequest {
    pub fn for_commit(commit_identity: impl Into<Arc<str>>) -> Self {
        Self {
            committed_patch: crate::adapter::RelationalCommittedPatchRequest::new(commit_identity),
        }
    }

    pub fn commit_identity(&self) -> &str {
        self.committed_patch.commit_identity()
    }

    pub(crate) fn into_committed_patch_request(
        self,
    ) -> crate::adapter::RelationalCommittedPatchRequest {
        self.committed_patch
    }
}
