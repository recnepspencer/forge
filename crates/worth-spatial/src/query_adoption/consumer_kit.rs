use forge_query::facade::runtime::{
    ForgeQueryGraphObligationDispatchError, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchReadVerb,
    ForgeQueryGraphTouchSelector,
};
use forge_query::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow, ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ForgeQueryGraphObligationSupportPin,
};

use crate::workload_platform::evidence_ledger::SpatialEvidenceQueryTouchDescriptor;

const CONSUMER_NAME: &str = "worth-spatial";
const RUNTIME_FAMILY: &str = "worth-spatial-evidence-touch-authority";
const COLLECTION: &str = "worth.spatial.evidence_touch";
const QUERY_ADOPTION_SOURCE_LABEL: &str = "crates/worth-spatial/src/query_adoption.rs";
const FACADE_QUERY_ADOPTION_SOURCE_LABEL: &str =
    "crates/worth-spatial/src/facade/query_adoption.rs";
const PERFORMANCE_COUNTERS_SOURCE_LABEL: &str =
    "crates/worth-spatial/src/query_adoption/performance_counters.rs";
const RESIDUE_SOURCE_LABEL: &str = "crates/worth-spatial/src/query_adoption/residue.rs";
const SUPPORT_PROJECTION_SOURCE_LABEL: &str =
    "crates/worth-spatial/src/query_adoption/support_projection.rs";

const QUERY_ADOPTION_RS: &str = include_str!("../query_adoption.rs");
const FACADE_QUERY_ADOPTION_RS: &str = include_str!("../facade/query_adoption.rs");
const PERFORMANCE_COUNTERS_RS: &str = include_str!("performance_counters.rs");
const RESIDUE_RS: &str = include_str!("residue.rs");
const SUPPORT_PROJECTION_RS: &str = include_str!("support_projection.rs");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthSpatialQueryConsumerKitAdoptionStatus {
    support_pin_report_digest: String,
    support_requirement_count: usize,
    support_observed_row_count: usize,
    support_matched_required_count: usize,
    support_snapshot_row_count: usize,
    support_blocking_finding_count: usize,
    evidence_report_identity: String,
    evidence_digest_participation_identity: String,
    boundary_audit_report_identity: String,
    boundary_audit_source_count: usize,
    boundary_audit_coverage_row_count: usize,
    workload_support_pin_row_count: usize,
    hard_prohibition_audit_clean: bool,
    adoption_manifest_digest: String,
    execution_proof_digest: String,
    selected_obligation_count: usize,
    execution_row_count: usize,
    attempted_bucket_lookup_count: usize,
    candidate_registration_count: usize,
    denied_row_count: usize,
    full_scan_count: usize,
    residue_row_count: usize,
}

impl WorthSpatialQueryConsumerKitAdoptionStatus {
    pub fn support_pin_report_digest(&self) -> &str {
        &self.support_pin_report_digest
    }

    pub const fn support_requirement_count(&self) -> usize {
        self.support_requirement_count
    }

    pub const fn support_observed_row_count(&self) -> usize {
        self.support_observed_row_count
    }

    pub const fn support_matched_required_count(&self) -> usize {
        self.support_matched_required_count
    }

    pub const fn support_snapshot_row_count(&self) -> usize {
        self.support_snapshot_row_count
    }

    pub const fn support_blocking_finding_count(&self) -> usize {
        self.support_blocking_finding_count
    }

    pub fn evidence_report_identity(&self) -> &str {
        &self.evidence_report_identity
    }

    pub fn evidence_digest_participation_identity(&self) -> &str {
        &self.evidence_digest_participation_identity
    }

    pub fn boundary_audit_report_identity(&self) -> &str {
        &self.boundary_audit_report_identity
    }

    pub const fn boundary_audit_source_count(&self) -> usize {
        self.boundary_audit_source_count
    }

    pub const fn boundary_audit_coverage_row_count(&self) -> usize {
        self.boundary_audit_coverage_row_count
    }

    pub const fn workload_support_pin_row_count(&self) -> usize {
        self.workload_support_pin_row_count
    }

    pub const fn hard_prohibition_audit_clean(&self) -> bool {
        self.hard_prohibition_audit_clean
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        &self.adoption_manifest_digest
    }

