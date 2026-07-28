use worth_query::facade::{
    foundation::ObservationLaneWitness,
    installed::{self, operation},
    read, runtime,
};

use super::{
    WorthUiAdmittedQueryBindingReference, WorthUiAdmittedQuerySettlementReference,
    WorthUiPreparedSnapshotConsumer, WorthUiQueryConsumerRequirements, WorthUiSettledSnapshotFact,
    WorthUiSnapshotConsumerExecutionOutcome, WorthUiSnapshotNativeAccess,
    WorthUiSnapshotNativeRequest, WorthUiSnapshotProjectionConsumptionOutcome,
    WorthUiSnapshotProjectionPublicationOutcome, WorthUiSnapshotProjectionSettlementOutcome,
};

mod settled_evidence;

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
        let admitted = match installed::transition::resource_admission(
            bound.admit_execution_resources(
                (),
                crate::installed_domain::execution_resources::operation_execution_resource_request(
                ),
                workspace,
            ),
        )
        .into_result()
        {
            Ok(admitted) => admitted,
            Err(stop) => {
                return WorthUiSnapshotConsumerExecutionOutcome::ResourceAdmission(Box::new(stop));
            }
        };
        match installed::transition::execution(admitted.execute(workspace)) {
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
