use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeSupportProfile,
};
use forge_server::{
    ForgeServerQueryHandoffOperation, ForgeServerQueryWorkspaceBindingError,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerQueryWorkspaceBindingTarget,
    ForgeServerQueryWorkspaceProvider,
};

#[path = "../query_handoff/runtime.rs"]
mod query_handoff_runtime;

#[derive(Clone, Debug, Default)]
pub(crate) struct SchedulerWorkspaceProvider;

impl ForgeServerQueryWorkspaceProvider for SchedulerWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "scheduler-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError>
    {
        validate_scheduler_world(request)?;
        query_handoff_runtime::TestWorkspaceProvider::default().bind_workspace(request)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SelectiveSharedReadWorkspaceProvider;

impl ForgeServerQueryWorkspaceProvider for SelectiveSharedReadWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "selective-shared-read-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError>
    {
        validate_scheduler_world(request)?;
        let workspace_id = workspace_id(request);
        if workspace_id == "workspace-shared-read-denied" {
            return query_handoff_runtime::ProfiledTestWorkspaceProvider::new(
                ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                    ForgeQueryRuntimeFamilySupport::unsupported(
                        ForgeQueryRuntimeFacadeFamily::SharedRead,
                        "shared-read is intentionally denied for scheduler hostility coverage",
                    ),
                ),
            )
            .bind_workspace(request);
        }
        query_handoff_runtime::TestWorkspaceProvider::default().bind_workspace(request)
    }
}

fn validate_scheduler_world(
    request: &ForgeServerQueryWorkspaceBindingRequest,
) -> Result<(), ForgeServerQueryWorkspaceBindingError> {
    let workspace_id = workspace_id(request);
    let Some(binding_label) = query_handoff_binding_label(request.target()) else {
        return Ok(());
    };

    let admitted = matches!(
        (workspace_id, binding_label),
        ("workspace-42", "users.profile")
            | ("workspace-43", "users.profile")
            | ("workspace-shared-read-denied", "users.profile")
            | ("workspace-missing", "users.profile.missing")
    );
    if admitted {
        Ok(())
    } else {
        Err(ForgeServerQueryWorkspaceBindingError::new(
            "scheduler_fixture_world",
            format!(
                "scheduler fixture world does not declare `{binding_label}` for workspace `{workspace_id}`"
            ),
        ))
    }
}

fn workspace_id(request: &ForgeServerQueryWorkspaceBindingRequest) -> &str {
    request
        .resolved_request_context()
        .request_context()
        .workspace_target()
        .workspace_id()
}

fn query_handoff_binding_label(target: &ForgeServerQueryWorkspaceBindingTarget) -> Option<&str> {
    let ForgeServerQueryWorkspaceBindingTarget::QueryHandoff { operation } = target else {
        return None;
    };
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
