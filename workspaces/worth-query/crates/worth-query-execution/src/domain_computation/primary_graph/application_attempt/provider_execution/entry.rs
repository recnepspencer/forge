use worth_query_admission::facade::basis::basis_lifecycle;
use worth_query_installation::facade::ApplicationSchema;
use worth_relational::facade::bridge::bridge_snapshot_identity_for_handle;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, BridgeTruthViewSelector,
    HistoricalEvaluationDeclaration, SnapshotReadPacket,
};

use super::super::provider_binding::prepare_provider_attempt;
use super::super::{
    provider_recomparison::recover_equivalent_commit_evidence, WorthQueryApplicationCommitDenial,
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationEffectProgram,
    WorthQueryApplicationIdempotencyBinding,
};
use super::elevation_currentness::WorthQueryElevationCommitCurrentness;
use super::outcome::WorthQueryProviderProgressionOutcome;
use super::progression::{execute_provider_progression, WorthQueryProviderProgression};
use super::support::denied;
use crate::domain_computation::operation_binding::WorthQueryApplicationOperationBindingInput;
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::{
    WorthQueryExecutionBoundOperationAuthority, WorthQueryManagedRunTerminalKind,
    WorthQueryManagedTruthReadRequest,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn compare_and_commit_application<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> WorthQueryApplicationCommitOutcome {
        if program.read_set.admission.has_elevation_lifecycle_binding() {
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::elevation_transition_required(),
            );
        }
        if program
            .read_set
            .admission
            .allowed_graph_contract()
            .execution_posture()
            .requires_delegation_activation()
        {
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::delegation_activation_required(),
            );
        }
        if program
            .read_set
            .admission
            .allowed_graph_contract()
            .execution_posture()
            .requires_capability_revocation()
        {
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::capability_revocation_required(),
            );
        }
        self.compare_and_commit_application_inner(program, idempotency)
    }

    pub(super) fn compare_and_commit_application_inner<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> WorthQueryApplicationCommitOutcome {
        self.compare_and_commit_application_inner_with_currentness(program, idempotency, None)
    }

    pub(super) fn compare_and_commit_application_inner_with_currentness<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        elevation_currentness: Option<WorthQueryElevationCommitCurrentness>,
    ) -> WorthQueryApplicationCommitOutcome {
        let WorthQueryApplicationEffectProgram {
            read_set,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        } = program;
        let mut admission = read_set.admission;
        let idempotency = idempotency
            .bind_operation(admission.operation_authority_identity_bytes())
            .bind_operation_scope(admission.operation_scope_binding())
            .bind_preconditions(admission.mutation_preconditions().identity())
            .bind_governed_input(admission.governed_input_identity())
            .bind_governed_proposal(admission.governed_proposal_identity());
        if admission.validate_current_authority().is_err() {
            return WorthQueryApplicationCommitOutcome::Cancelled;
        }
        if let Some(outcome) = self.resolve_retained_idempotency(&mut admission, idempotency) {
            return outcome;
        }
        if elevation_currentness
            .as_ref()
            .is_some_and(|currentness| !currentness.remains_current(&self.authorization_clock))
        {
            return denied(DenialStage::DecisionReadSet);
        }
        let (authorization, commit_authorization) =
            match admission.take_authorization_dependencies(self.authorization.bridge()) {
                Ok(authorization) => authorization,
                Err(_) => return denied(DenialStage::DecisionReadSet),
            };
        let prepared = match prepare_provider_attempt(
            read_set.facts,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        ) {
            Ok(prepared) => prepared,
            Err(_) => return denied(DenialStage::ProposalBinding),
        };
        let snapshot = read_set.lease.snapshot();
        let branch = admission.graph_work().branch().truth().clone();
        let operation = match self.bind_execution_operation(&admission, snapshot) {
            Ok(operation) => operation,
            Err(outcome) => return outcome,
        };
        let reserved = match admission.graph_work_mut().take_operation_capacity() {
            Some(reserved) => reserved,
            None => return denied(DenialStage::ResourceAdmission),
        };
        let attempt = match self
            .runtime
            .start_reserved_direct_resource_attempt(&operation, reserved)
        {
            Ok(attempt) => attempt,
            Err(_) => return denied(DenialStage::ResourceAdmission),
        };
        let read_request = WorthQueryManagedTruthReadRequest::new(
            snapshot.version_id,
            branch,
            SnapshotReadPacket::new(Vec::new()),
        );
        let request_bridge = self.bridge.fork_managed_request_lane();
        let mut running = match self
            .runtime
            .managed_run_admission(&request_bridge, &self.relational_source)
            .admit_direct(&operation, attempt, read_request)
        {
            Ok(admitted) => admitted.start(),
            Err(_) => return denied(DenialStage::ManagedRunAdmission),
        };
        let mutation_run = match admission.graph_work().bind_mutation_run(&running) {
            Some(binding) => binding,
            None => {
                let _ = running
                    .terminate_for_convergence(WorthQueryManagedRunTerminalKind::Failed)
                    .cleanup();
                return denied(DenialStage::ManagedRunAdmission);
            }
        };
        let serialization = self.primary_provider.serialize_application_commit();
        let outcome = execute_provider_progression(WorthQueryProviderProgression {
            application: self,
            running: &mut running,
            graph: &self.primary_graph_authority,
            provider: &self.primary_provider,
            admission: &admission,
            prepared,
            authorization,
            commit_authorization,
            idempotency,
            mutation_run: &mutation_run,
            serialization: &serialization,
        });
        let terminal = match &outcome {
            WorthQueryProviderProgressionOutcome::Committed(_)
            | WorthQueryProviderProgressionOutcome::AlreadyCommitted(_)
            | WorthQueryProviderProgressionOutcome::Stale(_) => {
                WorthQueryManagedRunTerminalKind::Completed
            }
            WorthQueryProviderProgressionOutcome::Cancelled => {
                WorthQueryManagedRunTerminalKind::Cancelled
            }
            WorthQueryProviderProgressionOutcome::Denied(_)
            | WorthQueryProviderProgressionOutcome::Aborted
            | WorthQueryProviderProgressionOutcome::Indeterminate => {
                WorthQueryManagedRunTerminalKind::Failed
            }
        };
        let snapshot_released = read_set.lease.release();
        let completion = match mutation_run.finish(running, terminal, snapshot_released) {
            Ok(completion) => completion,
            Err(()) => return WorthQueryApplicationCommitOutcome::Indeterminate,
        };
        outcome
            .finish(completion)
            .unwrap_or(WorthQueryApplicationCommitOutcome::Indeterminate)
    }

    fn resolve_retained_idempotency<Operation, Input, Scope>(
        &self,
        admission: &mut crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> Option<WorthQueryApplicationCommitOutcome> {
        let serialization = self.primary_provider.serialize_application_commit();
        let branch = admission.graph_work().branch().relational().clone();
        let proof = match self.authorize_retained_idempotency(admission, &serialization) {
            Ok(proof) => proof,
            Err(_) => return Some(denied(DenialStage::DecisionReadSet)),
        };
        match proof.govern((), |()| {
            self.primary_provider
                .resolve_idempotency_binding(idempotency, &branch)
        }) {
            Err(()) => Some(denied(DenialStage::DecisionReadSet)),
            Ok(Ok(WorthQueryProviderIdempotencyResolution::Absent)) => None,
            Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
                Some(WorthQueryApplicationCommitOutcome::AlreadyCommitted(
                    WorthQueryApplicationCommitReceipt::from_recovered_provider(
                        receipt,
                        recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                        admission.canonical_work(),
                    ),
                ))
            }
            Ok(Ok(WorthQueryProviderIdempotencyResolution::Drift)) => {
                Some(WorthQueryApplicationCommitOutcome::Denied(
                    WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                ))
            }
            Ok(Err(_)) => Some(denied(DenialStage::Idempotency)),
        }
    }

    fn bind_execution_operation<Operation, Input, Scope>(
        &self,
        admission: &crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> Result<WorthQueryExecutionBoundOperationAuthority, WorthQueryApplicationCommitOutcome>
    {
        let branch = admission.graph_work().branch().truth().clone();
        let bridge_snapshot = bridge_snapshot_identity_for_handle(snapshot);
        self.bridge
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
        let basis_path = basis_lifecycle()
            .branch_snapshot(
                admission.graph_work_branch().0.clone(),
                format!(
                    "relational-snapshot:{}:{}",
                    snapshot.snapshot_id.0, snapshot.version_id.0
                ),
            )
            .for_mutation_preparation()
            .map_err(|_| denied(DenialStage::BasisAdmission))?;
        let basis = basis_path
            .admit()
            .map_err(|_| denied(DenialStage::BasisAdmission))?;
        Ok(
            WorthQueryExecutionBoundOperationAuthority::bind_application(
                WorthQueryApplicationOperationBindingInput {
                    runtime: &self.runtime,
                    owner: self.installed_schema.owner(),
                    installed_operation_fingerprint: admission
                        .retain_installed_operation_fingerprint(),
                    resource_binding_identity: admission.retain_resource_binding_identity(),
                    basis: &basis,
                    contracts: admission.allowed_graph_contract(),
                    graph: &self.primary_graph_authority,
                    support: self.primary_provider.application_resource_support(),
                    graph_work_session: admission.graph_work_session_identity(),
                    graph_work_managed_run: admission.graph_work_managed_run_identity(),
                },
            ),
        )
    }
}
