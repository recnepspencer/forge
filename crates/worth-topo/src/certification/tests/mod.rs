use crate::certification::support::reporting::ReplayParityStatus;
use crate::facade::{
    milestone_one_closeout_requirements, milestone_one_closeout_suite_definition,
    milestone_two_closeout_requirements, milestone_two_closeout_suite_definition,
    AdmittedRangeSweepReport, BridgeProofReport, CertificationCanonicalRow, CertificationParityRow,
    CertificationRejectionRow, CertificationRequiredOutput, CertificationSuiteDefinition,
    DerivedEquivalenceContractAggregateReport, DerivedFallbackAggregateReport,
    DerivedInvalidationAggregateReport, DerivedRebuildAggregateReport,
    DerivedValidatorCoverageReport, DeterministicDigest, FailureLocalityReport,
    MilestoneOneCloseoutReport, MilestoneOneCounters, MilestoneTwoCloseoutReport,
    MilestoneTwoCounters, PrimitiveCorpusCoverageMatrix, PrimitiveCorpusParityReport,
};
#[cfg(feature = "slow-certification")]
use crate::facade::{
    milestone_three_closeout_requirements, milestone_three_closeout_suite_definition,
    MilestoneThreeDeterminismRuleKind, MilestoneThreeMutationFalloutClass,
};
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::platform::authority::{ShellInterpretationClass, WireInterpretationClass};
use schema::facade::topology_authoring::{
    MilestoneOnePrimitiveCase, MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
};

use crate::certification::facade::{
    certify_milestone_two_topology_commit_input_traced, certify_topology_commit_input_traced,
};
use crate::certification::{
    certify_milestone_one_branch_local_primitive_scenarios, certify_milestone_one_closeout,
    certify_milestone_one_default_primitive_corpus, certify_milestone_one_primitive_corpus,
    certify_milestone_one_read_basis_traced, certify_milestone_two_closeout,
    certify_milestone_two_default_derived_corpus, certify_milestone_two_read_basis_traced,
};
use crate::test_support::primitive_corpus::authored_topology::milestone_one_default_corpus_scenarios;
use crate::test_support::primitive_corpus::branch_replay_cases::milestone_one_default_branch_local_admitted_scenarios;
use crate::test_support::primitive_corpus::validated_topology::{
    committed_primitive_input, seeded_bootstrap,
};

mod closeout;
mod parameter_sweeps;
mod primitive_corpus;
mod public_facade_contracts;
mod query_runtime;
mod read_surfaces;
mod support;
#[cfg(feature = "slow-certification")]
mod topology_operator_closeout;
mod verified_commit;
