use forge_proof::TransitionReadiness;
use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, project_workspace_support_snapshot,
    query_test_backend_residue_audit, support_pinning_contract, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture,
};
use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::{
    config::ForgeServerQueryHandoffConfig, query_handoff::ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceBindingError, ForgeServerRequestContextFacade,
    ForgeServerRequestContextInput,
};

use super::{
    covered_path_inventory, covered_paths, forge_server_query_boundary_source_inventory,
    ForgeServerQueryDependencyAuditProvenance, ForgeServerQueryDependencyAuditReceipt,
    ForgeServerQueryDependencyAuditRow, ForgeServerQueryDependencyAuditRowId,
    ForgeServerQueryDependencyBindingKind, ForgeServerQueryDependencyBoundaryAuditProvenance,
    ForgeServerQueryDependencyClosurePosture, ForgeServerQueryDependencyConsumerKitPosture,
    ForgeServerQueryDependencyCoveredPath, ForgeServerQueryDependencyRuntimeReadiness,
    ForgeServerQueryDependencyScopePosture, ForgeServerQueryDependencySupportPinProvenance,
    ForgeServerQueryDependencyTestBackendResidueProvenance,
};

const TEST_BACKEND_SUPPORT_ROOT: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/query_handoff");

pub(crate) fn run_query_dependency_audit(
    request_contexts: &ForgeServerRequestContextFacade,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
) -> ForgeServerQueryDependencyAuditReceipt {
    let inventory = covered_path_inventory();
    let mut rows = covered_paths()
        .into_iter()
        .map(|path| classify_path(request_contexts, query_handoff_config, path))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.row_id().cmp(right.row_id()));
    ForgeServerQueryDependencyAuditReceipt::new(inventory, rows)
}

fn classify_path(
    request_contexts: &ForgeServerRequestContextFacade,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    path: ForgeServerQueryDependencyCoveredPath,
) -> ForgeServerQueryDependencyAuditRow {
    if matches!(
        path.binding_kind,
        ForgeServerQueryDependencyBindingKind::StaticTestOnly
    ) {
        return classify_static_test_only_path(path);
    }
    if matches!(
        path.binding_kind,
        ForgeServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit
    ) {
        return classify_boundary_audit_path(path);
    }

    let (consumer_kit_posture, closure_posture, provenance, reason) =
        classify_query_bound_path(request_contexts, query_handoff_config, &path);
    ForgeServerQueryDependencyAuditRow::new(
        ForgeServerQueryDependencyAuditRowId::new(path.row_id),
        path.path_kind,
        path.runtime_readiness,
        consumer_kit_posture,
        scope_posture_for(path.path_kind),
        closure_posture,
        path.ordinary_path,
        provenance,
        reason,
    )
}

fn classify_static_test_only_path(
    path: ForgeServerQueryDependencyCoveredPath,
) -> ForgeServerQueryDependencyAuditRow {
    let report = query_test_backend_residue_audit("forge-server")
        .required_root(TEST_BACKEND_SUPPORT_ROOT)
        .evaluate();
    let (consumer_kit_posture, provenance, reason) = match report {
        Ok(report) if report.finding_count() == 0 => (
            ForgeServerQueryDependencyConsumerKitPosture::QueryTestBackendResidueAuditAdopted,
            ForgeServerQueryDependencyAuditProvenance::TestBackendResidue(
                ForgeServerQueryDependencyTestBackendResidueProvenance::new(
                    report.audited_roots().to_vec(),
                    report
                        .report_identity()
                        .terminal_projection_for_reporting()
                        .to_string(),
                    report.finding_count(),
                    report.scanned_file_count(),
                ),
            ),
            format!(
                "query test backend residue audit clean across {} scanned files",
                report.scanned_file_count()
            ),
        ),
        Ok(report) => (
            ForgeServerQueryDependencyConsumerKitPosture::LocalFolklore,
            ForgeServerQueryDependencyAuditProvenance::TestBackendResidue(
                ForgeServerQueryDependencyTestBackendResidueProvenance::new(
                    report.audited_roots().to_vec(),
                    report
                        .report_identity()
                        .terminal_projection_for_reporting()
                        .to_string(),
                    report.finding_count(),
                    report.scanned_file_count(),
                ),
            ),
            format!(
                "query test backend residue findings: {} across {} scanned files",
                report.finding_count(),
                report.scanned_file_count()
            ),
        ),
        Err(error) => (
            ForgeServerQueryDependencyConsumerKitPosture::LocalFolklore,
            ForgeServerQueryDependencyAuditProvenance::TestBackendResidue(
                ForgeServerQueryDependencyTestBackendResidueProvenance::new(
                    vec![TEST_BACKEND_SUPPORT_ROOT.to_string()],
                    "query-test-backend-residue-audit-error".to_string(),
                    usize::MAX,
                    0,
                ),
            ),
            format!("query test backend residue audit failed: {error:?}"),
        ),
    };

    ForgeServerQueryDependencyAuditRow::new(
        ForgeServerQueryDependencyAuditRowId::new(path.row_id),
        path.path_kind,
        ForgeServerQueryDependencyRuntimeReadiness::StaticTestOnly,
        consumer_kit_posture,
        ForgeServerQueryDependencyScopePosture::StaticTestOnly,
        ForgeServerQueryDependencyClosurePosture::StaticTestOnly,
        false,
        provenance,
        reason,
    )
}

