use worth_query::facade::{
    foundation,
    installed::{self, collection, observation, operation},
    runtime,
};

use crate::{
    operation_live::resource::WorthUiOperationLiveSources, WorthUiCollectionAllocationPolicy,
    WorthUiInstalledQueryBindingReference, WorthUiOperationLiveResource,
    WorthUiQueryConsumerRequirements, WorthUiQueryOperationAttemptDenial,
    WorthUiSnapshotConsumerPreparationDenial,
};

pub(crate) type Settled = operation::WorthQuerySettledDomainProjection<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    foundation::ObservationLaneWitness,
>;
type Deferred = operation::WorthQueryDeferredDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    foundation::ObservationLaneWitness,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOperationLiveOpenRequest {
    requirements: WorthUiQueryConsumerRequirements,
    breadth: collection::WorthQueryCollectionWindowBreadth,
    allocation: WorthUiCollectionAllocationPolicy,
}

impl WorthUiOperationLiveOpenRequest {
    pub fn new(
        requirements: WorthUiQueryConsumerRequirements,
        breadth: collection::WorthQueryCollectionWindowBreadth,
        allocation: WorthUiCollectionAllocationPolicy,
    ) -> Self {
        Self {
            requirements,
            breadth,
            allocation,
        }
    }
}

pub enum WorthUiOperationLiveOpenError {
    Attempt(WorthUiQueryOperationAttemptDenial),
    Preparation(WorthUiSnapshotConsumerPreparationDenial),
    ProjectionRequest(operation::WorthQueryNativeProjectionRequestDenial),
    Deferred(Box<Deferred>),
    Execution(operation::WorthQueryBoundExecutionDenial),
    Publication(operation::WorthQueryPublicationDenial),
    Consumption(operation::WorthQueryProgressionDenial),
    Settlement(operation::WorthQueryProgressionDenial),
    CollectionConsumer(installed::collection::WorthQueryCollectionConsumerPreparationDenial),
    Promotion(
        Box<
            observation::WorthQueryProjectionPromotionOutcome<
                crate::WorthUiDomainEntry,
                crate::WorthUiSnapshotMeasurement,
                crate::WorthUiSnapshotMeasurementFamily,
                foundation::ObservationLaneWitness,
            >,
        >,
    ),
    LeaseAdmission(
        Box<
            observation::WorthQueryProjectionLeaseAdmissionStop<
                crate::WorthUiDomainEntry,
                crate::WorthUiSnapshotMeasurement,
                crate::WorthUiSnapshotMeasurementFamily,
                foundation::ObservationLaneWitness,
            >,
        >,
    ),
}

impl std::fmt::Debug for WorthUiOperationLiveOpenError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Attempt(_) => "Attempt",
            Self::Preparation(_) => "Preparation",
            Self::ProjectionRequest(_) => "ProjectionRequest",
            Self::Deferred(_) => "Deferred",
            Self::Execution(_) => "Execution",
            Self::Publication(_) => "Publication",
            Self::Consumption(_) => "Consumption",
            Self::Settlement(_) => "Settlement",
            Self::CollectionConsumer(_) => "CollectionConsumer",
            Self::Promotion(_) => "Promotion",
            Self::LeaseAdmission(_) => "LeaseAdmission",
        };
        formatter
            .debug_tuple(name)
            .field(&"exact stop retained")
            .finish()
    }
}

pub(crate) fn open_operation_live_resource(
    reference: WorthUiInstalledQueryBindingReference,
    request: WorthUiOperationLiveOpenRequest,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> Result<WorthUiOperationLiveResource, WorthUiOperationLiveOpenError> {
    let settled = settle_once(&reference, request.requirements, workspace)?;
    WorthUiOperationLiveResource::open(
        WorthUiOperationLiveSources {
            installed_reference: reference,
            settled,
        },
        request.breadth,
        request.allocation,
        workspace,
    )
}

fn settle_once(
    reference: &WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> Result<Settled, WorthUiOperationLiveOpenError> {
    let gateway = reference
        .enter_snapshot_attempt(workspace)
        .map_err(WorthUiOperationLiveOpenError::Attempt)?;
    let prepared = gateway
        .prepare_snapshot_consumer(requirements)
        .map_err(WorthUiOperationLiveOpenError::Preparation)?;
    let (_, bound, consumer, _) = prepared.into_parts();
    let mut projection = consumer.into_query_contract().projection_request();
    projection
        .select_display_native_field_name("value")
        .map_err(WorthUiOperationLiveOpenError::ProjectionRequest)?;
    let projection = projection
        .build()
        .map_err(WorthUiOperationLiveOpenError::ProjectionRequest)?;
    let executed = match installed::transition::execution(bound.execute((), workspace)) {
        installed::transition::WorthQueryExecutionTransition::Executed(executed) => executed,
        installed::transition::WorthQueryExecutionTransition::Deferred(deferred) => {
            return Err(WorthUiOperationLiveOpenError::Deferred(Box::new(deferred)));
        }
        installed::transition::WorthQueryExecutionTransition::Denied(stop)
        | installed::transition::WorthQueryExecutionTransition::Stale(stop)
        | installed::transition::WorthQueryExecutionTransition::RebindRequired(stop)
        | installed::transition::WorthQueryExecutionTransition::Failed(stop) => {
            return Err(WorthUiOperationLiveOpenError::Execution(stop));
        }
    };
    let published = match installed::transition::publication(executed.publish()) {
        installed::transition::WorthQueryPublicationTransition::Published(value) => value,
        installed::transition::WorthQueryPublicationTransition::Denied(stop)
        | installed::transition::WorthQueryPublicationTransition::Stale(stop)
        | installed::transition::WorthQueryPublicationTransition::RebindRequired(stop)
        | installed::transition::WorthQueryPublicationTransition::Failed(stop) => {
            return Err(WorthUiOperationLiveOpenError::Publication(stop));
        }
    };
    let consumed = match installed::transition::consumption(published.consume_bound(projection)) {
        installed::transition::WorthQueryConsumptionTransition::Consumed(value) => value,
        installed::transition::WorthQueryConsumptionTransition::Denied(stop)
        | installed::transition::WorthQueryConsumptionTransition::Deferred(stop)
        | installed::transition::WorthQueryConsumptionTransition::Stale(stop)
        | installed::transition::WorthQueryConsumptionTransition::RebindRequired(stop)
        | installed::transition::WorthQueryConsumptionTransition::Failed(stop) => {
            return Err(WorthUiOperationLiveOpenError::Consumption(stop));
        }
    };
    match installed::transition::settlement(consumed.settle()) {
        installed::transition::WorthQuerySettlementTransition::Settled(value) => Ok(value),
        installed::transition::WorthQuerySettlementTransition::Denied(stop)
        | installed::transition::WorthQuerySettlementTransition::Stale(stop)
        | installed::transition::WorthQuerySettlementTransition::RebindRequired(stop)
        | installed::transition::WorthQuerySettlementTransition::Failed(stop) => {
            Err(WorthUiOperationLiveOpenError::Settlement(stop))
        }
    }
}
