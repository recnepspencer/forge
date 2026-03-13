use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::diagnostics::observations::ValidationDiagnosticObservation;
use crate::authority::commit::preparation::reduction::identity::ValidationResultIdentity;
use crate::authority::commit::preparation::reduction::keys::ValidationReductionKey;
use crate::validation::data::InvariantCheckResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InvariantWorkerEnvelope {
    pub(crate) packet_index: usize,
    pub(crate) reduction_key: ValidationReductionKey,
    pub(crate) result_identity: ValidationResultIdentity,
    pub(crate) result: InvariantCheckResult,
    pub(crate) diagnostic_observations: Vec<ValidationDiagnosticObservation>,
    pub(crate) counters: ValidationPreparationCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidationReducerConflict {
    pub(crate) identity: ValidationResultIdentity,
}
