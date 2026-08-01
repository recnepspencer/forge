//! Session-owned current Relational basis for privileged authorization checks.

use worth_query_installation::facade::ApplicationSchema;

use crate::domain_computation::authorization::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryOperationGraphWorkSession,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

pub(super) fn with_current_authorization_basis<Schema, Output>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    session: &WorthQueryOperationGraphWorkSession,
    observe: impl FnOnce(
        &mut worth_relational::facade::runtime::RelationalRuntime,
        &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> Result<Output, WorthQueryOperationAuthorizationDenial>,
) -> Result<Output, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let graph = application
        .runtime
        .primary_graph()
        .ok_or_else(foreign_runtime)?;
    let branch = session.branch_affinity().relational_branch().clone();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let version = handle
            .ensure_primary_indexes_current(runtime, &branch)
            .map_err(|_| unavailable_basis())?;
        let basis = runtime
            .snapshots()
            .admit_execution_basis(&branch, version)
            .map_err(|_| unavailable_basis())?;
        let result = observe(runtime, basis.snapshot_handle());
        if !basis.release().released() {
            return Err(inconsistent_basis());
        }
        result
    })
}

fn foreign_runtime() -> WorthQueryOperationAuthorizationDenial {
    denial(WorthQueryOperationAuthorizationDenialKind::ForeignRuntime)
}

fn unavailable_basis() -> WorthQueryOperationAuthorizationDenial {
    denial(WorthQueryOperationAuthorizationDenialKind::GraphWorkAdmissionUnavailable)
}

fn inconsistent_basis() -> WorthQueryOperationAuthorizationDenial {
    denial(WorthQueryOperationAuthorizationDenialKind::InconsistentDecision)
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, "application-authorization-basis")
}
