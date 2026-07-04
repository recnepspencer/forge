use super::error::ConflictBatchAdmissionInventoryError;
use super::row::{
    ConflictBatchAdmissionAuthorityKind as AuthorityKind,
    ConflictBatchAdmissionCertificationPosture as CertificationPosture,
    ConflictBatchAdmissionCostPosture as CostPosture,
    ConflictBatchAdmissionDisposition as Disposition, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionOwner as Owner, ConflictBatchAdmissionQuerySurface as QuerySurface,
    ConflictBatchAdmissionReplacementPhase as ReplacementPhase,
    ConflictBatchAdmissionRowScope as RowScope, ConflictBatchAdmissionSurfaceIdentity as Surface,
};

pub(crate) fn phase_thirteen_firewall_rows(
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    let mut rows = vec![row(
            Surface::ConflictInputLookupRouteDenial,
            "crates/worth-kernel/src/workload_composition/conflict_input/spatial.rs",
            "conflict_lookup_route_denial",
            Owner::WorthKernel,
            "admit_spatial_conflict_input",
            AuthorityKind::SpatialTouchAuthorityAdmission,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            "spatial conflict input still lowers lookup route denial into a typed kernel admission error",
            "phase 13 public/read cutover keeps lookup route denial explicit while typed products carry the reusable authority",
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::PriorProofBoundary,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?];
    rows.extend(dual_row(
            Surface::LookupConsumedWorkloadReuseProductSerialization,
            Surface::LookupConsumedWorkloadReuseProductCompatibility,
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/mod.rs",
            "LookupConsumedWorkloadReuseProduct",
            "LookupConsumedWorkloadComposition::admit_lookup_reuse_resolution",
            "lookup-consumed workload exposes reused vs rebuilt products as a typed route result instead of caller-owned rows",
            "phase 13 public/read cutover consumes typed reuse products and query-boundary proof instead of local stability folklore",
        )?);
    rows.extend(dual_row(
            Surface::LookupConsumedWorkloadRequireResolutionProductSerialization,
            Surface::LookupConsumedWorkloadRequireResolutionProductCompatibility,
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/mod.rs",
            "LookupConsumedWorkloadComposition::require_resolution_product",
            "LookupConsumedWorkloadComposition::admit_lookup_reuse_resolution",
            "lookup-consumed workload still proves reused or rebuilt products match the handoff before exposure",
            "phase 13 public/read cutover preserves typed handoff/product proof and denies local compatibility folklore",
        )?);
    rows.extend(dual_row(
            Surface::LookupConsumedWorkloadMismatchLocusNameSerialization,
            Surface::LookupConsumedWorkloadMismatchLocusNameCompatibility,
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/reuse_resolution_denial.rs",
            "mismatch_locus_name",
            "LookupConsumedWorkloadReuseResolutionDenied::human_reason",
            "lookup-consumed denial still lowers typed mismatch loci into stable names for public denial narratives",
            "phase 13 public/read cutover keeps denial explanation attached to typed mismatch loci rather than rendered-row coincidence",
        )?);
    Ok(rows)
}

fn dual_row(
    serialization_surface: Surface,
    compatibility_surface: Surface,
    source_path: &'static str,
    surface_name: &'static str,
    current_caller: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
) -> Result<Vec<ConflictBatchAdmissionInventoryRow>, ConflictBatchAdmissionInventoryError> {
    Ok(vec![
        row(
            serialization_surface,
            source_path,
            surface_name,
            Owner::WorthKernel,
            current_caller,
            AuthorityKind::LookupConsumedWorkloadAdmission,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            blocker,
            removal_trigger,
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::ReceiptBackedTypedLookup,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
        row(
            compatibility_surface,
            source_path,
            surface_name,
            Owner::WorthKernel,
            current_caller,
            AuthorityKind::CompatibilityPostureAdmission,
            Disposition::Migrate,
            ReplacementPhase::PhaseFourAdmittedConflictInput,
            blocker,
            removal_trigger,
            CertificationPosture::OrdinaryProductionReachable,
            CostPosture::ReceiptBackedTypedLookup,
            QuerySurface::NotQuery,
            RowScope::ConcreteSource,
        )?,
    ])
}

fn row(
    surface_identity: Surface,
    source_path: &'static str,
    surface_name: &'static str,
    owner: Owner,
    current_caller: &'static str,
    authority_kind: AuthorityKind,
    disposition: Disposition,
    replacement_phase: ReplacementPhase,
    blocker: &'static str,
    removal_trigger: &'static str,
    certification_posture: CertificationPosture,
    cost_posture: CostPosture,
    query_surface: QuerySurface,
    row_scope: RowScope,
) -> Result<ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionInventoryError> {
    ConflictBatchAdmissionInventoryRow::builder()
        .surface_identity(surface_identity)
        .source_path(source_path)
        .surface_name(surface_name)
        .owner(owner)
        .current_caller(current_caller)
        .authority_kind(authority_kind)
        .disposition(disposition)
        .replacement_phase(replacement_phase)
        .blocker(blocker)
        .removal_trigger(removal_trigger)
        .certification_posture(certification_posture)
        .cost_posture(cost_posture)
        .query_surface(query_surface)
        .row_scope(row_scope)
        .build()
}