fn classify_boundary_audit_path(
    path: ForgeServerQueryDependencyCoveredPath,
) -> ForgeServerQueryDependencyAuditRow {
    let inventory = forge_server_query_boundary_source_inventory();
    let (consumer_kit_posture, closure_posture, provenance, reason) =
        match hard_prohibition_boundary_audit()
            .covering_sources(inventory.boundary_sources())
            .evaluate()
        {
            Ok(report) if report.findings().is_empty() => (
                ForgeServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditAdopted,
                ForgeServerQueryDependencyClosurePosture::Ready,
                ForgeServerQueryDependencyAuditProvenance::BoundaryAudit(
                    ForgeServerQueryDependencyBoundaryAuditProvenance::new(
                        inventory.required_roots().to_vec(),
                        inventory.source_paths().to_vec(),
                        inventory.inventory_digest().to_string(),
                        report
                            .report_identity()
                            .terminal_projection_for_reporting()
                            .to_string(),
                        report.findings().len(),
                        report.parsed_item_count(),
                        report.visited_call_count(),
                    ),
                ),
                format!(
                    "query boundary audit clean with report `{}` across {} sources from inventory `{}`",
                    report.report_identity().terminal_projection_for_reporting(),
                    report.source_labels().len(),
                    inventory.inventory_digest()
                ),
            ),
            Ok(report) => (
                ForgeServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditBlocked,
                ForgeServerQueryDependencyClosurePosture::Blocked,
                ForgeServerQueryDependencyAuditProvenance::BoundaryAudit(
                    ForgeServerQueryDependencyBoundaryAuditProvenance::new(
                        inventory.required_roots().to_vec(),
                        inventory.source_paths().to_vec(),
                        inventory.inventory_digest().to_string(),
                        report
                            .report_identity()
                            .terminal_projection_for_reporting()
                            .to_string(),
                        report.findings().len(),
                        report.parsed_item_count(),
                        report.visited_call_count(),
                    ),
                ),
                format!(
                    "query boundary audit found {} prohibited seam usage finding(s) in report `{}`",
                    report.findings().len(),
                    report.report_identity().terminal_projection_for_reporting()
                ),
            ),
            Err(error) => (
                ForgeServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditBlocked,
                ForgeServerQueryDependencyClosurePosture::Blocked,
                ForgeServerQueryDependencyAuditProvenance::BoundaryAudit(
                    ForgeServerQueryDependencyBoundaryAuditProvenance::new(
                        inventory.required_roots().to_vec(),
                        inventory.source_paths().to_vec(),
                        inventory.inventory_digest().to_string(),
                        "query-boundary-audit-error".to_string(),
                        usize::MAX,
                        0,
                        0,
                    ),
                ),
                format!("query-boundary-audit-error: {error}"),
            ),
        };

    ForgeServerQueryDependencyAuditRow::new(
        ForgeServerQueryDependencyAuditRowId::new(path.row_id),
        path.path_kind,
        path.runtime_readiness,
        consumer_kit_posture,
        scope_posture_for(path.path_kind),
        closure_posture,
        path.ordinary_path,
        provenance,
        reason,
    )
}

