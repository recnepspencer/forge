use worth_query::facade::{
    foundation::ObservationLaneWitness,
    installed::{collection, observation},
    runtime,
};

use crate::{
    WorthUiCollectionAllocationPolicy, WorthUiCollectionPatchConsequences,
    WorthUiInstalledQueryBindingReference, WorthUiOperationLiveOpenError,
    WorthUiQueryViewDefinition, WorthUiSettledSnapshotFact,
};

type QueryLease = observation::WorthQuerySharedLiveProjectionLease<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;

#[must_use = "a stopped disposal retains the live resource for retry"]
pub enum WorthUiOperationLiveCloseOutcome {
    Closed(WorthUiOperationLiveCloseReceipt),
    Stopped(WorthUiOperationLiveCloseStop),
}

#[must_use = "a disposal stop retains the exact Query failure and retryable resource"]
pub struct WorthUiOperationLiveCloseStop {
    resource: WorthUiOperationLiveResource,
    query_error: runtime::WorthQueryRuntimeError,
    counters: runtime::WorthQuerySharedLeaseReleaseCounters,
}

pub struct WorthUiOperationLiveCloseReceipt {
    definition: WorthUiQueryViewDefinition,
    owner_terminal: bool,
    counters: runtime::WorthQuerySharedLeaseReleaseCounters,
}

pub enum WorthUiOperationLiveRefreshError {
    Drain(observation::WorthQuerySharedProjectionDrainStop),
    Delta(observation::WorthQueryConsumerInvalidationDeltaStop),
    Readmission(observation::WorthQueryConsumerInvalidationAdmissionStop),
    Delivery(collection::WorthQueryCollectionDeliveryDenial),
}

