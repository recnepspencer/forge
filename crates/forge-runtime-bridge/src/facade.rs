//! Public API boundary for `forge-runtime-bridge`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

use std::sync::Arc;

use crate::mapping::{FrozenAspectMappingRegistry, FrozenMappingRegistry};

pub use crate::adapter::{
    CommittedPatchSource, InvalidationSink, RelationalBridgeSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, SignalBridgeSink, SignalBridgeSinkError, SnapshotReadSource,
    SnapshotReaderPool,
};
use crate::diagnostics::DiagnosticSink;
pub use crate::builder::RuntimeBridgeBuilder;
pub use crate::delivery::{
    BridgeDeliveryReceipt, BridgePreparedDeliveryRequest, BridgeSignalEvaluationRequest,
};
pub use crate::diagnostics::{
    BridgeCanonicalRouteRecord, BridgeDiagnosticsFacade, BridgeDiagnosticsHandle,
    BridgeContractDiagnosticsRecord, BridgeFailureClass, BridgeFailureRecord,
    BridgeLoweringDiagnosticsRecord, BridgeReplayRecord, BridgeReplaySummary,
    BridgeRouteExplanation, BridgeRouteExplanationEntry, BridgeRouteRecord, BridgeRouteRecordMatch,
    BridgeRouteRecordEntry, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2,
};
pub use crate::error::{
    BridgeBuildError, BridgeBuildErrorKind, BridgeDeliveryError, BridgeDeliveryErrorKind,
    BridgeErrorContext, BridgePatchCoordinate, BridgeReplayError, BridgeReplayErrorKind,
    BridgeRouteError, BridgeRouteErrorKind, BridgeSnapshotReadCoordinate,
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
pub use crate::policy::{BridgeDiagnosticsRetentionBudget, BridgeDiagnosticsTier, BridgeRuntimePolicy};
pub use crate::routing::{
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeInvalidationTarget,
    BridgeLoweringPlanSummary, BridgeLoweringProvenance, BridgeLoweringSummary,
    BridgePlannedRoute, BridgePlanningProvenance, BridgePlanningSummary, BridgeRouteIdentity,
    BridgeRouteContractProof, BridgeRouteOutcomeReference, BridgeRouteResult, BridgeRouteResultSummary,
    BridgeRoutingCounters, BridgeRoutingSummary, BridgeExecutionCounts, BridgeLineageContext, BridgeMappingContext,
    BridgeRouteSourceSummary,
    FineGrainedMatchOutcome, FineGrainedMatchStatus, BridgeSignalInvalidationDelivery, BridgeSubscriptionSlice,
    BridgeSubscriptionSliceIdentity, CanonicalInvalidationTargets, CanonicalSubscriptionSlices,
};
pub use crate::snapshot::{
    AdmittedSnapshotContext, BridgeSnapshotContext, BridgeSnapshotReadError, BridgeSnapshotToken,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    TruthSnapshotIdentity, TruthSnapshotReader, ValidatedSnapshotReadPacketResult,
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

    pub fn diagnostics(&self) -> &BridgeDiagnosticsFacade {
        &self.diagnostics
    }

    pub(crate) fn new(
        policy: BridgeRuntimePolicy,
        committed_patch_source: Arc<dyn CommittedPatchSource>,
        snapshot_read_source: Arc<dyn SnapshotReadSource>,
        signal_sink: Arc<dyn InvalidationSink>,
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
