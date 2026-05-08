use crate::certification::report::ReplayParityStatus;
use crate::facade::{
    milestone_one_closeout_requirements, milestone_one_closeout_suite_definition,
    milestone_three_closeout_requirements, milestone_three_closeout_suite_definition,
    milestone_two_closeout_requirements, milestone_two_closeout_suite_definition,
    AdmittedRangeSweepReport, BridgeProofReport, CertificationCanonicalRow, CertificationParityRow,
    CertificationRejectionRow, CertificationRequiredOutput, CertificationSuiteDefinition,
    DerivedEquivalenceContractAggregateReport, DerivedFallbackAggregateReport,
    DerivedInvalidationAggregateReport, DerivedRebuildAggregateReport,
    DerivedValidatorCoverageReport, DeterministicDigest, FailureLocalityReport,
    MilestoneOneCloseoutReport, MilestoneOneCounters, MilestoneThreeHostileOutcomeClass,
    MilestoneTwoCloseoutReport, MilestoneTwoCounters, PrimitiveCorpusCoverageMatrix,
    PrimitiveCorpusParityReport,
};
use forge_relational::facade::history::BranchId;
use schema::facade::topology_authoring::{
    MilestoneOnePrimitiveCase, MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
};
use schema::facade::{MutationOrigin, RawTopologyIntent, TopologyMutation};
use schema::facade::{ShellInterpretationClass, WireInterpretationClass};

use crate::certification::{
    certify_milestone_one_branch_local_primitive_scenarios, certify_milestone_one_closeout,
    certify_milestone_one_default_primitive_corpus, certify_milestone_one_primitive_corpus,
    certify_milestone_one_read_basis_traced, certify_milestone_three_closeout,
    certify_milestone_two_closeout, certify_milestone_two_default_derived_corpus,
    certify_milestone_two_read_basis_traced, certify_milestone_two_verified_topology_commit_traced,
    certify_verified_topology_commit_traced,
};
use crate::fixtures::authored_topology::milestone_one_default_corpus_scenarios;
use crate::fixtures::branch_replay_cases::milestone_one_default_branch_local_admitted_scenarios;
use crate::fixtures::validated_topology::{
    seeded_bootstrap, verified_primitive, verified_primitive_on_branch,
};

mod closeout;
mod milestone_three;
mod parameter_sweeps;
mod primitive_corpus;
mod query_runtime;
mod read_surfaces;
mod support;
mod verified_commit;