impl std::fmt::Debug for WorthUiOperationLiveRefreshError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Drain(stop) => formatter.debug_tuple("Drain").field(stop.error()).finish(),
            Self::Delta(stop) => formatter.debug_tuple("Delta").field(&stop.kind()).finish(),
            Self::Readmission(stop) => formatter
                .debug_tuple("Readmission")
                .field(&stop.kind())
                .finish(),
            Self::Delivery(stop) => formatter
                .debug_tuple("Delivery")
                .field(&stop.kind())
                .finish(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiOperationLiveRefreshOutcome {
    NoSemanticDelivery,
    Applied(WorthUiCollectionPatchConsequences),
}

#[must_use = "operation-live resources retain a Query lease until explicitly disposed"]
pub struct WorthUiOperationLiveResource {
    installed_reference: WorthUiInstalledQueryBindingReference,
    fact: std::sync::Arc<WorthUiSettledSnapshotFact>,
    lease: QueryLease,
    consumer: collection::WorthQueryCollectionConsumerWindow,
    allocation: WorthUiCollectionAllocationPolicy,
}

pub(crate) struct WorthUiOperationLiveSources {
    pub(crate) installed_reference: WorthUiInstalledQueryBindingReference,
    pub(crate) settled: super::open::Settled,
}

impl WorthUiOperationLiveResource {
    pub(crate) fn open(
        sources: WorthUiOperationLiveSources,
        breadth: collection::WorthQueryCollectionWindowBreadth,
        allocation: WorthUiCollectionAllocationPolicy,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<Self, WorthUiOperationLiveOpenError> {
        let WorthUiOperationLiveSources {
            installed_reference,
            settled,
        } = sources;
        let fact = std::sync::Arc::new(WorthUiSettledSnapshotFact::from_settled(&settled));
        let consumer = settled
            .prepare_collection_consumer(breadth)
            .map_err(WorthUiOperationLiveOpenError::CollectionConsumer)?;
        let promoted = match settled.into_lifecycle().promote(workspace) {
            observation::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
            stopped => {
                return Err(WorthUiOperationLiveOpenError::Promotion(Box::new(stopped)));
            }
        };
        let lease = match promoted.into_managed_lease(workspace) {
            observation::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
            observation::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
                return Err(WorthUiOperationLiveOpenError::LeaseAdmission(Box::new(
                    stop,
                )));
            }
        };
        Ok(Self {
            installed_reference,
            fact,
            lease,
            consumer,
            allocation,
        })
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        self.installed_reference.definition()
    }

    pub fn installed_reference(&self) -> &WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }

    pub fn fact(&self) -> &WorthUiSettledSnapshotFact {
        &self.fact
    }

    pub(crate) fn shared_fact(&self) -> std::sync::Arc<WorthUiSettledSnapshotFact> {
        std::sync::Arc::clone(&self.fact)
    }

    pub(crate) fn attach_source_coordinates(
        &mut self,
        generation: crate::WorthUiSettledSnapshotSourceGeneration,
        order: crate::WorthUiSettledSnapshotSourceOrder,
    ) {
        std::sync::Arc::get_mut(&mut self.fact)
            .expect("coordinates attach before live fact sharing")
            .attach_source_coordinates(generation, order);
    }

    pub fn lease_identity(&self) -> &str {
        self.lease.identity()
    }

    pub fn rows(&self) -> &[collection::WorthQueryCollectionRowHandle] {
        self.consumer.rows()
    }

    pub fn refresh(
        &mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<WorthUiOperationLiveRefreshOutcome, WorthUiOperationLiveRefreshError> {
        let delivery = self
            .lease
            .drain(workspace)
            .map_err(WorthUiOperationLiveRefreshError::Drain)?;
        if delivery.delivery().is_empty() {
            return Ok(WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery);
        }
        let delta = self
            .lease
            .consumer_invalidation_delta(delivery)
            .map_err(WorthUiOperationLiveRefreshError::Delta)?;
        let admitted = self
            .lease
            .admit_consumer_invalidation_delta(delta, workspace)
            .map_err(WorthUiOperationLiveRefreshError::Readmission)?;
        self.consumer
            .bind_shared_target(&admitted, workspace)
            .map_err(WorthUiOperationLiveRefreshError::Delivery)?;
        let patch = match self.consumer.plan_patch(&admitted, workspace) {
            collection::WorthQueryCollectionDeliveryOutcome::Patch(patch) => patch,
            collection::WorthQueryCollectionDeliveryOutcome::NoDelivery(stop) => {
                return Err(WorthUiOperationLiveRefreshError::Delivery(stop));
            }
        };
        let receipt = self
            .consumer
            .apply_patch(patch)
            .map_err(WorthUiOperationLiveRefreshError::Delivery)?;
        Ok(WorthUiOperationLiveRefreshOutcome::Applied(
            WorthUiCollectionPatchConsequences::from_query_receipt(&receipt, self.allocation),
        ))
    }

    pub fn close(
        self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> WorthUiOperationLiveCloseOutcome {
        let Self {
            installed_reference,
            fact,
            lease,
            consumer,
            allocation,
        } = self;
        match lease.dispose(workspace) {
            observation::WorthQuerySharedProjectionDisposalOutcome::Disposed(disposed) => {
                WorthUiOperationLiveCloseOutcome::Closed(WorthUiOperationLiveCloseReceipt {
                    definition: installed_reference.definition().clone(),
                    owner_terminal: disposed.release().owner_terminal(),
                    counters: disposed.release().counters(),
                })
            }
            observation::WorthQuerySharedProjectionDisposalOutcome::Stopped(stop) => {
                let (lease, query_error, counters) = stop.into_parts();
                WorthUiOperationLiveCloseOutcome::Stopped(WorthUiOperationLiveCloseStop {
                    resource: Self {
                        installed_reference,
                        fact,
                        lease,
                        consumer,
                        allocation,
                    },
                    query_error,
                    counters,
                })
            }
        }
    }
}

impl WorthUiOperationLiveCloseStop {
    pub fn query_error(&self) -> &runtime::WorthQueryRuntimeError {
        &self.query_error
    }

    pub const fn counters(&self) -> runtime::WorthQuerySharedLeaseReleaseCounters {
        self.counters
    }

    pub fn into_resource(self) -> WorthUiOperationLiveResource {
        self.resource
    }
}

impl WorthUiOperationLiveCloseReceipt {
    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn owner_terminal(&self) -> bool {
        self.owner_terminal
    }

    pub fn counters(&self) -> runtime::WorthQuerySharedLeaseReleaseCounters {
        self.counters
    }
}
