mod catalog;
mod execution;
mod localization;
mod mapped_guard;
mod physical_replay;
mod scheduled_shrink;

pub use catalog::ControlledProtocolMutant;
pub use execution::{
    run_controlled_mutant_program, ControlledMutantRejection, MutationProgramFailure,
    MutationProgramReport,
};
pub use localization::{
    ControlledMutantLocalization, ControlledMutantLocalizationDenial,
    SharedControlledMutantLocalization,
};
pub use physical_replay::{
    ConcreteCounterexampleGuard, CounterexampleOwnerIdentity, CounterexamplePhysicalReplayDenial,
    CounterexamplePhysicalReplayEvidence, CounterexampleReplayEvidenceIdentity,
};

#[cfg(test)]
pub(in crate::courtroom::protocol_models) use execution::structural_mutation_fixture_for_closeout_tests;
