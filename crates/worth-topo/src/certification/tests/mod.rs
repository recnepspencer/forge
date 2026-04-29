use crate::certification::report::WorthReplayParityStatus;
use crate::facade::{
    milestone_one_closeout_requirements, milestone_one_closeout_suite_definition,
    milestone_two_closeout_requirements, milestone_two_closeout_suite_definition,
    WorthAdmittedRangeSweepReport, WorthBridgeProofReport, WorthCertificationCanonicalRow,
    WorthCertificationParityRow, WorthCertificationRejectionRow, WorthCertificationRequiredOutput,
    WorthCertificationSuiteDefinition, WorthDerivedEquivalenceContractAggregateReport,
    WorthDerivedFallbackAggregateReport, WorthDerivedInvalidationAggregateReport,
    WorthDerivedRebuildAggregateReport, WorthDerivedValidatorCoverageReport,
    WorthDeterministicDigest, WorthFailureLocalityReport, WorthMilestoneOneCloseoutReport,
    WorthMilestoneOneCounters, WorthMilestoneTwoCloseoutReport, WorthMilestoneTwoCounters,
    WorthPrimitiveCorpusCoverageMatrix, WorthPrimitiveCorpusParityReport,
};
use forge_relational::facade::history::BranchId;
use serde_json::Value;
use worth_schema::facade::{
    RawWorthTopologyIntent, WorthMilestoneOnePrimitiveCase,
    WorthMilestoneOnePrimitiveExpectedOutcome, WorthMilestoneOnePrimitiveRole, WorthMutationOrigin,
    WorthTopologyAuthority, WorthTopologyMutation,
};
use worth_schema::facade::{WorthShellInterpretationClass, WorthWireInterpretationClass};

use crate::certification::{
    certify_milestone_one_branch_local_primitive_scenarios, certify_milestone_one_closeout,
    certify_milestone_one_default_primitive_corpus, certify_milestone_one_primitive_corpus,
    certify_milestone_one_read_view_traced, certify_milestone_two_closeout,
    certify_milestone_two_default_derived_corpus, certify_milestone_two_read_view_traced,
    certify_milestone_two_verified_topology_commit_traced, certify_verified_topology_commit_traced,
};
use crate::fixtures::authored_topology::milestone_one_default_corpus_scenarios;
use crate::fixtures::branch_replay_cases::milestone_one_default_branch_local_admitted_scenarios;
use crate::fixtures::validated_topology::{
    seeded_bootstrap, verified_primitive, verified_primitive_on_branch,
};
use crate::reader::WorthTopologyReader;

mod closeout;
mod parameter_sweeps;
mod primitive_corpus;
mod query_import;
mod read_surfaces;
mod support;
mod verified_commit;
