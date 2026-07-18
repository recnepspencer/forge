mod artifact_reference;
mod bundle;
mod c2_readiness;
mod command_contract;
mod historical_policy;
mod iteration_case;
mod iteration_envelope;
mod mutation_execution;
mod mutation_kind;
mod mutation_report;
mod preservation_checked;
mod quarantine;
mod subject_map;

pub use artifact_reference::CloseoutArtifactReference;
pub use bundle::{
    CloseoutPredicate, CloseoutPredicateEvidence, TestArchitectureCloseoutBundle,
    TestArchitectureCloseoutInputs,
};
pub use c2_readiness::C2TestArchitectureReadiness;
pub use command_contract::StableProofCommand;
pub use historical_policy::{
    HistoricalEvidenceDecision, HistoricalEvidenceDisposition, HistoricalEvidencePolicy,
};
pub use iteration_case::{
    DeveloperEditCase, DeveloperIterationCaseEvidence, IterationRunObservation, SourceEditReceipt,
};
pub use iteration_envelope::{DeveloperIterationEnvelope, ReferenceDevelopmentProfile};
pub use mutation_kind::ControlledDefectKind;
pub use mutation_report::{
    ControlledDefectObservation, InterpretableProductPosture, InterpretableProofProduct,
    MutationExecutionEvidence, ProofMutationSensitivityReport,
};
pub use preservation_checked::{PreservationAuthorityDigests, PreservationCheckedProofRun};
pub use quarantine::C2QuarantinedClaim;
pub use subject_map::ProductionSubject;

pub(crate) use mutation_execution::execute_mutation_matrix;

#[cfg(test)]
mod tests;
