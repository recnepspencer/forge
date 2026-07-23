use worth_query::facade::{
    foundation::ObservationLaneWitness,
    installed::{self, conditional, operation},
    read, runtime,
};

use super::{
    WorthUiPreparedSnapshotConsumer, WorthUiQueryConsumerRequirements, WorthUiSettledSnapshotFact,
    WorthUiSnapshotConsumerExecutionOutcome, WorthUiSnapshotProjectionConsumptionOutcome,
    WorthUiSnapshotProjectionPublicationOutcome, WorthUiSnapshotProjectionSettlementOutcome,
};

type Executed = operation::WorthQueryExecutedDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
    read::WorthQueryReadCompletion,
>;
type Published = operation::WorthQueryPublishedDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type Consumed = operation::WorthQueryConsumedDomainProjection<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type Settled = operation::WorthQuerySettledDomainProjection<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type ConsumerBoundary = operation::WorthQueryConsumerBoundary<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;
type Deferred = operation::WorthQueryDeferredDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
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
        match installed::transition::execution(bound.execute((), workspace)) {
            installed::transition::WorthQueryExecutionTransition::Executed(executed) => {
                WorthUiSnapshotConsumerExecutionOutcome::Executed(WorthUiExecutedSnapshotConsumer {
                    executed,
                    consumer,
                    reference,
                    requirements,
                })
            }
            installed::transition::WorthQueryExecutionTransition::Deferred(deferred) => {
                WorthUiSnapshotConsumerExecutionOutcome::Deferred(WorthUiDeferredSnapshotConsumer {
                    deferred,
                    consumer,
                    reference,
                    requirements,
                })
            }
            installed::transition::WorthQueryExecutionTransition::Denied(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::Denied(stop)
            }
            installed::transition::WorthQueryExecutionTransition::Stale(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::Stale(stop)
            }
            installed::transition::WorthQueryExecutionTransition::RebindRequired(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::RebindRequired(stop)
            }
            installed::transition::WorthQueryExecutionTransition::Failed(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::Failed(stop)
            }
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
    pub fn query_boundary_requirements(&self) -> operation::WorthQueryConsumerBoundaryRequirements {
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
        match installed::transition::publication(executed.publish()) {
            installed::transition::WorthQueryPublicationTransition::Published(published) => {
                WorthUiSnapshotProjectionPublicationOutcome::Published(
                    WorthUiPublishedSnapshotConsumer {
                        published,
                        consumer,
                        reference,
                        requirements,
                    },
                )
            }
            installed::transition::WorthQueryPublicationTransition::Denied(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::Denied(stop)
            }
            installed::transition::WorthQueryPublicationTransition::Stale(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::Stale(stop)
            }
            installed::transition::WorthQueryPublicationTransition::RebindRequired(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::RebindRequired(stop)
            }
            installed::transition::WorthQueryPublicationTransition::Failed(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::Failed(stop)
            }
        }
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
        map_consumption(
            installed::transition::consumption(
                published.consume(consumer.into_query_contract(), request),
            ),
            reference,
            requirements,
        )
    }

    pub fn consume_bound(
        self,
        request: operation::WorthQueryBoundProjectionRequest<
            crate::WorthUiDomainEntry,
            crate::WorthUiSnapshotMeasurement,
            crate::WorthUiSnapshotMeasurementFamily,
            worth_query::facade::foundation::ObservationLaneWitness,
        >,
    ) -> WorthUiSnapshotProjectionConsumptionOutcome {
        let Self {
            published,
            consumer: _,
            reference,
            requirements,
        } = self;
        map_consumption(
            installed::transition::consumption(published.consume_bound(request)),
            reference,
            requirements,
        )
    }
}

impl WorthUiConsumedSnapshotProjection {
    pub fn settle(self) -> WorthUiSnapshotProjectionSettlementOutcome {
        let Self {
            consumed,
            reference,
            requirements,
        } = self;
        match installed::transition::settlement(consumed.settle()) {
            installed::transition::WorthQuerySettlementTransition::Settled(settled) => {
                let fact = WorthUiSettledSnapshotFact::from_settled(&settled);
                WorthUiSnapshotProjectionSettlementOutcome::Settled(
                    WorthUiSettledSnapshotProjection {
                        settled,
                        reference,
                        requirements,
                        fact: std::sync::Arc::new(fact),
                    },
                )
            }
            installed::transition::WorthQuerySettlementTransition::Denied(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::Denied(stop)
            }
            installed::transition::WorthQuerySettlementTransition::Stale(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::Stale(stop)
            }
            installed::transition::WorthQuerySettlementTransition::RebindRequired(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::RebindRequired(stop)
            }
            installed::transition::WorthQuerySettlementTransition::Failed(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::Failed(stop)
            }
        }
    }
}

fn map_consumption(
    outcome: installed::transition::WorthQueryConsumptionTransition<
        crate::WorthUiDomainEntry,
        crate::WorthUiSnapshotMeasurement,
        crate::WorthUiSnapshotMeasurementFamily,
        ObservationLaneWitness,
    >,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
) -> WorthUiSnapshotProjectionConsumptionOutcome {
    match outcome {
        installed::transition::WorthQueryConsumptionTransition::Consumed(consumed) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Consumed(
                WorthUiConsumedSnapshotProjection {
                    consumed,
                    reference,
                    requirements,
                },
            )
        }
        installed::transition::WorthQueryConsumptionTransition::Denied(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Denied(stop)
        }
        installed::transition::WorthQueryConsumptionTransition::Deferred(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Deferred(stop)
        }
        installed::transition::WorthQueryConsumptionTransition::Stale(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Stale(stop)
        }
        installed::transition::WorthQueryConsumptionTransition::RebindRequired(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::RebindRequired(stop)
        }
        installed::transition::WorthQueryConsumptionTransition::Failed(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Failed(stop)
        }
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
    pub fn execution_warnings(&self) -> &[operation::WorthQueryOperationExecutionWarning] {
        self.settled.warnings()
    }
    pub fn projection_warnings(
        &self,
    ) -> Option<&worth_query::facade::foundation::ProjectionConsumptionWarnings> {
        self.settled.projection_warnings()
    }
    pub fn result_state(&self) -> operation::WorthQueryOperationResultState {
        self.settled.result_state()
    }
    pub fn counters(&self) -> operation::WorthQueryOperationExecutionCounters {
        self.settled.counters()
    }
    pub fn publication_receipt(&self) -> &operation::WorthQueryDerivedPublicationReceipt {
        self.settled.publication_receipt()
    }
    pub fn conditional_provenance(&self) -> &[conditional::WorthQueryConditionalProvenance] {
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
