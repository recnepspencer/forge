use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkIntent;
use worth_query_admission::integration::{
    admit_application_operation_graph_work, require_selected_graph_work,
    select_installed_graph_obligations,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};
use worth_relational::facade::identity::EntityId;

use crate::domain_computation::operation_binding::{
    WorthQueryApplicationOperationBindingInput, WorthQueryExecutionBoundOperationAuthority,
};
use crate::domain_computation::primary_graph::{
    application_resource_request, WorthQueryApplicationSnapshotLease,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::{
    start_mutation_graph_work_session, WorthQueryGraphWorkAccessContextAffinity,
    WorthQueryGraphWorkBasisAffinity, WorthQueryGraphWorkSessionAffinity,
    WorthQueryManagedGraphWorkSession, WorthQueryMutationGraphWorkLane,
};

use super::{WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind};

pub(in crate::domain_computation) type WorthQueryOperationGraphWorkSession =
    WorthQueryManagedGraphWorkSession<
        WorthQueryMutationGraphWorkLane,
        WorthQueryApplicationSnapshotLease,
    >;

pub(super) fn start_operation_graph_work<Schema, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    resource_binding_identity: &str,
    principal: EntityId,
    access_context: WorthQueryGraphWorkAccessContextAffinity,
) -> Result<WorthQueryOperationGraphWorkSession, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            operation.operation(),
        )
    })?;
    let branch = runtime.branch_affinity().clone();
    let lease = WorthQueryApplicationSnapshotLease::acquire(
        graph.integration_handle(),
        graph.retain_layout(),
        branch.relational_branch(),
    )
    .map_err(|denial| {
        graph_work_denial(format!("{}: {}", operation.operation(), denial.detail()))
    })?;
    let operation_authority = bind_operation_execution_authority(
        runtime,
        operation,
        resource_binding_identity,
        &lease,
        &branch,
    )?;
    let selected = select_installed_graph_obligations(
        operation.contracts().obligations(),
        WorthQueryGraphWorkIntent::application_operation_mutation(),
    )
    .map_err(|_| graph_work_denial(operation.operation()))?;
    let required = require_selected_graph_work(selected, runtime.graph_admission_authority())
        .map_err(|_| graph_work_denial(operation.operation()))?;
    let request = application_resource_request(operation.contracts())
        .ok_or_else(|| graph_work_denial(operation.operation()))?;
    let plan = admit_application_operation_graph_work(
        required,
        resource_binding_identity,
        &request,
        runtime.graph_work_resource_support(),
    )
    .map_err(|_| graph_work_denial(operation.operation()))?;
    let basis = WorthQueryGraphWorkBasisAffinity::mutation(lease.snapshot(), &branch)
        .map_err(|_| graph_work_denial(operation.operation()))?;
    let affinity = WorthQueryGraphWorkSessionAffinity::new(
        &plan,
        runtime.runtime.authority_identity(),
        operation.contracts().obligations().identity(),
        operation.authority_identity(),
        principal,
        access_context,
        branch,
        basis,
        runtime.graph_work_provider_authority(),
    )
    .map_err(|_| graph_work_denial(operation.operation()))?;
    start_mutation_graph_work_session(
        plan,
        lease,
        affinity,
        &runtime.runtime,
        &operation_authority,
    )
    .map_err(|_| graph_work_denial(operation.operation()))
}

pub(in crate::domain_computation) fn bind_operation_execution_authority<Schema, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    resource_binding_identity: &str,
    lease: &WorthQueryApplicationSnapshotLease,
    branch: &crate::domain_computation::provider_session::WorthQueryGraphWorkBranchAffinity,
) -> Result<WorthQueryExecutionBoundOperationAuthority, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let snapshot = lease.snapshot();
    let basis_path = basis_lifecycle()
        .branch_snapshot(
            branch.relational_branch().0.clone(),
            format!(
                "relational-snapshot:{}:{}",
                snapshot.snapshot_id.0, snapshot.version_id.0
            ),
        )
        .for_mutation_preparation()
        .map_err(|_| graph_work_denial(operation.operation()))?;
    let basis = basis_path
        .admit()
        .map_err(|_| graph_work_denial(operation.operation()))?;
    Ok(
        WorthQueryExecutionBoundOperationAuthority::bind_application(
            WorthQueryApplicationOperationBindingInput {
                runtime: &runtime.runtime,
                owner: runtime.installed_schema.owner(),
                installed_operation_fingerprint: Arc::from(operation.authority_identity()),
                resource_binding_identity: Arc::from(resource_binding_identity),
                basis: &basis,
                contracts: operation.contracts(),
                graph: runtime.graph_work_provider_authority(),
                support: runtime.graph_work_resource_support(),
            },
        ),
    )
}

fn graph_work_denial(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::GraphWorkAdmissionUnavailable,
        subject,
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
use std::sync::Arc;

use worth_query_admission::facade::basis::basis_lifecycle;
