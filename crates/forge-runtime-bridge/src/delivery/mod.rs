use crate::diagnostics::{BridgeFailureSource, BridgeRouteRecord};
use crate::error::{BridgeDeliveryError, BridgeDeliveryErrorKind, BridgeErrorContext, BridgeSnapshotReadCoordinate};
use crate::facade::RuntimeBridge;
use crate::adapter::SnapshotReaderPool;
use crate::routing::outcome::BridgeRouteOutcomeReference;
use crate::routing::result::{BridgeRouteResult, BridgeRouteResultSummary};
use crate::routing::BridgePlannedRoute;
use crate::routing::{BridgeExecutionCounts, BridgeRouteSourceSummary};
use crate::snapshot::{
    validate_snapshot_read_result_contract, AdmittedSnapshotContext, BridgeSnapshotContext,
    SnapshotReadPacket, TruthSnapshotReader,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDeliveryReceipt {
    delivered_target_count: usize,
    snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
}

impl BridgeDeliveryReceipt {
    pub fn new(
        delivered_target_count: usize,
        snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
    ) -> Self {
        Self {
            delivered_target_count,
            snapshot_identity,
        }
    }

    pub fn delivered_target_count(&self) -> usize {
        self.delivered_target_count
    }

    pub fn snapshot_identity(&self) -> &crate::snapshot::TruthSnapshotIdentity {
        &self.snapshot_identity
    }
}

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
    pub(crate) fn new(
        prepared: crate::routing::planning::BridgePreparedDelivery,
    ) -> Self {
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

    pub(crate) fn failure_source(&self) -> BridgeFailureSource {
        self.inner.failure_source()
    }
}

pub(crate) fn deliver_planned_route(
    runtime: &RuntimeBridge,
    route: BridgePlannedRoute,
) -> Result<BridgeRouteResult, BridgeDeliveryError> {
    let prepared = prepare_planned_route_for_delivery(route);
    deliver_prepared_route(runtime, prepared)
}

pub(crate) fn prepare_planned_route_for_delivery(
    route: BridgePlannedRoute,
) -> BridgePreparedDeliveryRequest {
    BridgePreparedDeliveryRequest::new(route.into_prepared_delivery())
}

pub(crate) fn deliver_prepared_route(
    runtime: &RuntimeBridge,
    prepared: BridgePreparedDeliveryRequest,
) -> Result<BridgeRouteResult, BridgeDeliveryError> {
    let failure_base = prepared.failure_source();
    let route_identity = prepared.routing_summary().route_identity().clone();
    let lowering_plan = prepared.validated_lowering_plan().plan();
    let mut counters = *prepared.counters();
    let read_packet = prepared.read_packet();
    let snapshot_reader = match open_snapshot_reader(runtime, lowering_plan.source_snapshot()) {
        Ok(snapshot_reader) => snapshot_reader,
        Err(error) => {
            let failure = BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotAcquisitionFailure,
                format!(
                    "Bridge failed to open snapshot `{}`: {error}",
                    lowering_plan.source_snapshot().as_str()
                ),
            )
            .with_context(delivery_context(
                route_identity.clone(),
                lowering_plan.source_snapshot().clone(),
            ));
            return Err(reject_delivery(
                runtime,
                failure_base.clone(),
                failure,
            ));
        }
    };
    let snapshot = match AdmittedSnapshotContext::admit_for(
        BridgeSnapshotContext::bind(snapshot_reader),
        lowering_plan.source_snapshot(),
    ) {
        Ok(snapshot) => snapshot,
        Err(bound_snapshot_identity) => {
            let counters = counters.with_snapshot_identity_mismatch();
            let error = BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotIdentityMismatch,
                format!(
                    "Snapshot reader bound `{}` but planned route required `{}`.",
                    bound_snapshot_identity.as_str(),
                    lowering_plan.source_snapshot().as_str()
                ),
            )
            .with_context(delivery_context(
                route_identity.clone(),
                lowering_plan.source_snapshot().clone(),
            ));
            return Err(reject_delivery(
                runtime,
                failure_base.clone().with_counters(counters),
                error,
            ));
        }
    };
    let read_result = snapshot.read_packet(read_packet).map_err(|error| {
        BridgeDeliveryError::new(
            BridgeDeliveryErrorKind::SnapshotReadFailure,
            format!(
                "Bridge failed to execute packetized snapshot reads for `{}`: {error}",
                lowering_plan.source_snapshot().as_str()
            ),
        )
        .with_context(delivery_context(
            route_identity.clone(),
            lowering_plan.source_snapshot().clone(),
        ))
    })?;
    if read_result.snapshot_identity() != lowering_plan.source_snapshot() {
        let counters = counters.with_snapshot_identity_mismatch();
        let error = BridgeDeliveryError::new(
            BridgeDeliveryErrorKind::SnapshotIdentityMismatch,
            format!(
                "Snapshot read returned `{}` but planned route required `{}`.",
                read_result.snapshot_identity().as_str(),
                lowering_plan.source_snapshot().as_str()
            ),
        )
        .with_context(delivery_context(
            route_identity.clone(),
            lowering_plan.source_snapshot().clone(),
        ));
        return Err(reject_delivery(
            runtime,
            failure_base.clone().with_counters(counters),
            error,
        ));
    }
    counters = counters.with_validation_record_count(read_result.records().len());
    let validated_reads =
        match validate_snapshot_read_result_contract(&read_packet, read_result) {
            Ok(validated_reads) => validated_reads,
            Err(error) => {
                let failure = BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                    format!(
                        "Bridge snapshot read contract failed for `{}`: {error}",
                        lowering_plan.source_snapshot().as_str()
                    ),
                )
                .with_context(
                    delivery_context(
                        route_identity.clone(),
                        lowering_plan.source_snapshot().clone(),
                    )
                    .with_snapshot_read(first_snapshot_read_coordinate(read_packet)),
                );
                return Err(reject_delivery(
                    runtime,
                    failure_base.clone(),
                    failure,
                ));
            }
        };

    let lowered = prepared.into_inner().into_lowered_execution(counters);
    let lowered_failure_base = lowered.failure_source();
    let delivery = crate::routing::BridgeSignalInvalidationDelivery::new(
        lowered.artifact(),
        lowered.contract_proof(),
    );
    let receipt = runtime
        .signal_sink
        .deliver_invalidation(delivery)
        .map_err(|error| {
            reject_delivery(
                runtime,
                lowered_failure_base.clone(),
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SignalSinkRejection,
                    format!("Signal bridge sink rejected invalidation delivery: {error}"),
                )
                .with_context(
                    delivery_context(
                        lowered.artifact().route_identity().clone(),
                        lowered.artifact().source_snapshot().clone(),
                    )
                    .with_invalidation_identity(lowered.artifact().invalidation_identity().clone())
                    .with_subscription_slice_identity(
                        lowered.artifact().subscription_slice_identity().clone(),
                    ),
                ),
            )
        })?;
    let result_summary = BridgeRouteResultSummary::new(
        BridgeRouteOutcomeReference::new(
            lowered.artifact().route_identity().clone(),
            lowered.artifact().invalidation_identity().clone(),
            BridgeRouteSourceSummary::new(
                lowered.artifact().source_commit().clone(),
                lowered.artifact().source_patch().clone(),
                lowered.artifact().source_snapshot().clone(),
            ),
            lowered.artifact().subscription_slice_identity().clone(),
        ),
        lowered.contract_proof().clone(),
        lowered.routing_summary().routing_entry_count(),
        BridgeExecutionCounts::new(
            lowered.artifact().invalidation_targets().targets().len(),
            lowered.artifact().subscription_slices().len(),
            validated_reads.records().len(),
        ),
        receipt.delivered_target_count(),
    );
    runtime.diagnostic_sink.record_route(BridgeRouteRecord::new(
        lowered.artifact().route_identity().clone(),
        lowered.artifact().invalidation_identity().clone(),
        lowered.artifact().source_commit().clone(),
        lowered.artifact().source_patch().clone(),
        lowered.artifact().source_snapshot().clone(),
        lowered.contract_proof().clone(),
        lowered.artifact().subscription_slice_identity().clone(),
        std::sync::Arc::clone(lowered.route_record_entries()),
        std::sync::Arc::clone(lowered.artifact().subscription_slices().shared()),
        std::sync::Arc::clone(lowered.artifact().invalidation_targets().shared()),
        *lowered.counters(),
    ));

    Ok(BridgeRouteResult::new(
        lowered.routing_summary().clone(),
        result_summary,
        *lowered.artifact().counters(),
        lowered.artifact().clone(),
        receipt,
    ))
}

