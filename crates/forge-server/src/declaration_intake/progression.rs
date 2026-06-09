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
    let support_snapshot = build_support_snapshot(&workspace, declaration.clone());
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
    workspace: &ForgeQueryWorkspace,
    declaration: ForgeServerDirectDeclaration,
) -> ForgeServerDirectSupportSnapshot {
    let support_matrix = workspace.public_support_matrix();
    let read_family_row = support_matrix
        .row_for_family(ForgeQueryRuntimeFacadeFamily::Read)
        .cloned()
        .expect("query runtime public support matrix must expose a read family row");
    let read_family_contract = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Read)
        .ok();
    ForgeServerDirectSupportSnapshot::new(
        declaration,
        read_family_row,
        read_family_contract,
        support_matrix.matrix_digest().to_string(),
    )
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
