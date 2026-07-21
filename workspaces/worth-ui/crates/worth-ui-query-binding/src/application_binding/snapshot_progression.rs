use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation::ObservationLaneWitness, read, runtime};

use super::{
    WorthUiPreparedSnapshotConsumer, WorthUiQueryConsumerRequirements, WorthUiSettledSnapshotFact,
};

type Executed = domain::WorthQueryExecutedDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
    read::WorthQueryReadCompletion,
>;
type Published = domain::WorthQueryPublishedDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type Consumed = domain::WorthQueryConsumedDomainProjection<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type Settled = domain::WorthQuerySettledDomainProjection<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type ConsumerBoundary = domain::WorthQueryConsumerBoundary<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type Deferred = domain::WorthQueryDeferredDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;

pub type WorthUiSnapshotConsumerExecutionOutcome = TransitionOutcome<
    WorthUiExecutedSnapshotConsumer,
    domain::WorthQueryBoundExecutionDenial,
    WorthUiDeferredSnapshotConsumer,
    domain::WorthQueryBoundExecutionDenial,
    domain::WorthQueryBoundExecutionDenial,
    domain::WorthQueryBoundExecutionDenial,
>;
pub type WorthUiSnapshotProjectionPublicationOutcome = TransitionOutcome<
    WorthUiPublishedSnapshotConsumer,
    domain::WorthQueryPublicationDenial,
    std::convert::Infallible,
    domain::WorthQueryPublicationDenial,
    domain::WorthQueryPublicationDenial,
    domain::WorthQueryPublicationDenial,
>;
pub type WorthUiSnapshotProjectionConsumptionOutcome = TransitionOutcome<
    WorthUiConsumedSnapshotProjection,
    domain::WorthQueryProgressionDenial,
    domain::WorthQueryProgressionDenial,
    domain::WorthQueryProgressionDenial,
    domain::WorthQueryProgressionDenial,
    domain::WorthQueryProgressionDenial,
>;
pub type WorthUiSnapshotProjectionSettlementOutcome = TransitionOutcome<
    WorthUiSettledSnapshotProjection,
    domain::WorthQueryProgressionDenial,
    std::convert::Infallible,
    domain::WorthQueryProgressionDenial,
    domain::WorthQueryProgressionDenial,
    domain::WorthQueryProgressionDenial,
>;

pub struct WorthUiDeferredSnapshotConsumer {
    deferred: Deferred,
    consumer: ConsumerBoundary,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

impl std::fmt::Debug for WorthUiDeferredSnapshotConsumer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiDeferredSnapshotConsumer")
            .field("binding_identity", &self.deferred.binding_identity())
            .field("requirements", &self.requirements)
            .finish_non_exhaustive()
    }
}

