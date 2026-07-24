use worth_query::facade::{
    foundation::ObservationLaneWitness,
    installed::{self, conditional, operation},
    read, runtime,
};

use super::{
    WorthUiAdmittedQueryBindingReference, WorthUiAdmittedQuerySettlementReference,
    WorthUiPreparedSnapshotConsumer, WorthUiQueryConsumerRequirements, WorthUiSettledSnapshotFact,
    WorthUiSnapshotConsumerExecutionOutcome, WorthUiSnapshotNativeAccess,
    WorthUiSnapshotNativeRequest, WorthUiSnapshotProjectionConsumptionOutcome,
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
type Deferred = operation::WorthQueryDeferredDomainOperation<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;

pub struct WorthUiDeferredSnapshotConsumer {
    deferred: Deferred,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

impl std::fmt::Debug for WorthUiDeferredSnapshotConsumer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiDeferredSnapshotConsumer")
            .field("requirements", &self.requirements)
            .field("query_artifact", &"sealed")
            .finish_non_exhaustive()
    }
}

pub struct WorthUiExecutedSnapshotConsumer {
    executed: Executed,
    native_request: WorthUiSnapshotNativeRequest,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

pub struct WorthUiPublishedSnapshotConsumer {
    published: Published,
    native_request: WorthUiSnapshotNativeRequest,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
}

pub struct WorthUiConsumedSnapshotProjection {
    consumed: Consumed,
    native_access: WorthUiSnapshotNativeAccess,
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
    binding_reference: WorthUiAdmittedQueryBindingReference,
    settlement_reference: WorthUiAdmittedQuerySettlementReference,
    installation_is_current: bool,
    ui_requirements: WorthUiQueryConsumerRequirements,
}

impl WorthUiPreparedSnapshotConsumer {
    pub fn execute(
        self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> WorthUiSnapshotConsumerExecutionOutcome {
        let (reference, bound, native_request, requirements) = self.into_parts();
        match installed::transition::execution(bound.execute((), workspace)) {
            installed::transition::WorthQueryExecutionTransition::Executed(executed) => {
                WorthUiSnapshotConsumerExecutionOutcome::Executed(Box::new(
                    WorthUiExecutedSnapshotConsumer {
                        executed,
                        native_request,
                        reference,
                        requirements,
                    },
                ))
            }
            installed::transition::WorthQueryExecutionTransition::Deferred(deferred) => {
                WorthUiSnapshotConsumerExecutionOutcome::Deferred(Box::new(
                    WorthUiDeferredSnapshotConsumer {
                        deferred,
                        reference,
                        requirements,
                    },
                ))
            }
            installed::transition::WorthQueryExecutionTransition::Denied(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::Denied(Box::new(stop))
            }
            installed::transition::WorthQueryExecutionTransition::Stale(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::Stale(Box::new(stop))
            }
            installed::transition::WorthQueryExecutionTransition::RebindRequired(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::RebindRequired(Box::new(stop))
            }
            installed::transition::WorthQueryExecutionTransition::Failed(stop) => {
                WorthUiSnapshotConsumerExecutionOutcome::Failed(Box::new(stop))
            }
        }
    }
}

impl WorthUiDeferredSnapshotConsumer {
    pub fn exact_query_deferred(&self) -> &Deferred {
        &self.deferred
    }

    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.requirements
    }
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.reference
    }
    pub fn query_boundary_requirements(&self) -> operation::WorthQueryConsumerBoundaryRequirements {
        self.requirements.query_boundary()
    }
}

impl WorthUiExecutedSnapshotConsumer {
    pub fn publish(self) -> WorthUiSnapshotProjectionPublicationOutcome {
        let Self {
            executed,
            native_request,
            reference,
            requirements,
        } = self;
        match installed::transition::publication(executed.publish()) {
            installed::transition::WorthQueryPublicationTransition::Published(published) => {
                WorthUiSnapshotProjectionPublicationOutcome::Published(Box::new(
                    WorthUiPublishedSnapshotConsumer {
                        published,
                        native_request,
                        reference,
                        requirements,
                    },
                ))
            }
            installed::transition::WorthQueryPublicationTransition::Denied(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::Denied(Box::new(stop))
            }
            installed::transition::WorthQueryPublicationTransition::Stale(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::Stale(Box::new(stop))
            }
            installed::transition::WorthQueryPublicationTransition::RebindRequired(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::RebindRequired(Box::new(stop))
            }
            installed::transition::WorthQueryPublicationTransition::Failed(stop) => {
                WorthUiSnapshotProjectionPublicationOutcome::Failed(Box::new(stop))
            }
        }
    }
}

