use forge_query::facade::ForgeQueryWorkspace;
use forge_server::{
    ForgeServerDirectDeclarationSourceKind, ForgeServerQueryHandoffOperation,
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceBindingTarget,
};

pub(super) fn install_requested_named_read(
    workspace: &mut ForgeQueryWorkspace,
    request: &ForgeServerQueryWorkspaceBindingRequest,
) -> Result<(), ForgeServerQueryWorkspaceBindingError> {
    let Some(binding_label) = binding_label(request.target()) else {
        return Ok(());
    };

    if binding_label.ends_with(".missing") {
        return Ok(());
    }

    workspace
        .live_view::<serde_json::Value>(binding_label, |q| {
            q.from("User")
                .select(["identity.id", "profile.display_name"])
                .schema_basis("forge-server-test-named-read")
        })
        .map(|_| ())
        .map_err(|error| {
            ForgeServerQueryWorkspaceBindingError::new(
                "workspace_declaration",
                format!("{error:?}"),
            )
        })
}

fn binding_label(target: &ForgeServerQueryWorkspaceBindingTarget) -> Option<&str> {
    match target {
        ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration {
            source_kind: ForgeServerDirectDeclarationSourceKind::NamedRead,
            binding_label,
        } => Some(binding_label),
        ForgeServerQueryWorkspaceBindingTarget::QueryHandoff { operation } => {
            query_handoff_binding_label(operation)
        }
        ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration { .. } => None,
    }
}

fn query_handoff_binding_label(operation: &ForgeServerQueryHandoffOperation) -> Option<&str> {
    match operation {
        ForgeServerQueryHandoffOperation::QueryRead { operation_name }
        | ForgeServerQueryHandoffOperation::DirectRead { operation_name } => Some(operation_name),
        ForgeServerQueryHandoffOperation::DirectState { target_label }
        | ForgeServerQueryHandoffOperation::DirectInspection { target_label }
        | ForgeServerQueryHandoffOperation::DirectProjection { target_label } => Some(target_label),
        ForgeServerQueryHandoffOperation::DirectMutation { .. }
        | ForgeServerQueryHandoffOperation::QueryMutation { .. }
        | ForgeServerQueryHandoffOperation::DownstreamDelivery { .. } => None,
    }
}