pub(crate) fn prepare_signal_evaluation(
    runtime: &RuntimeBridge,
    route: BridgePlannedRoute,
) -> Result<BridgeSignalEvaluationRequest, BridgeDeliveryError> {
    let prepared = prepare_planned_route_for_delivery(route);
    let counters = *prepared.counters();
    let read_packet = prepared.read_packet().clone();
    let snapshot = open_planned_snapshot(runtime, prepared.validated_lowering_plan().plan().source_snapshot())?;
    let artifact = prepared.into_inner().into_lowered_execution(counters).artifact().clone();

    Ok(BridgeSignalEvaluationRequest::new(artifact, read_packet, snapshot))
}

fn open_planned_snapshot(
    runtime: &RuntimeBridge,
    snapshot_identity: &crate::snapshot::TruthSnapshotIdentity,
) -> Result<AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>, BridgeDeliveryError> {
    let snapshot_reader = open_snapshot_reader(runtime, snapshot_identity).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotAcquisitionFailure,
                format!(
                    "Bridge failed to open snapshot `{}`: {error}",
                    snapshot_identity.as_str()
                ),
            )
            .with_context(BridgeErrorContext::snapshot(snapshot_identity.clone()))
        })?;
    let snapshot = BridgeSnapshotContext::bind(snapshot_reader);
    let admitted = AdmittedSnapshotContext::admit_for(snapshot, snapshot_identity).map_err(
        |bound_snapshot_identity| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotIdentityMismatch,
                format!(
                    "Snapshot reader bound `{}` but planned route required `{}`.",
                    bound_snapshot_identity.as_str(),
                    snapshot_identity.as_str()
                ),
            )
            .with_context(BridgeErrorContext::snapshot(snapshot_identity.clone()))
        },
    )?;

    if admitted.snapshot_identity() != snapshot_identity {
        return Err(BridgeDeliveryError::new(
            BridgeDeliveryErrorKind::SnapshotIdentityMismatch,
            format!(
                "Snapshot reader bound `{}` but planned route required `{}`.",
                admitted.snapshot_identity().as_str(),
                snapshot_identity.as_str()
            ),
        )
        .with_context(BridgeErrorContext::snapshot(snapshot_identity.clone())));
    }

    Ok(admitted)
}