    pub fn execution_proof_digest(&self) -> &str {
        &self.execution_proof_digest
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn execution_row_count(&self) -> usize {
        self.execution_row_count
    }

    pub const fn attempted_bucket_lookup_count(&self) -> usize {
        self.attempted_bucket_lookup_count
    }

    pub const fn candidate_registration_count(&self) -> usize {
        self.candidate_registration_count
    }

    pub const fn denied_row_count(&self) -> usize {
        self.denied_row_count
    }

    pub const fn full_scan_count(&self) -> usize {
        self.full_scan_count
    }

    pub const fn residue_row_count(&self) -> usize {
        self.residue_row_count
    }
}

#[derive(Debug)]
pub enum WorthSpatialQueryConsumerKitAdoptionError {
    QueryConsumerKit(ForgeQueryGraphObligationConsumerKitError),
    QueryDescriptor(ForgeQueryGraphTouchDescriptorDenial),
    QueryRegistration(ForgeQueryGraphObligationRegistrationDenial),
    QueryRuleIdentity(ForgeQueryGraphObligationDispatchError),
}

pub fn spatial_query_graph_obligation_adoption_proof() -> Result<
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    WorthSpatialQueryConsumerKitAdoptionError,
> {
    let descriptor = spatial_graph_touch_descriptor()?;
    let operating_world = spatial_operating_world_descriptor();
    prove_spatial_query_graph_obligation_adoption(&descriptor, &operating_world)
}

pub fn spatial_query_graph_obligation_adoption_proof_for_descriptor(
    descriptor: &SpatialEvidenceQueryTouchDescriptor,
) -> Result<
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    WorthSpatialQueryConsumerKitAdoptionError,
> {
    prove_spatial_query_graph_obligation_adoption(
        descriptor.touch_descriptor(),
        descriptor.operating_world(),
    )
}

fn prove_spatial_query_graph_obligation_adoption(
    descriptor: &ForgeQueryGraphTouchDescriptor,
    operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
) -> Result<
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    WorthSpatialQueryConsumerKitAdoptionError,
> {
    let registration = spatial_graph_obligation_registration()?;
    graph_obligation_consumer_kit(CONSUMER_NAME)
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                RUNTIME_FAMILY,
                [registration],
            )
            .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryConsumerKit)?,
        )
        .declare_selector_coverage(spatial_selector_coverage()?)
        .pin_support(spatial_support_pin())
        .against_support_matrix(
            ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation(),
        )
        .audit_local_ceremony(spatial_local_ceremony_audit())
        .account_for_residue(spatial_query_graph_obligation_residue_manifest()?)
        .prove_execution_with(descriptor, operating_world)
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryConsumerKit)?
        .prove_adoption_with_execution()
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryConsumerKit)
}

pub fn current_spatial_query_consumer_kit_adoption_status(
) -> Result<WorthSpatialQueryConsumerKitAdoptionStatus, WorthSpatialQueryConsumerKitAdoptionError> {
    let proof = spatial_query_graph_obligation_adoption_proof()?;
    let manifest = proof.manifest();
    let support_pin = proof.support_pin();
    let support_matrix = ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();
    let audit = proof.local_ceremony_audit();
    let residue_manifest = proof.residue_manifest();
    let execution_proof = proof.execution_proof();
    let selection_counters = execution_proof.selection_proof().selection_counters();
    let selected_obligation_count = execution_proof.selected_obligation_count();
    let execution_row_count = execution_proof.rows().len();

    Ok(WorthSpatialQueryConsumerKitAdoptionStatus {
        support_pin_report_digest: support_pin.pin_digest().to_string(),
        support_requirement_count: support_pin.row_count(),
        support_observed_row_count: support_matrix
            .rows_for_lane(ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection)
            .count(),
        support_matched_required_count: support_pin.row_count(),
        support_snapshot_row_count: support_matrix.rows().len(),
        support_blocking_finding_count: support_pin.findings(&support_matrix).len(),
        evidence_report_identity: manifest.manifest_digest().to_string(),
        evidence_digest_participation_identity: proof
            .adoption_proof()
            .manifest()
            .manifest_digest()
            .to_string(),
        boundary_audit_report_identity: audit.audit_digest().to_string(),
        boundary_audit_source_count: audit.evaluated_source_count(),
        boundary_audit_coverage_row_count: audit.findings().len(),
        workload_support_pin_row_count: support_pin.row_count(),
        hard_prohibition_audit_clean: audit.is_clean(),
        adoption_manifest_digest: manifest.manifest_digest().to_string(),
        execution_proof_digest: execution_proof.proof_digest().to_string(),
        selected_obligation_count,
        execution_row_count,
        attempted_bucket_lookup_count: selection_counters.attempted_bucket_lookup_count(),
        candidate_registration_count: selection_counters.candidate_registration_count(),
        denied_row_count: selected_obligation_count.saturating_sub(execution_row_count),
        full_scan_count: selection_counters.registration_full_scan_count(),
        residue_row_count: residue_manifest.rows().len(),
    })
}

