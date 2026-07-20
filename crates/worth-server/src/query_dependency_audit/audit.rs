use worth_proof::TransitionReadiness;
use worth_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, project_workspace_support_snapshot,
    query_test_backend_residue_audit, support_pinning_contract, WorthQueryPinnedSupportStatus,
    WorthQueryPinnedTeachingPosture, WorthQuerySupportPinContractBuilder,
    WorthQuerySupportPinningError,
};
use worth_query::facade::runtime::WorthQueryRuntimeFacadeFamily;

use crate::{
    config::WorthServerQueryHandoffConfig, query_handoff::WorthServerQueryWorkspaceBindingRequest,
    WorthServerQueryWorkspaceBindingError, WorthServerRequestContextFacade,
    WorthServerRequestContextInput,
};

use super::{
    covered_path_inventory, covered_paths, worth_server_query_boundary_source_inventory,
    WorthServerQueryDependencyAuditProvenance, WorthServerQueryDependencyAuditReceipt,
    WorthServerQueryDependencyAuditRow, WorthServerQueryDependencyAuditRowId,
    WorthServerQueryDependencyBindingKind, WorthServerQueryDependencyBoundaryAuditProvenance,
    WorthServerQueryDependencyClosurePosture, WorthServerQueryDependencyConsumerKitPosture,
    WorthServerQueryDependencyCoveredPath, WorthServerQueryDependencyRuntimeReadiness,
    WorthServerQueryDependencyScopePosture, WorthServerQueryDependencySupportPinProvenance,
    WorthServerQueryDependencyTestBackendResidueProvenance,
};

const TEST_BACKEND_SUPPORT_ROOT: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/query_handoff");

pub(crate) fn run_query_dependency_audit(
    request_contexts: &WorthServerRequestContextFacade,
    query_handoff_config: &WorthServerQueryHandoffConfig,
) -> WorthServerQueryDependencyAuditReceipt {
    let inventory = covered_path_inventory();
    let mut rows = covered_paths()
        .into_iter()
        .map(|path| classify_path(request_contexts, query_handoff_config, path))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.row_id().cmp(right.row_id()));
    WorthServerQueryDependencyAuditReceipt::new(inventory, rows)
}

fn classify_path(
    request_contexts: &WorthServerRequestContextFacade,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    path: WorthServerQueryDependencyCoveredPath,
) -> WorthServerQueryDependencyAuditRow {
    if matches!(
        path.binding_kind,
        WorthServerQueryDependencyBindingKind::StaticTestOnly
    ) {
        return classify_static_test_only_path(path);
    }
    if matches!(
        path.binding_kind,
        WorthServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit
    ) {
        return classify_boundary_audit_path(path);
    }

    let (consumer_kit_posture, closure_posture, provenance, reason) =
        classify_query_bound_path(request_contexts, query_handoff_config, &path);
    WorthServerQueryDependencyAuditRow::new(super::WorthServerQueryDependencyAuditRowParts {
        row_id: WorthServerQueryDependencyAuditRowId::new(path.row_id),
        path_kind: path.path_kind,
        runtime_readiness: path.runtime_readiness,
        consumer_kit_posture,
        scope_posture: scope_posture_for(path.path_kind),
        closure_posture,
        ordinary_path: path.ordinary_path,
        provenance,
        reason,
    })
}

