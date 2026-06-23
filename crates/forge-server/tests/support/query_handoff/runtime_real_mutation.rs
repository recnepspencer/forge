use forge_query::facade::consumer_kit::{in_memory_test_runtime, ForgeQueryTestBackendSchema};
use forge_query::facade::ForgeQueryWorkspace;
use forge_server::{
    ForgeServerQueryHandoffOperation, ForgeServerQueryWorkspaceBindingError,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerQueryWorkspaceBindingTarget,
    ForgeServerQueryWorkspaceProvider,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct RealMutationWorkspaceProvider;

impl ForgeServerQueryWorkspaceProvider for RealMutationWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "real-mutation-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError> {
        match request.target() {
            ForgeServerQueryWorkspaceBindingTarget::QueryHandoff {
                operation:
                    ForgeServerQueryHandoffOperation::DirectMutation { .. }
                    | ForgeServerQueryHandoffOperation::QueryMutation { .. },
            } => {}
            target => {
                return Err(ForgeServerQueryWorkspaceBindingError::new(
                    "workspace_target",
                    format!(
                        "real mutation workspace provider only supports mutation handoff targets, got {target:?}"
                    ),
                ));
            }
        }

        let workspace_id = request
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .workspace_id();
        in_memory_test_runtime()
            .with_schema(task_schema())
            .workspace(workspace_id)
            .map_err(|error| {
                ForgeServerQueryWorkspaceBindingError::new("workspace_bind", format!("{error:?}"))
            })
    }
}

fn task_schema() -> ForgeQueryTestBackendSchema {
    ForgeQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("task schema identity aspect should be valid")
        .aspect("title.value", "title.value")
        .expect("task schema title aspect should be valid")
}
