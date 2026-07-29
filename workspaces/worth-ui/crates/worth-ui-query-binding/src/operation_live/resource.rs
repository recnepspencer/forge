use worth_query::facade::{
    foundation::ObservationLaneWitness,
    installed::{collection, observation},
    runtime,
};

use crate::{
    WorthUiCollectionAllocationPolicy, WorthUiCollectionChangeConsequence,
    WorthUiCollectionChangeSourceReference, WorthUiInstalledQueryBindingReference,
    WorthUiQueryViewDefinition, WorthUiSettledSnapshotFact,
};

mod opening;

type QueryLease = observation::WorthQuerySharedLiveProjectionLease<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;

#[must_use = "a stopped disposal retains the live resource for retry"]
pub enum WorthUiOperationLiveCloseOutcome {
    Closed(WorthUiOperationLiveCloseReceipt),
    Stopped(Box<WorthUiOperationLiveCloseStop>),
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
    Ui(WorthUiOperationLiveRefreshDenial),
    Drain(Box<observation::WorthQuerySharedProjectionDrainStop>),
    Delta(Box<observation::WorthQueryConsumerInvalidationDeltaStop>),
    Readmission(Box<observation::WorthQueryConsumerInvalidationAdmissionStop>),
    Delivery(Box<collection::WorthQueryCollectionDeliveryDenial>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOperationLiveRefreshDenial {
    SourceNotAdmitted,
    ChangeOrderExhausted,
    PublicationPending,
    ResourceNotRetained,
}

impl std::fmt::Debug for WorthUiOperationLiveRefreshError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ui(denial) => formatter.debug_tuple("Ui").field(denial).finish(),
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

#[derive(Debug)]
pub enum WorthUiOperationLiveRefreshOutcome {
    NoSemanticDelivery,
    Applied(WorthUiCollectionChangeConsequence),
}

#[must_use = "operation-live resources retain a Query lease until explicitly disposed"]
pub struct WorthUiOperationLiveResource {
    installed_reference: WorthUiInstalledQueryBindingReference,
    fact: std::sync::Arc<WorthUiSettledSnapshotFact>,
    lease: QueryLease,
    consumer: collection::WorthQueryCollectionConsumerWindow,
    allocation: WorthUiCollectionAllocationPolicy,
    collection_source: Option<WorthUiCollectionChangeSourceReference>,
    next_change_order: u64,
    staged_change: Option<crate::collection_delivery::WorthUiRetainedCollectionChangeConsequence>,
    staged_change_admitted: bool,
    admitted_changes: std::collections::VecDeque<
        crate::collection_delivery::WorthUiRetainedCollectionChangeConsequence,
    >,
}

pub(crate) struct WorthUiOperationLiveSources {
    pub(crate) installed_reference: WorthUiInstalledQueryBindingReference,
    pub(crate) settlement: super::open::WorthUiOperationLiveSettlement,
}

impl WorthUiOperationLiveResource {
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
        self.collection_source = Some(WorthUiCollectionChangeSourceReference::mint());
    }

    pub(crate) fn refresh(
        &mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<WorthUiOperationLiveRefreshOutcome, WorthUiOperationLiveRefreshError> {
        if self.staged_change.is_some() {
            return Err(WorthUiOperationLiveRefreshError::Ui(
                WorthUiOperationLiveRefreshDenial::PublicationPending,
            ));
        }
        let source = self
            .collection_source
            .clone()
            .ok_or(WorthUiOperationLiveRefreshError::Ui(
                WorthUiOperationLiveRefreshDenial::SourceNotAdmitted,
            ))?;
        let change_order =
            self.next_change_order
                .checked_add(1)
                .ok_or(WorthUiOperationLiveRefreshError::Ui(
                    WorthUiOperationLiveRefreshDenial::ChangeOrderExhausted,
                ))?;
        let delivery = self
            .lease
            .drain(workspace)
            .map_err(|stop| WorthUiOperationLiveRefreshError::Drain(Box::new(stop)))?;
        if delivery.delivery().is_empty() {
            return Ok(WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery);
        }
        let delta = self
            .lease
            .consumer_invalidation_delta(delivery)
            .map_err(|stop| WorthUiOperationLiveRefreshError::Delta(Box::new(stop)))?;
        let admitted = self
            .lease
            .admit_consumer_invalidation_delta(delta, workspace)
            .map_err(|stop| WorthUiOperationLiveRefreshError::Readmission(Box::new(stop)))?;
        self.consumer
            .bind_shared_target(&admitted, workspace)
            .map_err(|stop| WorthUiOperationLiveRefreshError::Delivery(Box::new(stop)))?;
        let patch = match self.consumer.plan_patch(&admitted, workspace) {
            collection::WorthQueryCollectionDeliveryOutcome::Patch(patch) => patch,
            collection::WorthQueryCollectionDeliveryOutcome::NoDelivery(stop) => {
                return Err(WorthUiOperationLiveRefreshError::Delivery(Box::new(stop)));
            }
        };
        let receipt = self
            .consumer
            .apply_patch(patch)
            .map_err(|stop| WorthUiOperationLiveRefreshError::Delivery(Box::new(stop)))?;
        let consequence = crate::collection_delivery::mint_collection_change_consequence(
            self.installed_reference.clone(),
            source,
            change_order,
            self.allocation,
            &receipt,
        );
        self.next_change_order = change_order;
        self.staged_change = Some(consequence.retain());
        self.staged_change_admitted = false;
        Ok(WorthUiOperationLiveRefreshOutcome::Applied(consequence))
    }

    pub(crate) fn admit_collection_change(
        &mut self,
        consequence: WorthUiCollectionChangeConsequence,
    ) -> Result<
        crate::WorthUiCollectionChangeStagingReceipt,
        crate::WorthUiCollectionChangeAdmissionStop,
    > {
        let receipt = self
            .validate_collection_change(&consequence)
            .map_err(|denial| {
                crate::WorthUiCollectionChangeAdmissionStop::new(denial, consequence)
            })?;
        self.staged_change_admitted = true;
        Ok(receipt)
    }

    pub(crate) fn validate_collection_change_observation(
        &self,
        consequence: WorthUiCollectionChangeConsequence,
    ) -> Result<
        crate::WorthUiValidatedCollectionChangeObservation,
        crate::WorthUiCollectionChangeAdmissionStop,
    > {
        let receipt = match self.validate_collection_change(&consequence) {
            Ok(receipt) => receipt,
            Err(denial) => {
                return Err(crate::WorthUiCollectionChangeAdmissionStop::new(
                    denial,
                    consequence,
                ));
            }
        };
        Ok(crate::WorthUiValidatedCollectionChangeObservation::seal(
            consequence,
            receipt,
        ))
    }

    fn validate_collection_change(
        &self,
        consequence: &WorthUiCollectionChangeConsequence,
    ) -> Result<
        crate::WorthUiCollectionChangeStagingReceipt,
        crate::WorthUiCollectionChangeAdmissionDenial,
    > {
        let belongs_to_resource = consequence.installed_reference() == &self.installed_reference
            && self
                .collection_source
                .as_ref()
                .is_some_and(|source| source == consequence.source())
            && consequence.change_order() == self.next_change_order
            && self
                .staged_change
                .as_ref()
                .is_some_and(|staged| staged.matches(consequence));
        if !belongs_to_resource {
            return Err(crate::WorthUiCollectionChangeAdmissionDenial::StaleOrForeignConsequence);
        }
        if self.staged_change_admitted {
            return Err(crate::WorthUiCollectionChangeAdmissionDenial::AlreadyAdmitted);
        }
        Ok(crate::WorthUiCollectionChangeStagingReceipt::from_consequence(consequence))
    }

    pub(crate) fn publish_staged_collection_change(&mut self) -> bool {
        if !self.staged_change_admitted {
            return false;
        }
        let Some(consequence) = self.staged_change.take() else {
            return false;
        };
        self.staged_change_admitted = false;
        self.admitted_changes.push_back(consequence);
        true
    }

    pub(crate) fn retry_collection_change_handoff(
        &self,
    ) -> Result<WorthUiCollectionChangeConsequence, crate::WorthUiCollectionChangeHandoffRetryDenial>
    {
        if self.staged_change_admitted {
            return Err(
                crate::WorthUiCollectionChangeHandoffRetryDenial::AlreadyAdmittedToFrameworkTurn,
            );
        }
        self.staged_change
            .as_ref()
            .map(crate::collection_delivery::WorthUiRetainedCollectionChangeConsequence::handoff)
            .ok_or(crate::WorthUiCollectionChangeHandoffRetryDenial::NoUnpublishedChange)
    }

    pub fn admitted_collection_change_count(&self) -> usize {
        self.admitted_changes.len()
    }

    pub fn staged_collection_change_count(&self) -> usize {
        usize::from(self.staged_change.is_some())
    }

    pub(crate) fn collection_change_observation(
        &self,
    ) -> crate::WorthUiOperationLiveChangeObservation {
        crate::WorthUiOperationLiveChangeObservation::new(
            self.staged_collection_change_count(),
            self.admitted_collection_change_count(),
            self.next_change_order,
        )
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
            collection_source,
            next_change_order,
            staged_change,
            staged_change_admitted,
            admitted_changes,
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
                WorthUiOperationLiveCloseOutcome::Stopped(Box::new(WorthUiOperationLiveCloseStop {
                    resource: Self {
                        installed_reference,
                        fact,
                        lease,
                        consumer,
                        allocation,
                        collection_source,
                        next_change_order,
                        staged_change,
                        staged_change_admitted,
                        admitted_changes,
                    },
                    query_error,
                    counters,
                }))
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