fn classify_static_test_only_path(
    path: WorthServerQueryDependencyCoveredPath,
) -> WorthServerQueryDependencyAuditRow {
    let report = query_test_backend_residue_audit("worth-server")
        .required_root(TEST_BACKEND_SUPPORT_ROOT)
        .evaluate();
    let (consumer_kit_posture, provenance, reason) = match report {
        Ok(report) if report.finding_count() == 0 => (
            WorthServerQueryDependencyConsumerKitPosture::QueryTestBackendResidueAuditAdopted,
            WorthServerQueryDependencyAuditProvenance::TestBackendResidue(
                WorthServerQueryDependencyTestBackendResidueProvenance::new(
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
            WorthServerQueryDependencyConsumerKitPosture::LocalFolklore,
            WorthServerQueryDependencyAuditProvenance::TestBackendResidue(
                WorthServerQueryDependencyTestBackendResidueProvenance::new(
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
            WorthServerQueryDependencyConsumerKitPosture::LocalFolklore,
            WorthServerQueryDependencyAuditProvenance::TestBackendResidue(
                WorthServerQueryDependencyTestBackendResidueProvenance::new(
                    vec![TEST_BACKEND_SUPPORT_ROOT.to_string()],
                    "query-test-backend-residue-audit-error".to_string(),
                    usize::MAX,
                    0,
                ),
            ),
            format!("query test backend residue audit failed: {error:?}"),
        ),
    };

    WorthServerQueryDependencyAuditRow::new(super::WorthServerQueryDependencyAuditRowParts {
        row_id: WorthServerQueryDependencyAuditRowId::new(path.row_id),
        path_kind: path.path_kind,
        runtime_readiness: WorthServerQueryDependencyRuntimeReadiness::StaticTestOnly,
        consumer_kit_posture,
        scope_posture: WorthServerQueryDependencyScopePosture::StaticTestOnly,
        closure_posture: WorthServerQueryDependencyClosurePosture::StaticTestOnly,
        ordinary_path: false,
        provenance,
        reason,
    })
}

fn classify_boundary_audit_path(
    path: WorthServerQueryDependencyCoveredPath,
) -> WorthServerQueryDependencyAuditRow {
    let inventory = worth_server_query_boundary_source_inventory();
    let (consumer_kit_posture, closure_posture, provenance, reason) =
        match hard_prohibition_boundary_audit()
            .covering_sources(inventory.boundary_sources())
            .evaluate()
        {
            Ok(report) if report.findings().is_empty() => (
                WorthServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditAdopted,
                WorthServerQueryDependencyClosurePosture::Ready,
                WorthServerQueryDependencyAuditProvenance::BoundaryAudit(
                    WorthServerQueryDependencyBoundaryAuditProvenance::new(
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
                WorthServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditBlocked,
                WorthServerQueryDependencyClosurePosture::Blocked,
                WorthServerQueryDependencyAuditProvenance::BoundaryAudit(
                    WorthServerQueryDependencyBoundaryAuditProvenance::new(
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
                WorthServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditBlocked,
                WorthServerQueryDependencyClosurePosture::Blocked,
                WorthServerQueryDependencyAuditProvenance::BoundaryAudit(
                    WorthServerQueryDependencyBoundaryAuditProvenance::new(
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

    WorthServerQueryDependencyAuditRow::new(super::WorthServerQueryDependencyAuditRowParts {
        row_id: WorthServerQueryDependencyAuditRowId::new(path.row_id),
        path_kind: path.path_kind,
        runtime_readiness: path.runtime_readiness,
        consumer_kit_posture,
        scope_posture: scope_posture_for(path.path_kind),
        closure_posture,
        ordinary_path: path.ordinary_path,
        provenance,
        reason,
    })
}

fn classify_query_bound_path(
    request_contexts: &WorthServerRequestContextFacade,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    path: &WorthServerQueryDependencyCoveredPath,
) -> (
    WorthServerQueryDependencyConsumerKitPosture,
    WorthServerQueryDependencyClosurePosture,
    WorthServerQueryDependencyAuditProvenance,
    String,
) {
    match evaluate_query_bound_support(request_contexts, query_handoff_config, path) {
        Ok(provenance) if provenance.blocking_finding_count() == 0 => (
            WorthServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningAdopted,
            WorthServerQueryDependencyClosurePosture::Ready,
            WorthServerQueryDependencyAuditProvenance::QuerySupportPin(provenance.clone()),
            format!(
                "workspace `{}` snapshot `{}` satisfied {} required families with report `{}`",
                provenance.workspace_name(),
                provenance.support_snapshot_digest(),
                provenance.required_families().len(),
                provenance.report_digest()
            ),
        ),
        Ok(provenance) => (
            WorthServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningBlocked,
            WorthServerQueryDependencyClosurePosture::Blocked,
            WorthServerQueryDependencyAuditProvenance::QuerySupportPin(provenance.clone()),
            format!(
                "workspace `{}` snapshot `{}` support pin findings: {} blocking across report `{}`",
                provenance.workspace_name(),
                provenance.support_snapshot_digest(),
                provenance.blocking_finding_count(),
                provenance.report_digest()
            ),
        ),
        Err(reason) => (
            WorthServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningBlocked,
            WorthServerQueryDependencyClosurePosture::Blocked,
            WorthServerQueryDependencyAuditProvenance::QuerySupportPin(
                WorthServerQueryDependencySupportPinProvenance::new(
                    super::WorthServerQueryDependencySupportPinProvenanceParts {
                        workspace_name: "unbound-workspace".to_string(),
                        required_families: path.required_query_families.to_vec(),
                        support_matrix_digest: reason.clone(),
                        support_snapshot_digest: reason.clone(),
                        contract_digest: reason.clone(),
                        report_digest: reason.clone(),
                        blocking_finding_count: usize::MAX,
                        matched_required_count: 0,
                    },
                ),
            ),
            reason,
        ),
    }
}

fn evaluate_query_bound_support(
    request_contexts: &WorthServerRequestContextFacade,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    path: &WorthServerQueryDependencyCoveredPath,
) -> Result<WorthServerQueryDependencySupportPinProvenance, String> {
    let request_input = WorthServerRequestContextInput::builder()
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
        WorthServerQueryWorkspaceBindingRequest::for_query_handoff(resolved, path.operation());
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

    Ok(WorthServerQueryDependencySupportPinProvenance::new(
        super::WorthServerQueryDependencySupportPinProvenanceParts {
            workspace_name: workspace.name().to_string(),
            required_families: path.required_query_families.to_vec(),
            support_matrix_digest: snapshot.source_matrix_digest().to_string(),
            support_snapshot_digest: snapshot.snapshot_digest().to_string(),
            contract_digest: report.contract_digest().to_string(),
            report_digest: report.report_digest().to_string(),
            blocking_finding_count: report.blocking_finding_count(),
            matched_required_count: report.matched_required_count(),
        },
    ))
}

fn binding_error_reason(error: WorthServerQueryWorkspaceBindingError) -> String {
    format!(
        "workspace-binding-failed:{}:{}",
        error.stage(),
        error.message()
    )
}

fn scope_posture_for(
    path_kind: super::WorthServerQueryDependencyAuditPathKind,
) -> WorthServerQueryDependencyScopePosture {
    match path_kind {
        super::WorthServerQueryDependencyAuditPathKind::DirectDeclarationSupportPosture
        | super::WorthServerQueryDependencyAuditPathKind::ServerConsumerBoundaryAudit => {
            WorthServerQueryDependencyScopePosture::ConsumerKitScoped
        }
        _ => WorthServerQueryDependencyScopePosture::QueryFamilyScoped,
    }
}

fn require_supported_family(
    builder: WorthQuerySupportPinContractBuilder,
    family: WorthQueryRuntimeFacadeFamily,
) -> Result<WorthQuerySupportPinContractBuilder, WorthQuerySupportPinningError> {
    builder.require_family(family, |row| {
        row.status(WorthQueryPinnedSupportStatus::Supported)
            .teaching_posture(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
            .bind_live_row_digest()
    })
}
