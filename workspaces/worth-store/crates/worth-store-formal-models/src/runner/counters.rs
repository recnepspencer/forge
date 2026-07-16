use super::{
    ExactProtocolRefinementCoverageReceipt, ProtocolCheckStatistics,
    ProtocolCounterEvidenceIdentity, ProtocolCounterexample,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolRunnerCounter {
    OwnerCasesDeclared,
    OwnerCasesExecuted,
    OwnerCasesMapped,
    TypedOutcomePosturesObserved,
    RuntimeObservationsRejected,
    OwnerCasesMissing,
    DuplicateMappings,
    NormalizationRejections,
    ReceiptEmission,
    OmissionClassification,
    MappingRejection,
    StateExploration,
    TransitionExploration,
    InvariantChecksExecuted,
    DeadlockDetection,
    BoundExhaustion,
    CounterexamplesProduced,
    CounterexampleLocalization,
    UnsupportedBackendMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolCounterSnapshot {
    identity: ProtocolCounterEvidenceIdentity,
    values: [u64; 19],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolConformanceCounterInput {
    owner_cases_declared: u64,
    owner_cases_executed: u64,
    owner_cases_mapped: u64,
    typed_outcome_postures_observed: u64,
    runtime_observations_rejected: u64,
    owner_cases_missing: u64,
    duplicate_mappings: u64,
    normalization_rejections: u64,
    receipt_emissions: u64,
    omission_classifications: u64,
    mapping_rejections: u64,
    unsupported_backend_mismatches: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCounterProjectionDenial {
    ProtocolMismatch,
    EmptyCounterexample,
    InitialStatesExceedGeneratedStates,
    NonExactOwnerCoverage,
}

impl ProtocolConformanceCounterInput {
    pub const fn from_exact_coverage(
        coverage: ExactProtocolRefinementCoverageReceipt,
        typed_outcome_postures_observed: u64,
        receipt_emissions: u64,
    ) -> Self {
        Self {
            owner_cases_declared: coverage.declared_owner_cases(),
            owner_cases_executed: coverage.ordinary_executed_cases(),
            owner_cases_mapped: coverage.mapped_model_actions(),
            typed_outcome_postures_observed,
            runtime_observations_rejected: 0,
            owner_cases_missing: 0,
            duplicate_mappings: 0,
            normalization_rejections: 0,
            receipt_emissions,
            omission_classifications: 0,
            mapping_rejections: 0,
            unsupported_backend_mismatches: 0,
        }
    }
}

pub fn project_checked_protocol_counters(
    identity: ProtocolCounterEvidenceIdentity,
    statistics: ProtocolCheckStatistics,
    conformance: ProtocolConformanceCounterInput,
    invariant_checks_executed: u64,
) -> Result<ProtocolCounterSnapshot, ProtocolCounterProjectionDenial> {
    require_exact_conformance(conformance)?;
    let mut values = conformance_values(conformance);
    set_model_exploration(&mut values, statistics)?;
    values[ProtocolRunnerCounter::InvariantChecksExecuted.index()] = invariant_checks_executed;
    Ok(ProtocolCounterSnapshot { identity, values })
}

pub fn project_counterexample_protocol_counters(
    identity: ProtocolCounterEvidenceIdentity,
    statistics: ProtocolCheckStatistics,
    invariant_checks_executed: u64,
    localized_counterexample: &ProtocolCounterexample,
) -> Result<ProtocolCounterSnapshot, ProtocolCounterProjectionDenial> {
    if identity.protocol() != localized_counterexample.protocol() {
        return Err(ProtocolCounterProjectionDenial::ProtocolMismatch);
    }
    if localized_counterexample.states().is_empty() {
        return Err(ProtocolCounterProjectionDenial::EmptyCounterexample);
    }
    let mut values = [0; 19];
    set_model_exploration(&mut values, statistics)?;
    values[ProtocolRunnerCounter::InvariantChecksExecuted.index()] = invariant_checks_executed;
    values[ProtocolRunnerCounter::CounterexamplesProduced.index()] = 1;
    values[ProtocolRunnerCounter::CounterexampleLocalization.index()] = 1;
    Ok(ProtocolCounterSnapshot { identity, values })
}

fn require_exact_conformance(
    input: ProtocolConformanceCounterInput,
) -> Result<(), ProtocolCounterProjectionDenial> {
    if input.owner_cases_declared != input.owner_cases_executed
        || input.owner_cases_declared != input.owner_cases_mapped
        || input.owner_cases_missing != 0
        || input.duplicate_mappings != 0
    {
        return Err(ProtocolCounterProjectionDenial::NonExactOwnerCoverage);
    }
    Ok(())
}

fn conformance_values(input: ProtocolConformanceCounterInput) -> [u64; 19] {
    let mut values = [0; 19];
    for (counter, value) in [
        (
            ProtocolRunnerCounter::OwnerCasesDeclared,
            input.owner_cases_declared,
        ),
        (
            ProtocolRunnerCounter::OwnerCasesExecuted,
            input.owner_cases_executed,
        ),
        (
            ProtocolRunnerCounter::OwnerCasesMapped,
            input.owner_cases_mapped,
        ),
        (
            ProtocolRunnerCounter::TypedOutcomePosturesObserved,
            input.typed_outcome_postures_observed,
        ),
        (
            ProtocolRunnerCounter::RuntimeObservationsRejected,
            input.runtime_observations_rejected,
        ),
        (
            ProtocolRunnerCounter::OwnerCasesMissing,
            input.owner_cases_missing,
        ),
        (
            ProtocolRunnerCounter::DuplicateMappings,
            input.duplicate_mappings,
        ),
        (
            ProtocolRunnerCounter::NormalizationRejections,
            input.normalization_rejections,
        ),
        (
            ProtocolRunnerCounter::ReceiptEmission,
            input.receipt_emissions,
        ),
        (
            ProtocolRunnerCounter::OmissionClassification,
            input.omission_classifications,
        ),
        (
            ProtocolRunnerCounter::MappingRejection,
            input.mapping_rejections,
        ),
        (
            ProtocolRunnerCounter::UnsupportedBackendMismatch,
            input.unsupported_backend_mismatches,
        ),
    ] {
        values[counter.index()] = value;
    }
    values
}

fn set_model_exploration(
    values: &mut [u64; 19],
    statistics: ProtocolCheckStatistics,
) -> Result<(), ProtocolCounterProjectionDenial> {
    let transitions = statistics
        .generated_states()
        .checked_sub(statistics.initial_states())
        .ok_or(ProtocolCounterProjectionDenial::InitialStatesExceedGeneratedStates)?;
    values[ProtocolRunnerCounter::StateExploration.index()] = statistics.distinct_states();
    values[ProtocolRunnerCounter::TransitionExploration.index()] = transitions;
    Ok(())
}

impl ProtocolCounterSnapshot {
    pub const fn identity(&self) -> &ProtocolCounterEvidenceIdentity {
        &self.identity
    }

    pub const fn get(&self, counter: ProtocolRunnerCounter) -> u64 {
        self.values[counter.index()]
    }
}

impl ProtocolRunnerCounter {
    pub const fn all() -> [Self; 19] {
        [
            Self::OwnerCasesDeclared,
            Self::OwnerCasesExecuted,
            Self::OwnerCasesMapped,
            Self::TypedOutcomePosturesObserved,
            Self::RuntimeObservationsRejected,
            Self::OwnerCasesMissing,
            Self::DuplicateMappings,
            Self::NormalizationRejections,
            Self::ReceiptEmission,
            Self::OmissionClassification,
            Self::MappingRejection,
            Self::StateExploration,
            Self::TransitionExploration,
            Self::InvariantChecksExecuted,
            Self::DeadlockDetection,
            Self::BoundExhaustion,
            Self::CounterexamplesProduced,
            Self::CounterexampleLocalization,
            Self::UnsupportedBackendMismatch,
        ]
    }

    const fn index(self) -> usize {
        self as usize
    }
}
