use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCollection, WorthQueryBoundCollectionWindow, WorthQueryBoundExecutionDenial,
    WorthQueryBoundExecutionOutcome, WorthQueryConsumedDomainProjection,
    WorthQueryDeferredDomainOperation, WorthQueryExecutableDomainOperation,
    WorthQueryExecutedDomainOperation, WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryProgressionDenial, WorthQueryPublicationDenial, WorthQueryPublishedDomainOperation,
    WorthQuerySettledDomainProjection,
};
use worth_proof::TransitionOutcome;

pub enum WorthQueryResourceAdmissionStop {
    Denied(WorthQueryExecutionResourceAdmissionDenial),
    Deferred(WorthQueryExecutionResourceAdmissionDenial),
    Stale(WorthQueryExecutionResourceAdmissionDenial),
    RebindRequired(WorthQueryExecutionResourceAdmissionDenial),
    Failed(WorthQueryExecutionResourceAdmissionDenial),
}

pub enum WorthQueryResourceAdmissionTransition<T> {
    Admitted(T),
    Denied(WorthQueryExecutionResourceAdmissionDenial),
    Deferred(WorthQueryExecutionResourceAdmissionDenial),
    Stale(WorthQueryExecutionResourceAdmissionDenial),
    RebindRequired(WorthQueryExecutionResourceAdmissionDenial),
    Failed(WorthQueryExecutionResourceAdmissionDenial),
}

impl<T> WorthQueryResourceAdmissionTransition<T> {
    pub fn into_result(self) -> Result<T, WorthQueryResourceAdmissionStop> {
        match self {
            Self::Admitted(value) => Ok(value),
            Self::Denied(stop) => Err(WorthQueryResourceAdmissionStop::Denied(stop)),
            Self::Deferred(stop) => Err(WorthQueryResourceAdmissionStop::Deferred(stop)),
            Self::Stale(stop) => Err(WorthQueryResourceAdmissionStop::Stale(stop)),
            Self::RebindRequired(stop) => {
                Err(WorthQueryResourceAdmissionStop::RebindRequired(stop))
            }
            Self::Failed(stop) => Err(WorthQueryResourceAdmissionStop::Failed(stop)),
        }
    }
}

pub fn resource_admission<T>(
    outcome: TransitionOutcome<
        T,
        WorthQueryExecutionResourceAdmissionDenial,
        WorthQueryExecutionResourceAdmissionDenial,
        WorthQueryExecutionResourceAdmissionDenial,
        WorthQueryExecutionResourceAdmissionDenial,
        WorthQueryExecutionResourceAdmissionDenial,
    >,
) -> WorthQueryResourceAdmissionTransition<T> {
    match outcome {
        TransitionOutcome::Success(value) => WorthQueryResourceAdmissionTransition::Admitted(value),
        TransitionOutcome::Denied(value) => WorthQueryResourceAdmissionTransition::Denied(value),
        TransitionOutcome::Deferred(value) => {
            WorthQueryResourceAdmissionTransition::Deferred(value)
        }
        TransitionOutcome::Stale(value) => WorthQueryResourceAdmissionTransition::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            WorthQueryResourceAdmissionTransition::RebindRequired(value)
        }
        TransitionOutcome::Failed(value) => WorthQueryResourceAdmissionTransition::Failed(value),
    }
}

pub enum WorthQueryExecutionTransition<D, O, F, L: BasisOperationLane, Output>
where
    O: WorthQueryExecutableDomainOperation<D, F>,
{
    Executed(WorthQueryExecutedDomainOperation<D, O, F, L, Output>),
    Deferred(WorthQueryDeferredDomainOperation<D, O, F, L>),
    Denied(WorthQueryBoundExecutionDenial),
    Stale(WorthQueryBoundExecutionDenial),
    RebindRequired(WorthQueryBoundExecutionDenial),
    Failed(WorthQueryBoundExecutionDenial),
}

pub fn execution<D, O, F, L: BasisOperationLane, Output>(
    outcome: WorthQueryBoundExecutionOutcome<D, O, F, L, Output>,
) -> WorthQueryExecutionTransition<D, O, F, L, Output>
where
    O: WorthQueryExecutableDomainOperation<D, F>,
{
    match outcome {
        TransitionOutcome::Success(value) => WorthQueryExecutionTransition::Executed(value),
        TransitionOutcome::Deferred(value) => WorthQueryExecutionTransition::Deferred(value),
        TransitionOutcome::Denied(value) => WorthQueryExecutionTransition::Denied(value),
        TransitionOutcome::Stale(value) => WorthQueryExecutionTransition::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            WorthQueryExecutionTransition::RebindRequired(value)
        }
        TransitionOutcome::Failed(value) => WorthQueryExecutionTransition::Failed(value),
    }
}

pub enum WorthQueryPublicationTransition<D, O, F, L: BasisOperationLane> {
    Published(WorthQueryPublishedDomainOperation<D, O, F, L>),
    Denied(WorthQueryPublicationDenial),
    Stale(WorthQueryPublicationDenial),
    RebindRequired(WorthQueryPublicationDenial),
    Failed(WorthQueryPublicationDenial),
}

pub fn publication<D, O, F, L: BasisOperationLane>(
    outcome: TransitionOutcome<
        WorthQueryPublishedDomainOperation<D, O, F, L>,
        WorthQueryPublicationDenial,
        std::convert::Infallible,
        WorthQueryPublicationDenial,
        WorthQueryPublicationDenial,
        WorthQueryPublicationDenial,
    >,
) -> WorthQueryPublicationTransition<D, O, F, L> {
    match outcome {
        TransitionOutcome::Success(value) => WorthQueryPublicationTransition::Published(value),
        TransitionOutcome::Deferred(never) => match never {},
        TransitionOutcome::Denied(value) => WorthQueryPublicationTransition::Denied(value),
        TransitionOutcome::Stale(value) => WorthQueryPublicationTransition::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            WorthQueryPublicationTransition::RebindRequired(value)
        }
        TransitionOutcome::Failed(value) => WorthQueryPublicationTransition::Failed(value),
    }
}

