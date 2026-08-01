use worth_query::facade::{foundation, runtime};

pub(super) struct WorthUiScalarProjectionMutationAuthority;

impl runtime::WorthQueryRuntimeWriteAuthorityAdapter for WorthUiScalarProjectionMutationAuthority {
    fn write(
        &mut self,
        _bridge: &worth_runtime_bridge::facade::RuntimeBridge,
        _relational_runtime: Option<&mut worth_relational::facade::runtime::RelationalRuntime>,
        _mutation: runtime::WorthQueryBackendAdmissibleMutation,
    ) -> Result<runtime::WriteAuthorityExecutionReceipt, foundation::WorthQueryWorkspaceError> {
        Err(foundation::WorthQueryWorkspaceError::new(
            "Worth UI product projection does not admit general writes; use its typed intent authority",
        ))
    }
}