pub struct WorthUiExecutedSnapshotConsumer {
    executed: Executed,
    consumer: ConsumerBoundary,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

pub struct WorthUiPublishedSnapshotConsumer {
    published: Published,
    consumer: ConsumerBoundary,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

pub struct WorthUiConsumedSnapshotProjection {
    consumed: Consumed,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

pub struct WorthUiSettledSnapshotProjection {
    settled: Settled,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
    fact: std::sync::Arc<WorthUiSettledSnapshotFact>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExactSettledSnapshotEvidence {
    installed_reference: crate::WorthUiInstalledQueryBindingReference,
    query_binding_identity: String,
    settlement_identity: String,
    installation_is_current: bool,
    ui_requirements: WorthUiQueryConsumerRequirements,
}

impl WorthUiPreparedSnapshotConsumer {
    pub fn execute(
        self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> WorthUiSnapshotConsumerExecutionOutcome {
        let (reference, bound, consumer, requirements) = self.into_parts();
        match bound.execute((), workspace) {
            TransitionOutcome::Success(executed) => {
                TransitionOutcome::Success(WorthUiExecutedSnapshotConsumer {
                    executed,
                    consumer,
                    reference,
                    requirements,
                })
            }
            TransitionOutcome::Deferred(deferred) => {
                TransitionOutcome::Deferred(WorthUiDeferredSnapshotConsumer {
                    deferred,
                    consumer,
                    reference,
                    requirements,
                })
            }
            TransitionOutcome::Denied(stop) => TransitionOutcome::Denied(stop),
            TransitionOutcome::Stale(stop) => TransitionOutcome::Stale(stop),
            TransitionOutcome::RebindRequired(stop) => TransitionOutcome::RebindRequired(stop),
            TransitionOutcome::Failed(stop) => TransitionOutcome::Failed(stop),
        }
    }
}

impl WorthUiDeferredSnapshotConsumer {
    pub fn binding_identity(&self) -> &str {
        self.deferred.binding_identity()
    }
    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.requirements
    }
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.reference
    }
    pub fn query_boundary_requirements(&self) -> domain::WorthQueryConsumerBoundaryRequirements {
        self.consumer.downstream_requirements()
    }
}

impl WorthUiExecutedSnapshotConsumer {
    pub fn publish(self) -> WorthUiSnapshotProjectionPublicationOutcome {
        let Self {
            executed,
            consumer,
            reference,
            requirements,
        } = self;
        executed
            .publish()
            .map_success(|published| WorthUiPublishedSnapshotConsumer {
                published,
                consumer,
                reference,
                requirements,
            })
    }
}

impl WorthUiPublishedSnapshotConsumer {
    pub fn consume(
        self,
        request: read::WorthQueryProjectionDeclaration,
    ) -> WorthUiSnapshotProjectionConsumptionOutcome {
        let Self {
            published,
            consumer,
            reference,
            requirements,
        } = self;
        published
            .consume(consumer.into_query_contract(), request)
            .map_success(|consumed| WorthUiConsumedSnapshotProjection {
                consumed,
                reference,
                requirements,
            })
    }
}

impl WorthUiConsumedSnapshotProjection {
    pub fn settle(self) -> WorthUiSnapshotProjectionSettlementOutcome {
        let Self {
            consumed,
            reference,
            requirements,
        } = self;
        consumed.settle().map_success(|settled| {
            let fact = WorthUiSettledSnapshotFact::from_settled(&settled);
            WorthUiSettledSnapshotProjection {
                settled,
                reference,
                requirements,
                fact: std::sync::Arc::new(fact),
            }
        })
    }
}

impl WorthUiSettledSnapshotProjection {
    pub(crate) fn attach_source_coordinates(
        &mut self,
        generation: super::WorthUiSettledSnapshotSourceGeneration,
        order: super::WorthUiSettledSnapshotSourceOrder,
    ) {
        std::sync::Arc::get_mut(&mut self.fact)
            .expect("source coordinates attach before settled fact sharing")
            .attach_source_coordinates(generation, order);
    }

    pub fn exact_query_projection(&self) -> &Settled {
        &self.settled
    }
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.reference
    }
    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.requirements
    }
    pub fn fact(&self) -> &WorthUiSettledSnapshotFact {
        &self.fact
    }
    pub(crate) fn shared_fact(&self) -> std::sync::Arc<WorthUiSettledSnapshotFact> {
        std::sync::Arc::clone(&self.fact)
    }
    pub fn execution_warnings(&self) -> &[domain::WorthQueryOperationExecutionWarning] {
        self.settled.warnings()
    }
    pub fn projection_warnings(
        &self,
    ) -> Option<&worth_query::facade::foundation::ProjectionConsumptionWarnings> {
        self.settled.projection_warnings()
    }
    pub fn result_state(&self) -> domain::WorthQueryOperationResultState {
        self.settled.result_state()
    }
    pub fn counters(&self) -> domain::WorthQueryOperationExecutionCounters {
        self.settled.counters()
    }
    pub fn publication_receipt(&self) -> &domain::WorthQueryDerivedPublicationReceipt {
        self.settled.publication_receipt()
    }
    pub fn conditional_provenance(&self) -> &[domain::WorthQueryConditionalProvenance] {
        self.settled.conditional_provenance()
    }
    pub fn exact_evidence(&self) -> WorthUiExactSettledSnapshotEvidence {
        WorthUiExactSettledSnapshotEvidence {
            installed_reference: self.reference.clone(),
            query_binding_identity: self.fact.query_binding_identity().to_owned(),
            settlement_identity: self.fact.settlement_identity().to_owned(),
            installation_is_current: self.reference.installation_is_current(),
            ui_requirements: self.requirements,
        }
    }
}

impl WorthUiExactSettledSnapshotEvidence {
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }
    pub fn query_binding_identity(&self) -> &str {
        &self.query_binding_identity
    }
    pub fn settlement_identity(&self) -> &str {
        &self.settlement_identity
    }
    pub fn installation_is_current(&self) -> bool {
        self.installation_is_current
    }
    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.ui_requirements
    }
}
