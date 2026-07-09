use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::diagnostics::failures::PreparationFailureClass;
use crate::authority::commit::preparation::diagnostics::observations::ValidationDiagnosticObservation;
use crate::authority::commit::preparation::reduction::identity::ValidationResultIdentity;
use crate::authority::commit::preparation::reduction::keys::ValidationReductionKey;
use crate::validation::data::InvariantCheckResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InvariantWorkerResult {
    pub(crate) result_identity: ValidationResultIdentity,
    pub(crate) result: InvariantCheckResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InvariantWorkerEnvelope {
    pub(crate) packet_index: usize,
    pub(crate) reduction_key: ValidationReductionKey,
    pub(crate) results: Vec<InvariantWorkerResult>,
    pub(crate) diagnostic_observations: Vec<ValidationDiagnosticObservation>,
    pub(crate) preparation_failures: Vec<PreparationFailureClass>,
    pub(crate) counters: ValidationPreparationCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidationReducerConflict {
    pub(crate) identity: ValidationResultIdentity,
}
