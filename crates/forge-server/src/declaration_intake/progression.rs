use forge_query::facade::consumer_kit::{
    project_workspace_support_snapshot, support_pinning_contract, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture,
};
use forge_query::facade::{ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace};

use crate::{
    config::ForgeServerQueryHandoffConfig, ForgeServerAdmission,
    ForgeServerQueryWorkspaceBindingRequest,
};

use super::{
    ForgeServerDirectDeclaration, ForgeServerDirectDeclarationDenial,
    ForgeServerDirectSupportSnapshot, ForgeServerPreparedDirectDeclaration,
};

pub(crate) fn prepare_direct_declaration(
    config: &ForgeServerQueryHandoffConfig,
    admission: ForgeServerAdmission,
    declaration: ForgeServerDirectDeclaration,
) -> Result<ForgeServerPreparedDirectDeclaration, ForgeServerDirectDeclarationDenial> {
    validate_declaration_identity(&admission, &declaration)?;
    let workspace = bind_workspace(config, &admission, &declaration)?;
    let support_snapshot = build_support_snapshot(
        admission.request_context().diagnostics_profile(),
        &workspace,
        declaration.clone(),
    )?;
    let declaration_digest = declaration_digest(&workspace, &declaration);

    Ok(ForgeServerPreparedDirectDeclaration::new(
        admission,
        declaration,
        workspace,
        declaration_digest,
        support_snapshot,
    ))
}

fn validate_declaration_identity(
    admission: &ForgeServerAdmission,
    declaration: &ForgeServerDirectDeclaration,
) -> Result<(), ForgeServerDirectDeclarationDenial> {
    if declaration.source().has_blank_binding_label() {
        return Err(
            ForgeServerDirectDeclarationDenial::invalid_declaration_identity(
                admission.request_context().diagnostics_profile(),
                format!(
                    "{} declaration identity cannot be blank",
                    declaration.source().kind().as_str()
                ),
            ),
        );
    }

    Ok(())
}

fn bind_workspace(
    config: &ForgeServerQueryHandoffConfig,
    admission: &ForgeServerAdmission,
    declaration: &ForgeServerDirectDeclaration,
) -> Result<ForgeQueryWorkspace, ForgeServerDirectDeclarationDenial> {
    let binding_request = ForgeServerQueryWorkspaceBindingRequest::for_direct_declaration(
        admission.resolved_request_context().clone(),
        declaration.source().kind(),
        declaration.source().binding_label(),
    );
    config
        .workspace_provider()
        .bind_workspace(&binding_request)
        .map_err(|error| {
            ForgeServerDirectDeclarationDenial::workspace_binding_failed(
                admission.request_context().diagnostics_profile(),
                format!("{}: {}", error.stage(), error.message()),
            )
        })
}

fn build_support_snapshot(
    diagnostics_profile: forge_foundational::DiagnosticRichnessProfile,
    workspace: &ForgeQueryWorkspace,
    declaration: ForgeServerDirectDeclaration,
) -> Result<ForgeServerDirectSupportSnapshot, ForgeServerDirectDeclarationDenial> {
    let support_snapshot = project_workspace_support_snapshot(workspace);
    let read_family_row = support_snapshot
        .rows()
        .iter()
        .find(|row| row.facade_family() == Some(ForgeQueryRuntimeFacadeFamily::Read.as_str()))
        .cloned()
        .ok_or_else(|| {
            ForgeServerDirectDeclarationDenial::query_facade_family_not_admitted(
                diagnostics_profile,
                "support snapshot did not expose the read facade family row",
                ForgeServerDirectSupportSnapshot::new(
                    declaration.clone(),
                    support_snapshot.rows().first().cloned().expect(
                        "support snapshot should expose at least one row when building direct support posture",
                    ),
                    None,
                    support_snapshot.source_matrix_digest().to_string(),
                    support_snapshot.snapshot_digest().to_string(),
                    "direct-declaration-read-row-missing".to_string(),
                    false,
                ),
            )
        })?;
    let read_family_contract = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Read)
        .ok();
    let pin_report = support_pinning_contract("forge-server.direct-declaration-intake")
        .against_snapshot(&support_snapshot)
        .and_then(|builder| {
            builder.require_family(ForgeQueryRuntimeFacadeFamily::Read, |row| {
                row.status(ForgeQueryPinnedSupportStatus::Supported)
                    .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                    .bind_live_row_digest()
            })
        })
        .and_then(|builder| builder.seal())
        .and_then(|contract| contract.evaluate_snapshot(&support_snapshot))
        .map_err(|error| {
            ForgeServerDirectDeclarationDenial::query_facade_family_not_admitted(
                diagnostics_profile,
                format!("direct declaration support pinning failed: {error}"),
                ForgeServerDirectSupportSnapshot::new(
                    declaration.clone(),
                    read_family_row.clone(),
                    read_family_contract.clone(),
                    support_snapshot.source_matrix_digest().to_string(),
                    support_snapshot.snapshot_digest().to_string(),
                    "direct-declaration-support-pin-evaluation-failed".to_string(),
                    false,
                ),
            )
        })?;

    Ok(ForgeServerDirectSupportSnapshot::new(
        declaration,
        read_family_row,
        read_family_contract,
        support_snapshot.source_matrix_digest().to_string(),
        support_snapshot.snapshot_digest().to_string(),
        pin_report.report_digest().to_string(),
        pin_report.satisfied(),
    ))
}

fn declaration_digest(
    workspace: &ForgeQueryWorkspace,
    declaration: &ForgeServerDirectDeclaration,
) -> String {
    format!(
        "forge-server-direct-declaration-v1|workspace:{}|source:{}|view-shape:{}",
        workspace.name(),
        declaration.source().canonical_label(),
        declaration.view_shape().as_str(),
    )
}