pub fn spatial_query_graph_obligation_residue_manifest(
) -> Result<ForgeQueryGraphObligationResidueManifest, WorthSpatialQueryConsumerKitAdoptionError> {
    ForgeQueryGraphObligationResidueManifest::capped([
        ForgeQueryGraphObligationResidueRow::explicit(
            "worth-spatial-runtime-facade-support-projection",
            "worth-spatial",
            "touched-graph-milestone-4-phase-8",
            1,
            1,
            "public facade still exposes current_spatial_workload_support_pin_rows for older query-native closeout consumers",
            "delete support_projection.rs facade export after Milestone 6.5 consumes graph-obligation adoption status directly",
            "capped-residue",
        )
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryConsumerKit)?,
    ])
    .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryConsumerKit)
}

fn spatial_graph_obligation_registration(
) -> Result<ForgeQueryGraphObligationRegistration, WorthSpatialQueryConsumerKitAdoptionError> {
    Ok(ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth-spatial",
            "spatial-evidence-touch-query-adoption",
            "1.0.0",
        )
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryRuleIdentity)?,
        ForgeQueryGraphTouchSelector::collection(COLLECTION)
            .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryRegistration)?,
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
    )))
}

fn spatial_selector_coverage() -> Result<
    ForgeQueryGraphObligationSelectorCoverageDeclaration,
    WorthSpatialQueryConsumerKitAdoptionError,
> {
    Ok(
        ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
            "spatial evidence touch collection coverage",
            ForgeQueryGraphTouchSelector::collection(COLLECTION)
                .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryRegistration)?,
        )]),
    )
}

fn spatial_support_pin() -> ForgeQueryGraphObligationSupportPin {
    ForgeQueryGraphObligationSupportPin::supported([(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
    )])
}

fn spatial_graph_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, WorthSpatialQueryConsumerKitAdoptionError> {
    ForgeQueryGraphTouchDescriptor::read_family(
        COLLECTION,
        [
            ForgeQueryGraphTouchReadVerb::ObservesCollection,
            ForgeQueryGraphTouchReadVerb::ObservesRelationKind,
            ForgeQueryGraphTouchReadVerb::ObservesAspectPath,
            ForgeQueryGraphTouchReadVerb::MaterializesDiagnostic,
            ForgeQueryGraphTouchReadVerb::CrossesOperatingWorld,
        ],
    )
    .map_err(WorthSpatialQueryConsumerKitAdoptionError::QueryDescriptor)
}

fn spatial_operating_world_descriptor() -> ForgeQueryGraphObligationOperatingWorldDescriptor {
    ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority()
}

fn spatial_local_ceremony_audit() -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(&spatial_local_ceremony_source_set())
}

fn spatial_local_ceremony_source_set() -> ForgeQueryBoundaryAuditSourceSet {
    ForgeQueryBoundaryAuditSourceSet::new("worth-spatial")
        .source_file(
            QUERY_ADOPTION_SOURCE_LABEL,
            QUERY_ADOPTION_SOURCE_LABEL,
            QUERY_ADOPTION_RS,
        )
        .source_file(
            FACADE_QUERY_ADOPTION_SOURCE_LABEL,
            FACADE_QUERY_ADOPTION_SOURCE_LABEL,
            FACADE_QUERY_ADOPTION_RS,
        )
        .source_file(
            PERFORMANCE_COUNTERS_SOURCE_LABEL,
            PERFORMANCE_COUNTERS_SOURCE_LABEL,
            PERFORMANCE_COUNTERS_RS,
        )
        .source_file(RESIDUE_SOURCE_LABEL, RESIDUE_SOURCE_LABEL, RESIDUE_RS)
        .source_file(
            SUPPORT_PROJECTION_SOURCE_LABEL,
            SUPPORT_PROJECTION_SOURCE_LABEL,
            SUPPORT_PROJECTION_RS,
        )
}

#[cfg(test)]
mod tests;
