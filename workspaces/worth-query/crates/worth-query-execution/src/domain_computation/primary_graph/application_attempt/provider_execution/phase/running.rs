use worth_query_admission::facade::basis::basis_lifecycle;
use worth_query_installation::facade::ApplicationSchema;
use worth_relational::facade::bridge::bridge_snapshot_identity_for_handle;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, BridgeTruthViewSelector,
    HistoricalEvaluationDeclaration, SnapshotReadPacket,
};

use super::super::super::{
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationCommitOutcome,
};
use super::super::support::denied;
use super::prepared::WorthQueryPreparedApplicationCommit;
use crate::domain_computation::operation_binding::WorthQueryApplicationOperationBindingInput;
use crate::domain_computation::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::{
    WorthQueryExecutionBoundOperationAuthority, WorthQueryManagedRunTerminalKind,
    WorthQueryManagedTruthReadRequest,
};

pub(in super::super) struct WorthQueryRunningApplicationCommit<Schema, Operation, Input, Scope> {
    pub(super) admission: WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    pub(super) lease: super::super::super::snapshot_lease::WorthQueryApplicationSnapshotLease,
    pub(super) provider_attempt:
        super::super::super::provider_binding::WorthQueryPreparedApplicationProviderAttempt,
    pub(super) authorization:
        crate::domain_computation::authorization::WorthQueryProviderAuthorizationDecisionFacts,
    pub(super) commit_authorization:
        crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    pub(super) idempotency: super::super::super::WorthQueryApplicationIdempotencyBinding,
    pub(super) running: crate::domain_computation::WorthQueryRunningDirectRun,
    pub(super) mutation_run:
        crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
    pub(super) aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

pub(in super::super) fn start_managed_application_commit<Schema, Operation, Input, Scope>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    prepared: WorthQueryPreparedApplicationCommit<Schema, Operation, Input, Scope>,
) -> Result<
    WorthQueryRunningApplicationCommit<Schema, Operation, Input, Scope>,
    WorthQueryApplicationCommitOutcome,
>
where
    Schema: ApplicationSchema,
{
    let WorthQueryPreparedApplicationCommit {
        mut admission,
        lease,
        provider_attempt,
        authorization,
        commit_authorization,
        idempotency,
        aftermath_causality,
    } = prepared;
    let snapshot = lease.snapshot();
    let operation = bind_execution_operation(application, &admission, snapshot)?;
    let reserved = admission
        .graph_work_mut()
        .take_operation_capacity()
        .ok_or_else(|| denied(DenialStage::ResourceAdmission))?;
    let attempt = application
        .runtime
        .start_reserved_direct_resource_attempt(&operation, reserved)
        .map_err(|_| denied(DenialStage::ResourceAdmission))?;
    let read_request = WorthQueryManagedTruthReadRequest::new(
        snapshot.version_id,
        admission.graph_work().branch().truth().clone(),
        SnapshotReadPacket::new(Vec::new()),
    );
    let request_bridge = application.bridge.fork_managed_request_lane();
    let running = application
        .runtime
        .managed_run_admission(&request_bridge, &application.relational_source)
        .admit_direct(&operation, attempt, read_request)
        .map_err(|_| denied(DenialStage::ManagedRunAdmission))?
        .start();
    let Some(mutation_run) = admission.graph_work().bind_mutation_run(&running) else {
        let _ = running
            .terminate_for_convergence(WorthQueryManagedRunTerminalKind::Failed)
            .cleanup();
        return Err(denied(DenialStage::ManagedRunAdmission));
    };
    Ok(WorthQueryRunningApplicationCommit {
        admission,
        lease,
        provider_attempt,
        authorization,
        commit_authorization,
        idempotency,
        running,
        mutation_run,
        aftermath_causality,
    })
}

fn bind_execution_operation<Schema, Operation, Input, Scope>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
) -> Result<WorthQueryExecutionBoundOperationAuthority, WorthQueryApplicationCommitOutcome>
where
    Schema: ApplicationSchema,
{
    application
        .primary_provider
        .observe_managed_application_bridge_plan();
    let branch = admission.graph_work().branch().truth().clone();
    let bridge_snapshot = bridge_snapshot_identity_for_handle(snapshot);
    application
        .bridge
        .plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                BridgeTruthViewSelector::branch_snapshot(branch, bridge_snapshot),
                BridgeReplayMode::Disabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            SnapshotReadPacket::new(Vec::new()),
        )
        .map_err(|_| denied(DenialStage::BridgePlanning))?;
    let basis = basis_lifecycle()
        .branch_snapshot(
            admission.graph_work_branch().0.clone(),
            format!(
                "relational-snapshot:{}:{}",
                snapshot.snapshot_id.0, snapshot.version_id.0
            ),
        )
        .for_mutation_preparation()
        .map_err(|_| denied(DenialStage::BasisAdmission))?
        .admit()
        .map_err(|_| denied(DenialStage::BasisAdmission))?;
    Ok(
        WorthQueryExecutionBoundOperationAuthority::bind_application(
            WorthQueryApplicationOperationBindingInput {
                runtime: &application.runtime,
                owner: application.installed_schema.owner(),
                installed_operation_fingerprint: admission.retain_installed_operation_fingerprint(),
                resource_binding_identity: admission.retain_resource_binding_identity(),
                basis: &basis,
                contracts: admission.allowed_graph_contract(),
                graph: &application.primary_graph_authority,
                support: application.primary_provider.application_resource_support(),
                graph_work_session: admission.graph_work_session_identity(),
                graph_work_managed_run: admission.graph_work_managed_run_identity(),
            },
        ),
    )
}
