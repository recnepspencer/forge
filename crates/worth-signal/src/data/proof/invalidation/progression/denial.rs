use super::ResolvedInvalidationWork;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InvalidationProgressionDenial {
    StaleGraphInstance,
    StaleDependencyRevision,
    StaleOriginGeneration,
    StaleReadinessEpoch,
    StaleStageOrder,
    RebindRequired,
    DependencyPending,
    ContractRejected,
}

pub(crate) type InvalidationOriginAdmissionOutcome = worth_proof::TransitionOutcome<
    ResolvedInvalidationWork,
    InvalidationProgressionDenial,
    InvalidationProgressionDenial,
    InvalidationProgressionDenial,
    InvalidationProgressionDenial,
    crate::data::error::SignalError,
>;
