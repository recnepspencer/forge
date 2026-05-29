use forge_query::facade::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryEntity, ForgeQueryInspection,
    ForgeQueryRuntimeStateKind,
};
use forge_relational::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode,
};
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{
    FallbackDisposition, MutationOrigin, TopologyMutationBatch,
};
use schema::facade::platform::authority::{ShellInterpretationClass, WireInterpretationClass};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::shared::{count_batch_mutations, coverage_entry, digest_rows};
use crate::certification::support::reporting::{
    BranchLocalTopologyReport, MilestoneOneCertificationReport, MilestoneOneCounters,
    NamingAttachmentReport, PrimitiveFamilyCoverageMatrix, ReplayParityReport, ReplayParityStatus,
    TopologyLocalizationEntityRow, TopologyLocalizationRelationRow, TopologyLocalizationReport,
};
use crate::certification::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure, DecisionTrace,
    DerivedTraceAnchor, DerivedTraceEvidence, IntegrityMarkers, NamedCounter,
    PerformanceAccounting,
};
use crate::facade::{
    build_derived_equivalence_contract, build_topology_read_artifact, certify_topology_view,
    compare_derived_equivalence_contracts, validate_named_topology_truth,
};
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};

pub type TracedMilestoneOneCertificationReport = BoundaryEnvelope<MilestoneOneCertificationReport>;

struct MilestoneOneQueryEvidence {
    affected_live_view_count: usize,
    affected_derived_view_count: usize,
    considered_computed_view_count: usize,
    topology_entity_row_count: usize,
    topology_relation_row_count: usize,
    persistent_name_row_count: usize,
    validation_materialized_row_count: usize,
    equivalence_materialized_row_count: usize,
    declared_aspect_operation_count: usize,
    mutation_metadata_key_count: usize,
}

pub struct MilestoneOneCertificationHarness;

mod localization_report;
mod query_evidence;
mod read_basis_trace;

pub(crate) use query_evidence::certification_integrity_markers;