fn delivery_context(
    route_identity: crate::routing::BridgeRouteIdentity,
    snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
) -> BridgeErrorContext {
    BridgeErrorContext::delivery(route_identity, snapshot_identity)
}

fn reject_delivery(
    runtime: &RuntimeBridge,
    source: BridgeFailureSource,
    error: BridgeDeliveryError,
) -> BridgeDeliveryError {
    runtime.diagnostic_sink.record_delivery_failure(source, &error);
    error
}

fn open_snapshot_reader(
    runtime: &RuntimeBridge,
    snapshot_identity: &crate::snapshot::TruthSnapshotIdentity,
) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
    if let Some(pool) = runtime.snapshot_reader_pool.as_ref() {
        let pool = std::sync::Arc::clone(pool);
        let reader = pool.acquire(snapshot_identity)?;
        return Ok(Box::new(PooledTruthSnapshotReader::new(pool, reader)));
    }

    runtime.snapshot_read_source.open_snapshot(snapshot_identity)
}

struct PooledTruthSnapshotReader {
    pool: std::sync::Arc<dyn SnapshotReaderPool>,
    reader: Option<Box<dyn TruthSnapshotReader>>,
}

impl PooledTruthSnapshotReader {
    fn new(
        pool: std::sync::Arc<dyn SnapshotReaderPool>,
        reader: Box<dyn TruthSnapshotReader>,
    ) -> Self {
        Self {
            pool,
            reader: Some(reader),
        }
    }

    fn reader(&self) -> &dyn TruthSnapshotReader {
        self.reader
            .as_deref()
            .expect("pooled snapshot reader should be present while borrowed")
    }
}

impl TruthSnapshotReader for PooledTruthSnapshotReader {
    fn snapshot_identity(&self) -> crate::snapshot::TruthSnapshotIdentity {
        self.reader().snapshot_identity()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError> {
        self.reader().read_packet(request)
    }
}

impl Drop for PooledTruthSnapshotReader {
    fn drop(&mut self) {
        if let Some(reader) = self.reader.take() {
            self.pool.release(reader);
        }
    }
}

fn first_snapshot_read_coordinate(packet: &SnapshotReadPacket) -> BridgeSnapshotReadCoordinate {
    let read = &packet.reads()[0];
    match (read.surface_label(), read.slice_kind()) {
        (Some(surface_label), Some(slice_kind)) => BridgeSnapshotReadCoordinate::new_subscription_slice(
            read.request_key(),
            read.entity_identity(),
            read.aspect_label(),
            surface_label,
            slice_kind.clone(),
        ),
        _ => BridgeSnapshotReadCoordinate::new_coarse(
            read.request_key(),
            read.entity_identity(),
            read.aspect_label(),
        ),
    }
}