fn classify_query_bound_path(
    request_contexts: &ForgeServerRequestContextFacade,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    path: &ForgeServerQueryDependencyCoveredPath,
) -> (
    ForgeServerQueryDependencyConsumerKitPosture,
    ForgeServerQueryDependencyClosurePosture,
    ForgeServerQueryDependencyAuditProvenance,
    String,
) {
    match evaluate_query_bound_support(request_contexts, query_handoff_config, path) {
        Ok(provenance) if provenance.blocking_finding_count() == 0 => (
            ForgeServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningAdopted,
            ForgeServerQueryDependencyClosurePosture::Ready,
            ForgeServerQueryDependencyAuditProvenance::QuerySupportPin(provenance.clone()),
            format!(
                "workspace `{}` snapshot `{}` satisfied {} required families with report `{}`",
                provenance.workspace_name(),
                provenance.support_snapshot_digest(),
                provenance.required_families().len(),
                provenance.report_digest()
            ),
        ),
        Ok(provenance) => (
            ForgeServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningBlocked,
            ForgeServerQueryDependencyClosurePosture::Blocked,
            ForgeServerQueryDependencyAuditProvenance::QuerySupportPin(provenance.clone()),
            format!(
                "workspace `{}` snapshot `{}` support pin findings: {} blocking across report `{}`",
                provenance.workspace_name(),
                provenance.support_snapshot_digest(),
                provenance.blocking_finding_count(),
                provenance.report_digest()
            ),
        ),
        Err(reason) => (
            ForgeServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningBlocked,
            ForgeServerQueryDependencyClosurePosture::Blocked,
            ForgeServerQueryDependencyAuditProvenance::QuerySupportPin(
                ForgeServerQueryDependencySupportPinProvenance::new(
                    "unbound-workspace".to_string(),
                    path.required_query_families.to_vec(),
                    reason.clone(),
                    reason.clone(),
                    reason.clone(),
                    reason.clone(),
                    usize::MAX,
                    0,
                ),
            ),
            reason,
        ),
    }
}

fn evaluate_query_bound_support(
    request_contexts: &ForgeServerRequestContextFacade,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    path: &ForgeServerQueryDependencyCoveredPath,
) -> Result<ForgeServerQueryDependencySupportPinProvenance, String> {
    let request_input = ForgeServerRequestContextInput::builder()
        .with_surface_family(path.surface_family())
        .with_transport_class(path.transport_class())
        .with_authenticated_principal_id("audit-principal")
        .with_tenant_id("audit-tenant")
        .with_workspace_id("audit-workspace")
        .build()
        .map_err(|error| format!("request-context-input-error: {error:?}"))?;
    let resolved = match request_contexts.resolve(request_input) {
        TransitionReadiness::Ready(resolved) => resolved,
        other => {
            return Err(format!("request-context-resolution-not-ready: {other:?}"));
        }
    };
    let binding_request =
        ForgeServerQueryWorkspaceBindingRequest::for_query_handoff(resolved, path.operation());
    let workspace = query_handoff_config
        .workspace_provider()
        .bind_workspace(&binding_request)
        .map_err(binding_error_reason)?;
    let snapshot = project_workspace_support_snapshot(&workspace);
    let mut builder = support_pinning_contract(path.row_id)
        .against_snapshot(&snapshot)
        .map_err(|error| format!("support-pinning-contract-error: {error:?}"))?;
    for family in path.required_query_families {
        builder = require_supported_family(builder, *family)
            .map_err(|error| format!("support-pinning-require-family-error: {error:?}"))?;
    }
    let contract = builder
        .seal()
        .map_err(|error| format!("support-pinning-seal-error: {error:?}"))?;
    let report = contract
        .evaluate_snapshot(&snapshot)
        .map_err(|error| format!("support-pinning-evaluation-error: {error:?}"))?;

    Ok(ForgeServerQueryDependencySupportPinProvenance::new(
        workspace.name().to_string(),
        path.required_query_families.to_vec(),
        snapshot.source_matrix_digest().to_string(),
        snapshot.snapshot_digest().to_string(),
        report.contract_digest().to_string(),
        report.report_digest().to_string(),
        report.blocking_finding_count(),
        report.matched_required_count(),
    ))
}

fn binding_error_reason(error: ForgeServerQueryWorkspaceBindingError) -> String {
    format!(
        "workspace-binding-failed:{}:{}",
        error.stage(),
        error.message()
    )
}

fn scope_posture_for(
    path_kind: super::ForgeServerQueryDependencyAuditPathKind,
) -> ForgeServerQueryDependencyScopePosture {
    match path_kind {
        super::ForgeServerQueryDependencyAuditPathKind::DirectDeclarationSupportPosture
        | super::ForgeServerQueryDependencyAuditPathKind::ServerConsumerBoundaryAudit => {
            ForgeServerQueryDependencyScopePosture::ConsumerKitScoped
        }
        _ => ForgeServerQueryDependencyScopePosture::QueryFamilyScoped,
    }
}

fn require_supported_family(
    builder: forge_query::ForgeQuerySupportPinContractBuilder,
    family: ForgeQueryRuntimeFacadeFamily,
) -> Result<
    forge_query::ForgeQuerySupportPinContractBuilder,
    forge_query::ForgeQuerySupportPinningError,
> {
    builder.require_family(family, |row| {
        row.status(ForgeQueryPinnedSupportStatus::Supported)
            .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
            .bind_live_row_digest()
    })
}
