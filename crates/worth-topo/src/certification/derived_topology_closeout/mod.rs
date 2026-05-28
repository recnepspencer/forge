use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryInspection, ForgeQueryRuntimeStateKind,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::CommitResult;
use schema::facade::platform::authority::DerivedTopologyReadBasis;

use crate::certification::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure,
    DecisionTrace, DerivedTraceAnchor, DerivedTraceEvidence, NamedCounter,
    PerformanceAccounting, TraceAvailability,
};

use crate::certification::authority_closeout::read_view::certification_integrity_markers;
use crate::certification::bridge::certify_milestone_one_bridge_proof;
use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::primitive_corpus::certify_milestone_one_default_primitive_corpus_impl;
use crate::certification::requirements::milestone_two_closeout_requirements;
use crate::certification::shared::digest_rows;
use crate::certification::support::parity::build_derived_equivalence_contract_report;
use crate::certification::support::reporting::{
    DerivedEquivalenceContractAggregateReport, DerivedEquivalenceContractAggregateRow,
    DerivedFallbackAggregateReport, DerivedFallbackAggregateRow, DerivedFamilyCoverageMatrix,
    DerivedFamilyCoverageRow, DerivedFamilyParityMatrix, DerivedFamilyParityRow,
    DerivedInvalidationAggregateReport, DerivedInvalidationAggregateRow,
    DerivedRebuildAggregateReport, DerivedRebuildAggregateRow, DerivedValidatorCoverageReport,
    DerivedValidatorCoverageRow, DeterministicDigest, FailureLocalityReport,
    MilestoneOneCertificationReport, MilestoneTwoBranchLocalParityReport,
    MilestoneTwoCloseoutReport, MilestoneTwoCounters, MilestoneTwoDerivedCorpusReport,
    MilestoneTwoDerivedReadReport, MilestoneTwoReplayParityReport, PrimitiveCorpusParityReport,
    PrimitiveCorpusReport,
};
use crate::facade::{
    build_topology_read_artifact, certify_topology_view, compare_derived_equivalence_contracts,
    validate_named_topology_truth, ReplayParityStatus, TopologyQueryAssembly,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};

pub type TracedMilestoneTwoDerivedReadReport = BoundaryEnvelope<MilestoneTwoDerivedReadReport>;

#[derive(Debug, Clone, Copy)]
struct MilestoneTwoQueryEvidence {
    affected_live_view_count: usize,
    affected_derived_view_count: usize,
    considered_computed_view_count: usize,
    validation_materialized_row_count: usize,
    equivalence_materialized_row_count: usize,
    validation_pending_refresh_fallback_count: usize,
    equivalence_pending_refresh_fallback_count: usize,
    declared_aspect_operation_count: usize,
    mutation_metadata_key_count: usize,
}

mod aggregate_reports;
mod closeout_assertions;
mod closeout_program;
mod derived_corpus;
mod read_basis;
mod traced_reports;

pub(crate) use closeout_program::certify_milestone_two_closeout_impl;
pub(crate) use derived_corpus::certify_milestone_two_default_derived_corpus_impl;
pub(crate) use read_basis::{
    certify_milestone_two_read_basis_runtime_traced_impl,
    certify_milestone_two_verified_commit_traced_impl,
};