pub enum WorthQueryConsumptionTransition<D, O, F, L: BasisOperationLane> {
    Consumed(WorthQueryConsumedDomainProjection<D, O, F, L>),
    Denied(WorthQueryProgressionDenial),
    Deferred(WorthQueryProgressionDenial),
    Stale(WorthQueryProgressionDenial),
    RebindRequired(WorthQueryProgressionDenial),
    Failed(WorthQueryProgressionDenial),
}

pub fn consumption<D, O, F, L: BasisOperationLane>(
    outcome: TransitionOutcome<
        WorthQueryConsumedDomainProjection<D, O, F, L>,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
    >,
) -> WorthQueryConsumptionTransition<D, O, F, L> {
    match outcome {
        TransitionOutcome::Success(value) => WorthQueryConsumptionTransition::Consumed(value),
        TransitionOutcome::Denied(value) => WorthQueryConsumptionTransition::Denied(value),
        TransitionOutcome::Deferred(value) => WorthQueryConsumptionTransition::Deferred(value),
        TransitionOutcome::Stale(value) => WorthQueryConsumptionTransition::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            WorthQueryConsumptionTransition::RebindRequired(value)
        }
        TransitionOutcome::Failed(value) => WorthQueryConsumptionTransition::Failed(value),
    }
}

pub enum WorthQuerySettlementTransition<D, O, F, L: BasisOperationLane> {
    Settled(WorthQuerySettledDomainProjection<D, O, F, L>),
    Denied(WorthQueryProgressionDenial),
    Stale(WorthQueryProgressionDenial),
    RebindRequired(WorthQueryProgressionDenial),
    Failed(WorthQueryProgressionDenial),
}

pub fn settlement<D, O, F, L: BasisOperationLane>(
    outcome: TransitionOutcome<
        WorthQuerySettledDomainProjection<D, O, F, L>,
        WorthQueryProgressionDenial,
        std::convert::Infallible,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
    >,
) -> WorthQuerySettlementTransition<D, O, F, L> {
    match outcome {
        TransitionOutcome::Success(value) => WorthQuerySettlementTransition::Settled(value),
        TransitionOutcome::Deferred(never) => match never {},
        TransitionOutcome::Denied(value) => WorthQuerySettlementTransition::Denied(value),
        TransitionOutcome::Stale(value) => WorthQuerySettlementTransition::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            WorthQuerySettlementTransition::RebindRequired(value)
        }
        TransitionOutcome::Failed(value) => WorthQuerySettlementTransition::Failed(value),
    }
}

pub enum WorthQueryCollectionCapabilityTransition<D, O, F, L: BasisOperationLane> {
    Bound(WorthQueryBoundCollection<D, O, F, L>),
    Denied(crate::domain_installation::WorthQueryCollectionCapabilityStop<D, O, F, L>),
    Stale(crate::domain_installation::WorthQueryCollectionCapabilityStop<D, O, F, L>),
}

pub fn collection_capability<D, O, F, L: BasisOperationLane>(
    outcome: crate::domain_installation::WorthQueryCollectionCapabilityOutcome<D, O, F, L>,
) -> WorthQueryCollectionCapabilityTransition<D, O, F, L> {
    match outcome {
        TransitionOutcome::Success(value) => WorthQueryCollectionCapabilityTransition::Bound(value),
        TransitionOutcome::Denied(value) => WorthQueryCollectionCapabilityTransition::Denied(value),
        TransitionOutcome::Stale(value) => WorthQueryCollectionCapabilityTransition::Stale(value),
        TransitionOutcome::Deferred(never)
        | TransitionOutcome::RebindRequired(never)
        | TransitionOutcome::Failed(never) => match never {},
    }
}

pub enum WorthQueryCollectionWindowTransition<Value> {
    Admitted(Value),
    Denied(crate::domain_installation::WorthQueryCollectionWindowDenial),
    Stale(crate::domain_installation::WorthQueryCollectionWindowDenial),
    RebindRequired(crate::domain_installation::WorthQueryCollectionWindowDenial),
}

pub fn collection_window_admission(
    outcome: crate::domain_installation::WorthQueryCollectionWindowAdmissionOutcome,
) -> WorthQueryCollectionWindowTransition<
    crate::domain_installation::WorthQueryAdmittedCollectionWindow,
> {
    collection_window(outcome)
}

pub fn collection_window_resolution(
    outcome: crate::domain_installation::WorthQueryCollectionWindowOutcome,
) -> WorthQueryCollectionWindowTransition<WorthQueryBoundCollectionWindow> {
    collection_window(outcome)
}

fn collection_window<Value>(
    outcome: TransitionOutcome<
        Value,
        crate::domain_installation::WorthQueryCollectionWindowDenial,
        std::convert::Infallible,
        crate::domain_installation::WorthQueryCollectionWindowDenial,
        crate::domain_installation::WorthQueryCollectionWindowDenial,
    >,
) -> WorthQueryCollectionWindowTransition<Value> {
    match outcome {
        TransitionOutcome::Success(value) => WorthQueryCollectionWindowTransition::Admitted(value),
        TransitionOutcome::Denied(value) => WorthQueryCollectionWindowTransition::Denied(value),
        TransitionOutcome::Stale(value) => WorthQueryCollectionWindowTransition::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            WorthQueryCollectionWindowTransition::RebindRequired(value)
        }
        TransitionOutcome::Deferred(never) | TransitionOutcome::Failed(never) => match never {},
    }
}