impl WorthUiPublishedSnapshotConsumer {
    pub fn consume(self) -> WorthUiSnapshotProjectionConsumptionOutcome {
        let Self {
            published,
            native_request,
            reference,
            requirements,
        } = self;
        let (request, native_access) = native_request.into_parts();
        map_consumption(
            installed::transition::consumption(published.consume_bound(request)),
            native_access,
            reference,
            requirements,
        )
    }
}

pub struct WorthUiSettledSnapshotDerivationStop {
    _settled: Settled,
    _native_access: WorthUiSnapshotNativeAccess,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
    error: super::WorthUiQueryMeasurementFactObservationError,
}

impl WorthUiConsumedSnapshotProjection {
    pub fn settle(self) -> WorthUiSnapshotProjectionSettlementOutcome {
        let Self {
            consumed,
            native_access,
            reference,
            requirements,
        } = self;
        match installed::transition::settlement(consumed.settle()) {
            installed::transition::WorthQuerySettlementTransition::Settled(settled) => {
                let binding_reference = WorthUiAdmittedQueryBindingReference::admit(&reference);
                let settlement_reference = WorthUiAdmittedQuerySettlementReference::mint();
                let fact = WorthUiSettledSnapshotFact::from_settled(
                    &settled,
                    &native_access,
                    binding_reference,
                    settlement_reference,
                );
                match fact {
                    Ok(fact) => WorthUiSnapshotProjectionSettlementOutcome::Settled(Box::new(
                        WorthUiSettledSnapshotProjection {
                            settled,
                            reference,
                            requirements,
                            fact: std::sync::Arc::new(fact),
                        },
                    )),
                    Err(error) => WorthUiSnapshotProjectionSettlementOutcome::DerivationStopped(
                        Box::new(WorthUiSettledSnapshotDerivationStop {
                            _settled: settled,
                            _native_access: native_access,
                            reference,
                            requirements,
                            error,
                        }),
                    ),
                }
            }
            installed::transition::WorthQuerySettlementTransition::Denied(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::Denied(Box::new(stop))
            }
            installed::transition::WorthQuerySettlementTransition::Stale(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::Stale(Box::new(stop))
            }
            installed::transition::WorthQuerySettlementTransition::RebindRequired(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::RebindRequired(Box::new(stop))
            }
            installed::transition::WorthQuerySettlementTransition::Failed(stop) => {
                WorthUiSnapshotProjectionSettlementOutcome::Failed(Box::new(stop))
            }
        }
    }
}

impl WorthUiSettledSnapshotDerivationStop {
    pub fn error(&self) -> &super::WorthUiQueryMeasurementFactObservationError {
        &self.error
    }

    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.reference
    }

    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.requirements
    }
}

fn map_consumption(
    outcome: installed::transition::WorthQueryConsumptionTransition<
        crate::WorthUiDomainEntry,
        crate::WorthUiSnapshotMeasurement,
        crate::WorthUiSnapshotMeasurementFamily,
        ObservationLaneWitness,
    >,
    native_access: WorthUiSnapshotNativeAccess,
    reference: crate::WorthUiInstalledQueryBindingReference,
    requirements: WorthUiQueryConsumerRequirements,
) -> WorthUiSnapshotProjectionConsumptionOutcome {
    match outcome {
        installed::transition::WorthQueryConsumptionTransition::Consumed(consumed) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Consumed(Box::new(
                WorthUiConsumedSnapshotProjection {
                    consumed,
                    native_access,
                    reference,
                    requirements,
                },
            ))
        }
        installed::transition::WorthQueryConsumptionTransition::Denied(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Denied(Box::new(stop))
        }
        installed::transition::WorthQueryConsumptionTransition::Deferred(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Deferred(Box::new(stop))
        }
        installed::transition::WorthQueryConsumptionTransition::Stale(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Stale(Box::new(stop))
        }
        installed::transition::WorthQueryConsumptionTransition::RebindRequired(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::RebindRequired(Box::new(stop))
        }
        installed::transition::WorthQueryConsumptionTransition::Failed(stop) => {
            WorthUiSnapshotProjectionConsumptionOutcome::Failed(Box::new(stop))
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
            binding_reference: self.fact.binding_reference().clone(),
            settlement_reference: self.fact.settlement_reference().clone(),
            installation_is_current: self.reference.installation_is_current(),
            ui_requirements: self.requirements,
        }
    }
}

impl WorthUiExactSettledSnapshotEvidence {
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }
    pub fn binding_reference(&self) -> &WorthUiAdmittedQueryBindingReference {
        &self.binding_reference
    }
    pub fn settlement_reference(&self) -> &WorthUiAdmittedQuerySettlementReference {
        &self.settlement_reference
    }
    pub fn installation_is_current(&self) -> bool {
        self.installation_is_current
    }
    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.ui_requirements
    }
}
