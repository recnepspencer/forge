use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode,
};
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{FallbackDisposition, MutationOrigin, TopologyMutation};
use schema::facade::platform::authority::{ShellInterpretationClass, WireInterpretationClass};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::shared::{count_topology_mutations, coverage_entry, digest_rows};
use crate::certification::support::reporting::{
    BranchLocalTopologyReport, MilestoneOneCertificationReport, MilestoneOneCounters,
    NamingAttachmentReport, PrimitiveFamilyCoverageMatrix, ReplayParityReport, ReplayParityStatus,
    TopologyLocalizationEntityRow, TopologyLocalizationRelationRow, TopologyLocalizationReport,
};
use crate::certification::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure, DecisionTrace,
    DerivedTraceAnchor, DerivedTraceEvidence, IntegrityMarkers, NamedCounter,
    PerformanceAccounting,
};
use crate::derived_topology::traversal_views::{
    build_topology_read_artifact, certify_topology_view,
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
